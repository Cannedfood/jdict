#pragma once

#include <algorithm>
#include <functional>
#include <initializer_list>
#include <memory>
#include <optional>
#include <set>
#include <type_traits>
#include <utility>
#include <vector>
#include <tuple>
#include <string_view>
#include <string>
#include <map>

namespace jdict {

template<class T>
concept IndexingStrategy =
	std::is_default_constructible_v<T> &&
	std::is_move_assignable_v<T> &&
	std::is_move_constructible_v<T> &&
	requires(T const t, std::string_view s) {
		t.get_fragments(s, [](std::string_view) {});
	};
template<class T>
concept FragmentCallback = std::is_invocable_v<T, std::string_view>;
template<class T, class Result>
concept ResultCallback   = std::is_invocable_v<T, Result>;

struct word_indexing_strategy {
	std::set<std::string, std::less<>> stop_words;

	void get_fragments(std::string_view s, FragmentCallback auto emitFragment) const {
		while(!s.empty()) {
			while(!s.empty() && s.front() <= ' ') s.remove_prefix(1);
			size_t i = 0;
			while(i < s.size() && s[i] >= ' ') i++;
			auto word = s.substr(0, i);
			if(!stop_words.contains(word))
				emitFragment(word);
			s.remove_prefix(i);
		}
	}
};
static_assert(IndexingStrategy<word_indexing_strategy>, "word_indexing_strategy isn't a IndexingStrategy");

struct ngram_indexing_strategy {
	int n = 2;

	void get_fragments(std::string_view s, FragmentCallback auto emitFragment) const {
		if(s.size() < n) {
			emitFragment(s);
		}
		else {
			for(int i = 0; i < int(s.size()) - n; i++) {
				emitFragment(s.substr(i, n));
			}
		}
	}
};
static_assert(IndexingStrategy<ngram_indexing_strategy>, "ngram_indexing_strategy isn't a IndexingStrategy");


template<class T, IndexingStrategy Strategy = ngram_indexing_strategy>
struct text_index {
	Strategy strategy;
	// TODO: optimize memory footprint
	std::map<std::string_view, std::set<T>, std::less<>> entries;

	text_index(Strategy s = {}) noexcept :
		strategy(std::move(s))
	{}

	void insert(std::string_view v, T value) noexcept {
		strategy.get_fragments(v, [&](std::string_view s) {
			entries[s].insert(value);
		});
	}
	void find(std::string_view s, ResultCallback<T> auto results) const noexcept {
		std::set<T> const* bestSet = nullptr;
		std::set<T> const* secondBestSet = nullptr;
		strategy.get_fragments(s, [&](std::string_view fragment) {
			auto iter = entries.find(fragment);
			if(iter == entries.end())
				return;
			if(!bestSet || iter->second.size() < bestSet->size()) {
				secondBestSet = bestSet;
				bestSet = &iter->second;
			}
		});
		if(bestSet) {
			for(auto& r : *bestSet) {
				results(r);
			}
		}
	}
};

} // namespace jdict
