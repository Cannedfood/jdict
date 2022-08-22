#pragma once

#include "./jmdict.hpp"
#include "text_index.hpp"

#include <functional>
#include <set>
#include <string_view>
#include <map>
#include <utility>

namespace jdict {

class jmdict_index {
public:
	jmdict_index() = default;
	jmdict_index(jmdict const& dict);
	std::vector<jmdict::entry const*> search(std::string_view query) const;
private:
	using TextIndex     = std::map<std::string, jmdict::entry const*, std::less<>>;
	using TextViewIndex = std::map<std::string_view, jmdict::entry const*, std::less<>>;
	using ResultWeights = std::map<jmdict::entry const*, unsigned>;
	using ngram_index = text_index<std::pair<std::string_view, jmdict::entry const*>, ngram_indexing_strategy>;
	using word_index  = text_index<jmdict::entry const*, word_indexing_strategy>;

	jmdict const* dict = nullptr;
	TextViewIndex bySequenceNumber;
	ngram_index idx_reading     = ngram_index(ngram_indexing_strategy { .n = 2 });
	ngram_index idx_translation = ngram_index(ngram_indexing_strategy { .n = 2 });

	void find_by_sequence_number(ResultWeights& results_out, int baseWeight, std::string_view query) const;
	void find_by_reading        (ResultWeights& results_out, int baseWeight, std::string_view query) const;
	void find_by_translation    (ResultWeights& results_out, int baseWeight, std::string_view query) const;

	static std::vector<jmdict::entry const*>  sortResults(ResultWeights&& weights);
};

} // namespace jdict
