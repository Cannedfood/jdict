#pragma once

#include "./utf8.hpp"
#include "./utf8_sliding_window.hpp"

#include <algorithm>
#include <functional>
#include <map>
#include <set>
#include <string>
#include <tuple>
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
template<class T, class... Results>
concept ResultCallback   = std::is_invocable_v<T, Results...>;

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

		auto window = utf8::sliding_window(s);
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

template<IndexingStrategy Strategy, class... Values>
struct full_text_index {
	using entry_t = std::tuple<std::string_view, Values...>;
	using set_t   = std::vector<unsigned>;

	Strategy strategy;
	std::vector<entry_t> values;
	std::unordered_map<std::string_view, set_t> entries;

	full_text_index(Strategy s = {}) noexcept :
		strategy(std::move(s))
	{}

	void insert(entry_t value) noexcept {
		unsigned value_index = values.size();
		values.push_back(value);
	}
	void insert(std::string_view text, Values... value) noexcept {
		insert(entry_t(text, std::move(value)...));
	}

	template<class Order = std::less<>, class Comp = std::equal_to<>>
	size_t remove_duplicates(Order order = {}, Comp comp = {}) {
		size_t before = values.size();
		std::sort(values.begin(), values.end(), order);
		values.erase(
			std::unique(values.begin(), values.end(), comp),
			values.end()
		);
		return before - values.size();
	}

	void build() {
		// Sort by memory order
		std::sort(values.begin(), values.end(), [](entry_t const& a, entry_t const& b) {
			return std::get<0>(a).data() < std::get<0>(b).data();
		});

		// Build entries
		for(unsigned i = 0; i < values.size(); i++) {
			auto& text = std::get<0>(values[i]);
			strategy.get_fragments(text, [&](std::string_view s) {
				auto& set = entries[s];
				if(set.empty() || set.back() != i)
					entries[s].push_back(i);
			});
		}
	}

	void find(std::string_view query, ResultCallback<std::string_view, Values...> auto emit_result) const noexcept {
		std::vector<unsigned> const* smallestSet = nullptr;
		strategy.get_fragments(query, [&](std::string_view fragment) {
			auto iter = entries.find(fragment);
			if(iter == entries.end())
				return;
			if(!smallestSet || iter->second.size() < smallestSet->size()) {
				smallestSet = &iter->second;
			}
		});
		if(smallestSet) {
			for(auto idx : *smallestSet) {
				auto& v = values[idx];
				if(std::get<0>(v).find(query) != std::string::npos) {
					std::apply(emit_result, values[idx]);
				}
			}
		}
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
