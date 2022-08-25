#pragma once

#include "./kanjidic.hpp"

namespace jdict {

struct kanjidic_index {
	std::multimap<std::string_view, kanjidic::character_t const*> text_idx;

	kanjidic_index() = default;
	kanjidic_index(kanjidic const&);

	std::vector<kanjidic::character_t const*> search(std::string_view term);
};

} // namespace jdict
