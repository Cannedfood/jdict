#pragma once

#include "./kanjidic.hpp"

namespace jdict {

struct kanjidic_index {
	std::map<std::string_view, kanjidic::character_t const*> utf8_idx;
	std::map<char32_t, kanjidic::character_t const*> codepoint_idx;

	kanjidic_index() = default;
	kanjidic_index(kanjidic const&);

	kanjidic::character_t const* find(char32_t codepoint);
	kanjidic::character_t const* find(std::string_view encoded_utf8);
};

} // namespace jdict
