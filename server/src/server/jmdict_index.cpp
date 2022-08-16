#include "./jmdict_index.hpp"

#include "./timer.hpp"
#include "./kana_util.hpp"

#include <set>
#include <cstdio>

namespace jdict {

std::vector<jmdict::entry const*> jmdict_index::search(std::string_view query) const {
	debug::timer _("search");

	// Find and rate results
	std::map<jmdict::entry const*, unsigned> weights;

	findByTranslation(weights, 1, query);
	findBySequenceNumber(weights, 1, query);
	findByReading(weights, 1, query);

	return sortResults(std::move(weights));
}

void jmdict_index::findBySequenceNumber(ResultWeights& weights, int baseWeight, std::string_view query) const {
	if(auto bySeq = bySequenceNumber.find(query); bySeq != bySequenceNumber.end()) {
		weights[bySeq->second] = baseWeight * 1;
	}
}
void jmdict_index::findByReading(ResultWeights& weights, int baseWeight, std::string_view query) const {
	auto readingQuery = to_romaji(query);
	for(auto& [reading, entry] : byReading) {
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
	}
}
void jmdict_index::findByTranslation(ResultWeights& weights, int baseWeight, std::string_view query) const {
	auto readingQuery = to_romaji(query);
	for(auto& [translation, entry] : byTranslation) {
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
	}
}

std::vector<jmdict::entry const*> jmdict_index::sortResults(ResultWeights&& weights) {
	// Sort results by rating
	auto pairs = std::vector<std::pair<jmdict::entry const*, unsigned>>(weights.begin(), weights.end());
	weights.clear();
	std::sort(pairs.begin(), pairs.end(), [](auto const& a, auto const& b) {
		return a.second > b.second;
	});

	// Compact results
	std::vector<jmdict::entry const*> sortedEntries;
	sortedEntries.reserve(pairs.size());
	for(auto [e, rating] : pairs) {
		sortedEntries.push_back(e);
	}
	return sortedEntries;
}

jmdict_index::jmdict_index(jmdict const& dict) :
	dict(&dict)
{
	debug::timer _("building index");
	for(auto& entry : dict.entries) {
		bySequenceNumber.emplace(entry.sequence, &entry);
		for(auto& r : entry.readings) {
			byReading.emplace(to_romaji(r.value), &entry);
		}
		for(auto& s : entry.senses) {
			for(auto& g : s.glosses) {
				byTranslation.emplace(g.content, &entry);
			}
		}
	}
}

} // namespace jdict
