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
	find_by_translation(weights, 1, query);
	find_by_reading(weights, 1, query);

	return sortResults(std::move(weights));
}

void jmdict_index::find_by_sequence_number(ResultWeights& weights, int baseWeight, std::string_view query) const {
	if(auto bySeq = bySequenceNumber.find(query); bySeq != bySequenceNumber.end()) {
		weights[bySeq->second] = baseWeight * 1;
	}
}
void jmdict_index::find_by_reading(ResultWeights& weights, int baseWeight, std::string_view query) const {
	auto readingQuery = to_romaji(query);
	idx_reading.find(query, [&](std::pair<std::string_view, jmdict::entry const*> const& e) {
		auto& [reading, entry] = e;
		bool containedInReading = reading.find(readingQuery) != std::string::npos;
		if(containedInReading) {
			unsigned rating = 1;
			if(reading == readingQuery) rating += 1000; // Exact match
			else if(reading.starts_with(readingQuery)) rating += 10; // Starts with search
			// TODO: occurence rating
			rating += 10 * std::max(0.f, 1 - reading.size() / 50.f); // Smaller is better
			rating *= baseWeight;

			auto& hitRating = weights[entry];
			hitRating = std::max(hitRating, rating);
		}
	});
}
void jmdict_index::find_by_translation(ResultWeights& weights, int baseWeight, std::string_view query) const {
	auto readingQuery = to_romaji(query);
	idx_translation.find(query, [&](std::pair<std::string_view, jmdict::entry const*> const& e) {
		auto& [translation, entry] = e;
		bool contained = translation.find(readingQuery) != std::string::npos;
		if(contained) {
			unsigned rating = 1;
			if(translation == readingQuery) rating += 1000; // Exact match
			else if(translation.starts_with(readingQuery)) rating += 10; // Starts with search
			// TODO: occurence rating
			rating += 10 * std::max(0.f, 1 - translation.size() / 50.f); // Smaller is better
			rating *= baseWeight;

			auto& hitRating = weights[entry];
			hitRating = std::max(hitRating, rating);
		}
	});
}

std::vector<jmdict::entry const*> jmdict_index::sortResults(ResultWeights&& weights) {
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
		bySequenceNumber.emplace(entry.sequence, &entry);
		for(auto& r : entry.readings) {
			if(!r.romaji.empty()) {
				idx_reading.insert(r.romaji, std::make_pair(std::string_view(r.romaji), &entry));
			}
		}
		for(auto& s : entry.senses) {
			for(auto& g : s.glosses) {
				idx_translation.insert(g.content, std::make_pair(std::string_view(g.content), &entry));
			}
		}
	}
}

} // namespace jdict
