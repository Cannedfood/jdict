#pragma once

#include <cstdio>
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
int decode(char const* bytes, int nbytes, char32_t* out_codepoint) {
	const auto decodingFailure = [&] {
		*out_codepoint = ReplacementCharacter;
		if(nbytes == 1) return 1;
		int n = 1;
		while(n < nbytes && is_continuation_byte(bytes[n]))
			n++;
		return n;
	};

	if(nbytes == 0) {
		out_codepoint = 0;
		return 0;
	}

	char c = bytes[0];
	if((c & 0b10000000) == 0) {
		*out_codepoint = c;
		return 1;
	}
	else if((c & 0b11100000) == 0b11000000) {
		if(nbytes < 2) return decodingFailure();
		*out_codepoint =
			(bytes[0] & 0b00011111) << 1*6 |
			(bytes[1] & 0b00111111) << 0*6;
		return 2;
	}
	else if((c & 0b11110000) == 0b11100000) {
		if(nbytes < 3) return decodingFailure();
		*out_codepoint =
			(bytes[0] & 0b00001111) << 2*6 |
			(bytes[1] & 0b00111111) << 1*6 |
			(bytes[2] & 0b00111111) << 0*6;
		return 3;
	}
	else if((c & 0b11111000) == 0b111100000) {
		if(nbytes < 4) return decodingFailure();
		*out_codepoint =
			(bytes[0] & 0b00000111) << 3*6 |
			(bytes[1] & 0b00111111) << 2*6 |
			(bytes[2] & 0b00111111) << 1*6 |
			(bytes[3] & 0b00111111) << 0*6;
		return 4;
	}
	else if((c & 0b11111100) == 0b11111000) {
		if(nbytes < 5) return decodingFailure();
		*out_codepoint =
			(bytes[0] & 0b00000011) << 4*6 |
			(bytes[1] & 0b00111111) << 3*6 |
			(bytes[2] & 0b00111111) << 2*6 |
			(bytes[3] & 0b00111111) << 1*6 |
			(bytes[4] & 0b00111111) << 0*6;
		return 5;
	}
	else if((c & 0b11111110) == 0b11111100) {
		if(nbytes < 6) return decodingFailure();
		*out_codepoint =
			(bytes[0] & 0b00000001) << 5*6 |
			(bytes[1] & 0b00111111) << 4*6 |
			(bytes[2] & 0b00111111) << 3*6 |
			(bytes[3] & 0b00111111) << 2*6 |
			(bytes[4] & 0b00111111) << 1*6 |
			(bytes[5] & 0b00111111) << 0*6;
		return 6;
	}
	return decodingFailure();
}

constexpr inline
char32_t decode(std::string_view& s) noexcept {
	char32_t result = 0;
	decode(s.data(), s.size(), &result);
	return result;
}

constexpr inline
char32_t decode_and_snip(std::string_view& s) noexcept {
	char32_t result = 0;
	s.remove_prefix(decode(s.data(), s.size(), &result));
	return result;
}

constexpr inline static bool is_ascii(char32_t c) { return c < 127; }
constexpr inline static bool is_cjk_base(char32_t c) { return c >= 0x4e00 && c <= 0x9FFF; }
constexpr inline static bool is_cjk_extension_a(char32_t c) { return c >= 0x3400  && c <= 0x4DBF;  }
constexpr inline static bool is_cjk_extension_b(char32_t c) { return c >= 0x20000 && c <= 0x2A6DF; }
constexpr inline static bool is_cjk_extension_c(char32_t c) { return c >= 0x2A700 && c <= 0x2B73F; }
constexpr inline static bool is_cjk_extension_d(char32_t c) { return c >= 0x2B740 && c <= 0x2B81F; }
constexpr inline static bool is_cjk_extension_e(char32_t c) { return c >= 0x2B820 && c <= 0x2CEAF; }
constexpr inline static bool is_cjk_extension_f(char32_t c) { return c >= 0x2CEB0 && c <= 0x2EBEF; }
constexpr inline static bool is_cjk_extension_g(char32_t c) { return c >= 0x30000 && c <= 0x3134F; }
constexpr inline static bool is_cjk_compat     (char32_t c) { return c >= 0xF900  && c <= 0xFAFF; }
constexpr inline static bool is_cjk(char32_t c) {
	return (
		is_cjk_base(c) ||
		is_cjk_extension_a(c) ||
		is_cjk_extension_b(c) ||
		is_cjk_extension_c(c) ||
		is_cjk_extension_d(c) ||
		is_cjk_extension_e(c) ||
		is_cjk_extension_f(c) ||
		is_cjk_extension_g(c) ||
		is_cjk_compat(c)
	);
}

constexpr inline static bool is_katakana(char32_t c) { return c >= 0x3040 && c < 0x30A0; }
constexpr inline static bool is_hiragana(char32_t c) { return c >= 0x30A0 && c < 0x3100; }
constexpr inline static bool is_kana(char32_t c) { return is_hiragana(c) || is_katakana(c); }
constexpr inline static bool is_kanji(char32_t c) { return is_cjk(c); }

} // namespace jdict::utf8
