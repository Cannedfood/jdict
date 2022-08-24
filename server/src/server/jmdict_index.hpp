#pragma once

#include "./jmdict.hpp"
#include "./util/full_text_index.hpp"

#include <functional>
#include <set>
#include <string_view>
#include <map>
#include <utility>

namespace jdict {

class jmdict_index {
public:
	using rating_t = int;
	using entry_ptr = jmdict::entry const*;
	using result_ratings_t = std::map<entry_ptr, rating_t>;
	using strview_map_t = std::map<std::string_view, entry_ptr, std::less<>>;
	using ngram_index_t = full_text_index<ngram_indexing_strategy, entry_ptr, rating_t>;
	using result_t = std::pair<entry_ptr, rating_t>;
	using results_t = std::vector<std::pair<entry_ptr, rating_t>>;

public:
	jmdict_index() = default;
	jmdict_index(jmdict const& dict);
	results_t search(std::string_view query) const;

private:
	jmdict const* dict = nullptr;
	strview_map_t idx_sequence_number;
	ngram_index_t idx_general = ngram_index_t(ngram_indexing_strategy {});

	void find_by_sequence_number(result_ratings_t& results_out, std::string_view query) const;
	void find_general           (result_ratings_t& results_out, std::string_view query) const;

	void build_indices();

	static results_t sort_results(result_ratings_t&& ratings);
};

} // namespace jdict
