#pragma once

#include "./utf8.hpp"

#include <algorithm>
#include <map>
#include <set>
#include <string>
#include <vector>

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

struct ngram_indexing_strategy {
	int n = 2;

	void get_fragments(std::string_view s, FragmentCallback auto emitFragment) const {
		if(s.size() < n) {
			emitFragment(s);
		}
		else {
			std::string_view ngram;
			for(size_t i = 0; i < n; i++)
				utf8::snip_codepoint(s);

			for(int i = 0; i < int(s.size()) - n; i++) {
				emitFragment(s.substr(i, n));
			}
		}
	}
};
static_assert(IndexingStrategy<ngram_indexing_strategy>, "ngram_indexing_strategy isn't a IndexingStrategy");

struct smart_ngram_indexing_strategy {
	unsigned ascii_n   = 2;
	unsigned unicode_n = 2;
	unsigned kanji_n   = 1;

	void get_fragments(std::string_view s, FragmentCallback auto emitFragment) const {
		if(s.size() < ascii_n) {
			emitFragment(s);
		}
		else {
			std::string_view remaining = s;
			while(!remaining.empty()) {
				auto c = utf8::snip_codepoint(remaining);
				if(c < 127) {

				}
			}
		}
	}
};
static_assert(IndexingStrategy<smart_ngram_indexing_strategy>, "ngram_indexing_strategy isn't a IndexingStrategy");


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

	void writeStats(std::string const& path) {
		std::vector<std::pair<unsigned, std::string_view>> ee;
		for(auto& [key, set] : entries)
			ee.emplace_back((unsigned) set.size(), key);
		std::sort(ee.begin(), ee.end());

		auto* file = fopen(path.c_str(), "w");
		for(auto& e : ee) {
			fprintf(file, "%.*s %u\n", (int) e.second.size(), e.second.data(), e.first);
		}
		fclose(file);
	}
};

} // namespace jdict
