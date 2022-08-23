#include "./jmdict_index.hpp"

#include "./timer.hpp"
#include "./kana_util.hpp"
#include "jmdict.hpp"
#include "text_index.hpp"

#include <algorithm>
#include <functional>
#include <set>
#include <cstdio>
#include <utility>

namespace jdict {

std::vector<jmdict::entry const*> jmdict_index::search(std::string_view query) const {
	// Find and rate results
	std::map<jmdict::entry const*, unsigned> weights;

	find_by_sequence_number(weights, 1, query);
	find_general(weights, 1, query);
	// find_by_translation(weights, 1, query);
	// find_by_reading(weights, 1, query);

	return sort_results(std::move(weights));
}

void jmdict_index::find_general(ResultWeights& weights, int baseWeight, std::string_view query) const {
	auto readingQuery = to_romaji(query);
	idx_general.find(query, [&](std::tuple<std::string_view, jmdict::entry const*> const& e) {
		auto& [text, entry] = e;
		if(text.find(readingQuery) != std::string::npos) {
			auto& hitRating = weights[entry];
			hitRating = std::max(hitRating, rate_match(readingQuery, text) * baseWeight);
		}
	});
}
void jmdict_index::find_by_sequence_number(ResultWeights& weights, int baseWeight, std::string_view query) const {
	if(auto bySeq = idx_sequence_number.find(query); bySeq != idx_sequence_number.end()) {
		weights[bySeq->second] = baseWeight * 1;
	}
}

unsigned jmdict_index::rate_match(std::string_view query, std::string_view match) {
	unsigned rating = 1;
	if(match == query) rating += 1000; // Exact match
	else if(match.starts_with(query)) rating += 10; // Starts with search
	// TODO: occurence rating
	rating += 10 * std::max(0.f, 1 - ((int) match.size() - (int) query.size()) / 50.f); // Smaller is better
	return rating;
}

std::vector<jmdict::entry const*> jmdict_index::sort_results(ResultWeights&& weights) {
	auto pairs = std::vector<std::pair<jmdict::entry const*, unsigned>>(weights.begin(), weights.end());

	// Sort results by rating
	weights.clear();
	std::sort(pairs.begin(), pairs.end(), [](auto const& a, auto const& b) {
		return a.second > b.second;
	});

	// Compact results
	std::vector<jmdict::entry const*> sorted_entries;
	sorted_entries.reserve(pairs.size());
	for(auto [e, rating] : pairs) {
		sorted_entries.push_back(e);
	}
	return sorted_entries;
}

jmdict_index::jmdict_index(jmdict const& dict) :
	dict(&dict)
{
	debug::timer _("building index");

	for(auto& entry : dict.entries) {
		idx_sequence_number.emplace(entry.sequence, &entry);
		for(auto& r : entry.readings) {
			if(!r.romaji.empty()) {
				idx_general.insert(r.romaji, std::make_tuple(std::string_view(r.romaji), &entry));
			}
		}
		for(auto& s : entry.senses) {
			for(auto& g : s.glosses) {
				idx_general.insert(g.content, std::make_tuple(std::string_view(g.content), &entry));
			}
		}
	}

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
}

} // namespace jdict
