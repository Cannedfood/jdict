#pragma once

#include "./utf8.hpp"
#include <cassert>
#include <utility>

namespace jdict {

struct utf8_sliding_window {
	utf8_sliding_window(std::string_view full_string) noexcept :
		string_end(full_string.data() + full_string.size()),
		window_start(full_string.data()),
		window_end(full_string.data())
	{
		prepare_next();
	}

	inline bool     at_end() const noexcept { return remaining_bytes() == 0; }
	inline char32_t peek() const noexcept { return next_char; }
	inline unsigned size() const noexcept { return nchars; }
	inline void     clear() noexcept { window_start = window_end; nchars = 0; }

	template<class Pred>
	void skip(Pred&& pred) {
		while(pred(next_char) && grow_back());
		clear();
	}

	template<class Callback>
	bool slide(int windowSize, Callback&& emit_fragment) {
		return slide(windowSize, std::forward<Callback>(emit_fragment));
	}

	template<class Callback, class CharPred>
	bool slide(int windowSize, Callback&& emit_fragment, CharPred&& accepts_char_class) {
		while(accepts_char_class(peek()) && size() < windowSize) {
			if(!grow_back())
				break;
		}
		if(!size()) return false;

		emit_fragment(value());
		while(accepts_char_class(peek()) && shift()) {
			emit_fragment(value());
		}

		clear();
		return true;
	}

	bool shift() {
		shrink_front();
		return grow_back();
	}

	void shrink_front() {
		if(window_start == window_end) return;
		char32_t _;
		window_start += utf8::decode(window_start, window_end - window_start, &_);
		nchars--;
	}
	bool grow_back() {
		if(next_char_length < 1)
			return false;
		window_end += next_char_length;
		prepare_next();
		nchars++;
		return true;
	}

	std::string_view value() const noexcept { return std::string_view(window_start, window_end); }
private:
	const char* window_start;
	const char* window_end;
	const char* string_end;

	int nchars = 0;

	int next_char_length = 0;
	char32_t next_char = 0;

	void prepare_next() {
		next_char_length = utf8::decode(
			window_end,
			remaining_bytes(),
			&next_char
		);
	}
	unsigned remaining_bytes() const noexcept {
		return string_end - window_end;
	}
};

} // namespace jdict
