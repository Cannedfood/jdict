#include "./jmdict_index.hpp"

#include <server/util/timer.hpp>
#include <server/util/kana.hpp>
#include <server/util/utf8.hpp>

#include <algorithm>
#include <functional>
#include <set>
#include <cstdio>
#include <string_view>
#include <utility>

namespace jdict {

using rating_t = jmdict_index::rating_t;
using result_t = jmdict_index::result_t;
using results_t = jmdict_index::results_t;
using entry_ptr = jmdict_index::entry_ptr;

static inline bool contains(std::vector<std::string> const& v, std::string_view s) {
	for(auto& ss : v) {
		if(ss == s) return true;
	}
	return false;
}

static inline std::string_view remove_braces(std::string_view s) {
	s = s.substr(0, s.find('('));
	while(s.size() && s.back() <= ' ') s.remove_suffix(1);
	return s;
}

namespace ratings {

constexpr inline rating_t match_exact         = 400000;
constexpr inline rating_t match_starts_with   = 300000;
constexpr inline rating_t match_ends_with     = 200000;
constexpr inline rating_t match_contains      = 100000;
constexpr inline rating_t match_fuzz_penalty  =   1000;

constexpr inline rating_t has_news1 = 2000;
constexpr inline rating_t has_news2 = 1000;
constexpr inline rating_t has_ichi1 = 2000;
constexpr inline rating_t has_ichi2 = 1000;
constexpr inline rating_t has_spec1 = 2000;
constexpr inline rating_t has_spec2 = 1000;
constexpr inline rating_t has_gai1  = 2000;
constexpr inline rating_t has_gai2  = 1000;
static_assert(has_news1 + has_news2 + has_ichi1 + has_ichi2 + has_spec1 + has_spec2 + has_gai1 + has_gai2 < match_contains);

constexpr inline rating_t word_length = 2000;
constexpr inline size_t   max_word_length = 100;

constexpr inline rating_t kanji           = 30;
constexpr inline rating_t reading_kana    = 20;
constexpr inline rating_t reading_romaji  = 20;
constexpr inline rating_t meaning         = 1;
constexpr inline rating_t sequence_number = 1000000;

constexpr inline rating_t position_penalty_kanji   = 400;
constexpr inline rating_t position_penalty_reading = 400;
constexpr inline rating_t position_penalty_sense   = 200;
constexpr inline rating_t position_penalty_gloss   = 400;

constexpr inline rating_t highlighted_gloss = 10;


static inline rating_t rate_match(std::string_view query, std::string_view match) {
	constexpr auto rate_match_exact = [&](std::string_view query, std::string_view match) {
		match = remove_braces(match);

		if(query == match) return ratings::match_exact;
		if(match.starts_with(query)) return ratings::match_starts_with;
		if(match.ends_with(query)) return ratings::match_ends_with;
		return ratings::match_contains;
	};
	constexpr auto rate_match_fuzzy = [=](std::string_view query, std::string_view match) {
		if(match.starts_with("to ")) match.remove_prefix(3);
		if(query.starts_with("to ")) query.remove_prefix(3);
		return rate_match_exact(query, match);
	};

	return std::max(
		rate_match_exact(query, match),
		rate_match_fuzzy(query, match) - match_fuzz_penalty
	);
}

static inline rating_t rate_word_length(std::string_view query, std::string_view match) {
	match = remove_braces(match);

	if(match.starts_with("to ") && !query.starts_with("to ")) match.remove_prefix(3);

	float n = ((int)utf8::count_codepoints(match) - (int) utf8::count_codepoints(query)) / float(max_word_length);
	return (rating_t) (ratings::word_length * std::max(0.f, 1 - n));
}

static inline rating_t priority(std::vector<std::string> const& v) {
	return
		has_news1 * contains(v, "news1") +
		has_news2 * contains(v, "news2") +
		has_ichi1 * contains(v, "ichi1") +
		has_ichi2 * contains(v, "ichi2") +
		has_spec1 * contains(v, "spec1") +
		has_spec2 * contains(v, "spec2") +
		has_gai1 * contains(v, "gai1") +
		has_gai2 * contains(v, "gai2");
}

} // namespace ratings

results_t jmdict_index::search(std::string_view query) const {
	// Find and rate results
	result_ratings_t ratings;

	find_by_sequence_number(ratings, query);
	find_general(ratings, query);
	// find_by_translation(ratings, 1, query);
	// find_by_reading(ratings, 1, query);

	return sort_results(std::move(ratings));
}

void jmdict_index::find_general(result_ratings_t& ratings, std::string_view query) const {
	idx_general.find(query, [&](std::string_view match, jmdict::entry_t const* entry, rating_t matchBaseRating) {
		rating_t rating =
			matchBaseRating +
			ratings::rate_match(query, match) +
			ratings::rate_word_length(query, match);

		auto& hitRating = ratings[entry];
		hitRating = std::max(hitRating, rating);
	});
}
void jmdict_index::find_by_sequence_number(result_ratings_t& ratings, std::string_view query) const {
	if(auto bySeq = idx_sequence_number.find(query); bySeq != idx_sequence_number.end()) {
		auto& hitRating = ratings[bySeq->second];
		hitRating = std::max(hitRating, ratings::sequence_number);
	}
}

results_t jmdict_index::sort_results(result_ratings_t&& ratings) {
	auto pairs = results_t(ratings.begin(), ratings.end());

	// Sort results by rating
	ratings.clear();
	std::sort(pairs.begin(), pairs.end(), [](auto const& a, auto const& b) {
		return a.second > b.second;
	});

	return pairs;
}

void jmdict_index::build_indices() {
	{
		debug::timer _("inserting into index");

		for(auto& entry : dict->entries) {
			rating_t max_priority_rating = 0;
			auto prio_rating = [&](std::vector<std::string> const& priorities) {
				auto result = ratings::priority(priorities);
				max_priority_rating = std::max(max_priority_rating, result);
				return result;
			};

			idx_sequence_number.emplace(entry.sequence, &entry);
			for(size_t k_idx = 0; k_idx < entry.kanji.size(); k_idx++) {
				auto& k = entry.kanji[k_idx];
				idx_general.insert(k.value, &entry,
					ratings::kanji
					+ prio_rating(k.priorities)
					- k_idx*k_idx * ratings::position_penalty_kanji
				);
			}
			for(size_t r_idx = 0; r_idx < entry.readings.size(); r_idx++) {
				auto& r = entry.readings[r_idx];
				auto rating = prio_rating(r.priorities) - r_idx*r_idx * ratings::position_penalty_reading;
				idx_general.insert(r.value, &entry, ratings::reading_kana + rating);
				if(!r.romaji.empty()) {
					idx_general.insert(r.romaji, &entry, ratings::reading_romaji + rating);
				}
			}

			for(size_t s_idx = 0; s_idx < entry.senses.size(); s_idx++) {
				auto& s = entry.senses[s_idx];
				for(size_t g_idx = 0; g_idx < s.glosses.size(); g_idx++) {
					auto& g = s.glosses[g_idx];
					idx_general.insert(remove_braces(g.content), &entry,
						ratings::meaning
						+ max_priority_rating
						+ g.highlight * ratings::highlighted_gloss
						- s_idx*s_idx * ratings::position_penalty_sense
						- g_idx*g_idx * ratings::position_penalty_gloss
					);
				}
			}
		}
	}
	{
		debug::timer _("building index");
		printf("Removed %zu duplicate entries\n", idx_general.remove_duplicates());
		idx_general.build();
	}
}

jmdict_index::jmdict_index(jmdict const& dict) :
	dict(&dict)
{
	build_indices();

	debug::timer _("writing stats");
	unsigned min = 40000000, totalEntries = 0, max = 0;
	unsigned count = 0;
	for(auto& e : idx_general.entries) {
		count++;
		totalEntries += e.second.size();
		min = std::min<unsigned>(e.second.size(), min);
		max = std::max<unsigned>(e.second.size(), max);
	}
	printf(
		"Got %u index entries in %u sets. Entries per set: min: %u, max: %u, avg: %u\n",
		totalEntries, count, min, max, totalEntries / count
	);
	idx_general.write_stats("./tmp.txt");
}

} // namespace jdict
