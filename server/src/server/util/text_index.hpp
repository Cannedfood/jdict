#pragma once

#include "./utf8.hpp"
#include "./utf8_sliding_window.hpp"

#include <algorithm>
#include <functional>
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
	int n_alpha   = 3;
	int n_kanji   = 1;
	int n_kana    = 2;
	int n_unicode = 1;

	void get_fragments(std::string_view s, FragmentCallback auto emit_fragment) const {
		auto ignored_characters = [](char32_t c) {
			return
				utf8::is_numeric(c) ||
				utf8::is_whitespace(c) ||
				utf8::is_punct_ascii(c);
		};

		auto window = utf8_sliding_window(s);
		do { window.skip(ignored_characters); }
		while(
			window.slide(n_alpha,   emit_fragment, utf8::is_alpha) ||
			window.slide(n_kanji,   emit_fragment, utf8::is_kanji) ||
			window.slide(n_kana,    emit_fragment, utf8::is_kana) ||
			window.slide(n_unicode, emit_fragment, [ignored_characters](char32_t c) {
				return !(
					ignored_characters(c) ||
					utf8::is_alpha(c) ||
					utf8::is_kanji(c) ||
					utf8::is_kana(c)
				);
			})
		);
	}
};
static_assert(IndexingStrategy<ngram_indexing_strategy>, "ngram_indexing_strategy isn't a IndexingStrategy");

template<class T, IndexingStrategy Strategy = ngram_indexing_strategy>
struct text_index {
	Strategy strategy;
	// TODO: optimize memory footprint
	std::unordered_map<std::string_view, std::vector<T>> entries;

	text_index(Strategy s = {}) noexcept :
		strategy(std::move(s))
	{}

	void insert(std::string_view v, T value) noexcept {
		strategy.get_fragments(v, [&](std::string_view s) {
			entries[s].push_back(value);
		});
	}
	void find(std::string_view s, ResultCallback<T> auto results) const noexcept {
		std::vector<T> const* bestSet = nullptr;
		std::vector<T> const* secondBestSet = nullptr;
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

	size_t remove_duplicates() {
		size_t n = 0;
		for(auto& [key, set] : entries) {
			size_t before = set.size();
			std::sort(set.begin(), set.end());
			set.erase(
				std::unique(
					set.begin(),
					set.end()
				),
				set.end()
			);
			set.shrink_to_fit();
			n += before - set.size();
		}
		return n;
	}

	void write_stats(std::string const& path) {
		std::vector<std::pair<unsigned, std::string_view>> ee;
		for(auto& [key, set] : entries)
			ee.emplace_back((unsigned) set.size(), key);
		std::sort(ee.begin(), ee.end(), std::greater<>());

		auto* file = fopen(path.c_str(), "w");
		for(auto& e : ee) {
			fprintf(file, "%.*s %u\n", (int) e.second.size(), e.second.data(), e.first);
		}
		fclose(file);
	}
};

} // namespace jdict
