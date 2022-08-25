#include "./to_json.hpp"
#include "server/database/kanjidic/kanjidic.hpp"
#include <cstdio>

namespace jdict {

nlohmann::json to_json(jmdict::sense_t::example::sentence const& s) {
	nlohmann::json result;
	if(!s.language.empty()) result["lang"] = s.language;
	result["value"] = s.value;
	return result;
}

nlohmann::json to_json(jmdict::sense_t::example const& e) {
	nlohmann::json result;
	result["source"]          = e.source;
	result["form_in_example"] = e.form_in_example;
	result["sentences"]       = to_json(e.sentences);
	return result;
}
nlohmann::json to_json(jmdict::sense_t::gloss const& g) {
	nlohmann::json result;
	result["content"] = g.content;
	if(!g.language.empty()) result["lang"]      = g.language;
	if(!g.gender.empty())   result["gender"]    = g.gender;
	if(!g.type.empty())     result["type"]      = g.type;
	if(g.highlight)         result["highlight"] = g.highlight;
	return result;
}
nlohmann::json to_json(jmdict::sense_t::source_language const& l) {
	nlohmann::json result;
	result["word"] = l.word;
	if(!l.language.empty()) result["lang"]      = l.language;
	if(l.partial)           result["partial"]   = l.partial;
	if(l.waseieigo)         result["waseieigo"] = l.waseieigo;
	return result;
}

nlohmann::json to_json(jmdict::kanji_t   const& k) {
	nlohmann::json result;
	result["value"] = k.value;
	if(!k.infos.empty())      result["infos"]      = k.infos;
	if(!k.priorities.empty()) result["priorities"] = k.priorities;
	return result;
}
nlohmann::json to_json(jmdict::reading_t const& r) {
	nlohmann::json result;
	result["value"] = r.value;
	if(!r.romaji.empty())         result["romaji"]             = r.romaji;
	if(r.not_actual_reading)      result["not_actual_reading"] = r.not_actual_reading;
	if(!r.restrict_kanji.empty()) result["restrict_kanji"]     = r.restrict_kanji;
	if(!r.infos.empty())          result["infos"]              = r.infos;
	if(!r.priorities.empty())     result["priorities"]         = r.priorities;
	return result;
}
nlohmann::json to_json(jmdict::sense_t const& s) {
	nlohmann::json result;
	if(!s.restrict_kanji.empty())      result["restrict_kanji"]      = s.restrict_kanji;
	if(!s.restrict_reading.empty())    result["restrict_reading"]    = s.restrict_reading;
	if(!s.part_of_speech_tags.empty()) result["part_of_speech_tags"] = s.part_of_speech_tags;
	if(!s.cross_references.empty())    result["cross_references"]    = s.cross_references;
	if(!s.antonyms.empty())            result["antonyms"]            = s.antonyms;
	if(!s.fields.empty())              result["fields"]              = s.fields;
	if(!s.misc_info.empty())           result["misc_info"]           = s.misc_info;
	if(!s.sense_info.empty())          result["sense_info"]          = s.sense_info;
	if(!s.lang_origin.empty())         result["lang_origin"]         = to_json(s.lang_origin);
	if(!s.dialects.empty())            result["dialects"]            = s.dialects;
	if(!s.glosses.empty())             result["glosses"]             = to_json(s.glosses);
	if(!s.examples.empty())            result["examples"]            = to_json(s.examples);
	return result;
}

nlohmann::json to_json(jmdict::entry_t const& entry) {
	nlohmann::json result;
	result["id"] = entry.sequence;

	result["kanji"]    = nlohmann::json::value_type::array();
	result["readings"] = nlohmann::json::value_type::array();
	result["senses"]   = nlohmann::json::value_type::array();

	for(auto& k : entry.kanji)
		result["kanji"].push_back(to_json(k));
	for(auto& r : entry.readings)
		result["readings"].push_back(to_json(r));
	for(auto& s : entry.senses)
		result["senses"].push_back(to_json(s));
	return result;
}

nlohmann::json to_json(std::pair<jmdict::entry_t const*, int> const& e) {
	auto result = to_json(*e.first);
	result["rating"] = e.second;
	return result;
}



nlohmann::json to_json(kanjidic::query_code_t const& q) {
	nlohmann::json result;
	result["type"] = to_string(q.type);
	result["value"] = q.value;
	if(q.skip_misclass != kanjidic::skip_misclass_t::none)
		result["skip_misclassification"] = to_string(q.skip_misclass);
	return result;
}
nlohmann::json to_json(kanjidic::radical_t const& m) {
	nlohmann::json result;
	if(m.classical != 0) result["classical"] = m.classical;
	if(m.nelson_c != 0) result["nelson_c"] = m.nelson_c;
	return result;
}
nlohmann::json to_json(kanjidic::misc_t const& m) {
	nlohmann::json result;
	if(m.grade != 0)
		result["grade"] = m.grade;
	if(!m.stroke_count.empty())
		result["stroke_count"] = m.stroke_count;
	if(!m.variant.empty())
		result["variant"] = to_json(m.variant);
	if(m.freq != 0)
		result["freq"] = m.freq;
	if(!m.rad_name.empty())
		result["rad_name"] = m.rad_name;
	if(m.jlpt != 0)
		result["jlpt"] = m.jlpt;
	return result;
}
nlohmann::json to_json(kanjidic::codepoint_t const& c) {
	nlohmann::json result;
	if(!c.jis208.empty()) result["jis208"] = c.jis208;
	if(!c.jis212.empty()) result["jis212"] = c.jis212;
	if(!c.jis213.empty()) result["jis213"] = c.jis213;
	if(!c.ucs.empty())    result["ucs"] = c.ucs;
	return result;
}
nlohmann::json to_json(kanjidic::variant_t const& v) {
	nlohmann::json result;
	result["type"] = to_string(v.type);
	result["value"] = v.value;
	return result;
}
nlohmann::json to_json(kanjidic::rm_group_t const& g) {
	nlohmann::json result;
	if(!g.readings.empty()) result["readings"] = to_json(g.readings);
	if(!g.meanings.empty()) result["meanings"] = to_json(g.meanings);
	return result;
}
nlohmann::json to_json(kanjidic::reading_t  const& r) {
	nlohmann::json result;
	result["value"] = r.value;
	result["type"] = to_string(r.type);
	if(r.approved_for_joyou_kanji) result["approved_for_joyou_kanji"] = true;
	if(r.on_type != kanjidic::on_type_t::none) result["on_type"] = to_string(r.on_type);
	return result;
}
nlohmann::json to_json(kanjidic::meaning_t  const& m) {
	nlohmann::json result;
	result["value"] = m.value;
	if(!m.lang.empty()) result["lang"] = m.lang;
	return result;
}

nlohmann::json to_json(kanjidic::character_t const& c) {
	nlohmann::json result = to_json(c.misc);
	result["literal"] = c.literal;
	if(c.codepoint)
		result["codepoint"] = to_json(c.codepoint);
	if(c.radical)
		result["radical"] = to_json(c.radical);
	if(!c.query_code.empty())
		result["query_code"] = to_json(c.query_code);
	if(!c.reading_meaning_groups.empty())
		result["reading_meaning_groups"] = to_json(c.reading_meaning_groups);
	if(!c.nanori.empty())
		result["nanori"] = c.nanori;
	return result;
}

} // namespace jmdict

