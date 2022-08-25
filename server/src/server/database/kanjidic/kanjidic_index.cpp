#include "./kanjidic_index.hpp"

#include <server/util/utf8.hpp>

namespace jdict {

kanjidic_index::kanjidic_index(kanjidic const& dic) {
	for(auto& c : dic.characters) {
		utf8_idx.emplace(c.literal, &c);
		codepoint_idx.emplace(utf8::decode(c.literal), &c);
	}
}

kanjidic::character_t const* kanjidic_index::find(char32_t codepoint) {
	auto iter = this->codepoint_idx.find(codepoint);
	if(iter != this->codepoint_idx.end())
		return iter->second;
	return nullptr;
}
kanjidic::character_t const* kanjidic_index::find(std::string_view encoded_utf8) {
	auto iter = this->utf8_idx.find(encoded_utf8);
	if(iter != this->utf8_idx.end())
		return iter->second;
	return nullptr;
}

} // namespace jdict
