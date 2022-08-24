#pragma once

#include "./jmdict.hpp"
#include "./util/text_index.hpp"

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
public:
	using entry_ptr = jmdict::entry const*;
	struct index_entry {
		std::string_view value;
		entry_ptr entry;
		unsigned weight;
	};
	using StringViewMap = std::map<std::string_view, entry_ptr, std::less<>>;
	using ResultWeights = std::map<entry_ptr, unsigned>;
	using ngram_index  = text_index<std::tuple<std::string_view, entry_ptr>, ngram_indexing_strategy>;
private:
	jmdict const* dict = nullptr;
	StringViewMap idx_sequence_number;
	ngram_index   idx_general = ngram_index(ngram_indexing_strategy {});

	void find_by_sequence_number(ResultWeights& results_out, int baseWeight, std::string_view query) const;
	void find_general           (ResultWeights& results_out, int baseWeight, std::string_view query) const;

	void build_indices();

	static unsigned rate_match(std::string_view query, std::string_view match);
	static std::vector<entry_ptr> sort_results(ResultWeights&& weights);
};

} // namespace jdict
