#pragma once

#include <string_view>

namespace jdict::utf8 {

constexpr char32_t ReplacementCharacter = 0xFFFD;

constexpr inline static
bool is_continuation_byte(char c) noexcept { return (c & 0b1100'0000) == 0b1000'0000; }

constexpr inline
int encoded_size(char c) noexcept {
	if(c < 127) return 1;
	else if((c & 0b11100000) == 0b11000000) return 2;
	else if((c & 0b11110000) == 0b11100000) return 3;
	else if((c & 0b11111000) == 0b111100000) return 4;
	else if((c & 0b11111100) == 0b11111000) return 5;
	else if((c & 0b11111110) == 0b11111100) return 6;
	return 0;
}

constexpr inline
char32_t snip_codepoint(std::string_view& s) noexcept {
	if(s.empty()) return 0;

	char c = s.front();
	if(c < 127) {
		s.remove_prefix(1);
		return c;
	}
	else if((c & 0b11100000) == 0b11000000) {
		if(s.size() < 2) return ReplacementCharacter;
		char32_t result =
			(s[0] << 3)         << 6 |
			(s[1] & 0b00111111) << 0*6;
		s.remove_prefix(2);
		return result;
	}
	else if((c & 0b11110000) == 0b11100000) {
		if(s.size() < 3) return ReplacementCharacter;
		char32_t result =
			(s[0] << 4)         << 2*6 |
			(s[1] & 0b00111111) << 1*6 |
			(s[2] & 0b00111111) << 0*6;
		s.remove_prefix(3);
		return result;
	}
	else if((c & 0b11111000) == 0b111100000) {
		if(s.size() < 4) return ReplacementCharacter;
		char32_t result =
			(s[0] << 5)         << 3*6 |
			(s[1] & 0b00111111) << 2*6 |
			(s[2] & 0b00111111) << 1*6 |
			(s[3] & 0b00111111) << 0*6;
		s.remove_prefix(4);
		return result;
	}
	else if((c & 0b11111100) == 0b11111000) {
		if(s.size() < 5) return ReplacementCharacter;
		char32_t result =
			(s[0] << 6)         << 4*6 |
			(s[1] & 0b00111111) << 3*6 |
			(s[2] & 0b00111111) << 2*6 |
			(s[3] & 0b00111111) << 1*6 |
			(s[4] & 0b00111111) << 0*6;
		s.remove_prefix(5);
		return result;
	}
	else if((c & 0b11111110) == 0b11111100) {
		if(s.size() < 6) return ReplacementCharacter;
		char32_t result =
			(s[0] << 7)         << 5*6 |
			(s[1] & 0b00111111) << 4*6 |
			(s[2] & 0b00111111) << 3*6 |
			(s[3] & 0b00111111) << 2*6 |
			(s[4] & 0b00111111) << 1*6 |
			(s[5] & 0b00111111) << 0*6;
		s.remove_prefix(6);
		return result;
	}
	return ReplacementCharacter;
}

} // namespace jdict::utf8
