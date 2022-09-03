#include "./kanjidic_index.hpp"

#include <server/util/utf8.hpp>

#include <algorithm>
#include <iterator>
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

	auto subsearch = [&](std::string_view term) {
		auto [start, end] = text_idx.equal_range(term);
		std::transform(
			start, end, std::back_inserter(result),
			[](std::pair<std::string_view, kanjidic::character_t const*> p) {
				return p.second;
			}
		);
	};

	while(!term.empty()) {
		utf8::snip_while(term, [](char32_t c) {
			return !utf8::is_alpha(c) && !utf8::is_cjk(c);
		});
		if(auto text = utf8::snip_while(term, utf8::is_alpha); !text.empty())
			subsearch(text);
		if(auto character = utf8::snip_if(term, utf8::is_cjk); !character.empty())
			subsearch(character);
	}
	return result;
}

} // namespace jdict
