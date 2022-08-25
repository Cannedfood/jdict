#include "./kanjidic_index.hpp"

#include <algorithm>
#include <server/util/utf8.hpp>
#include <utility>

namespace jdict {

kanjidic_index::kanjidic_index(kanjidic const& dic) {
	for(auto& c : dic.characters) {
		text_idx.emplace(c.literal, &c);
		if(!c.codepoint.jis208.empty())
			text_idx.emplace(c.codepoint.jis208, &c);
		if(!c.codepoint.jis212.empty()) 
			text_idx.emplace(c.codepoint.jis212, &c);
		if(!c.codepoint.jis213.empty()) 
			text_idx.emplace(c.codepoint.jis213, &c);
		if(!c.codepoint.ucs.empty()) 
			text_idx.emplace(c.codepoint.ucs, &c);
	}
}

std::vector<kanjidic::character_t const*> kanjidic_index::search(std::string_view term) {
	std::vector<kanjidic::character_t const*> result;
	auto [start, end] = text_idx.equal_range(term);
	std::transform(
		start, end, std::back_inserter(result),
		[](std::pair<std::string_view, kanjidic::character_t const*> p) {
			return p.second;
		}
	);
	return result;
}

} // namespace jdict
