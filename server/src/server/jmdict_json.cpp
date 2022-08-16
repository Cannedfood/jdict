#include "./jmdict_json.hpp"

namespace jdict {

nlohmann::json to_json(jmdict::sense::example::sentence const& s) {
	nlohmann::json result;
	if(!s.language.empty()) result["lang"] = s.language;
	result["value"] = s.value;
	return result;
}

nlohmann::json to_json(jmdict::sense::example const& e) {
	nlohmann::json result;
	result["source"] = e.source;
	result["form_in_example"] = e.form_in_example;
	result["sentences"] = to_json(e.sentences);
	return result;
}
nlohmann::json to_json(jmdict::sense::gloss const& g) {
	nlohmann::json result;
	result["content"] = g.content;
	if(!g.language.empty()) result["lang"] = g.language;
	if(!g.gender.empty())   result["gender"] = g.gender;
	if(!g.type.empty())	 result["type"] = g.type;
	if(g.highlight)		 result["highlight"] = g.highlight;
	return result;
}
nlohmann::json to_json(jmdict::sense::source_language const& l) {
	nlohmann::json result;
	result["word"] = l.word;
	if(!l.language.empty()) result["lang"] = l.language;
	if(l.partial)		   result["partial"] = l.partial;
	if(l.waseieigo)		 result["waseieigo"] = l.waseieigo;
	return result;
}

nlohmann::json to_json(jmdict::kanji   const& k) {
	nlohmann::json result;
	result["value"] = k.value;
	if(!k.infos.empty())	  result["infos"] = k.infos;
	if(!k.priorities.empty()) result["priorities"] = k.priorities;
	return result;
}
nlohmann::json to_json(jmdict::reading const& r) {
	nlohmann::json result;
	result["value"] = r.value;
	if(r.not_actual_reading)	  result["not_actual_reading"] = r.not_actual_reading;
	if(!r.restrict_kanji.empty()) result["restrict_kanji"] = r.restrict_kanji;
	if(!r.infos.empty())		  result["infos"] = r.infos;
	if(!r.priorities.empty())	 result["priorities"] = r.priorities;
	return result;
}
nlohmann::json to_json(jmdict::sense   const& s) {
	nlohmann::json result;
	if(!s.restrict_kanji.empty())	  result["restrict_kanji"] = s.restrict_kanji;
	if(!s.restrict_reading.empty())	result["restrict_reading"] = s.restrict_reading;
	if(!s.part_of_speech_tags.empty()) result["part_of_speech_tags"] = s.part_of_speech_tags;
	if(!s.cross_references.empty())	result["cross_references"] = s.cross_references;
	if(!s.antonyms.empty())			result["antonyms"] = s.antonyms;
	if(!s.fields.empty())			  result["fields"] = s.fields;
	if(!s.misc_info.empty())		   result["misc_info"] = s.misc_info;
	if(!s.sense_info.empty())		  result["sense_info"] = s.sense_info;
	if(!s.lang_origin.empty())		 result["lang_origin"] = to_json(s.lang_origin);
	if(!s.dialects.empty())			result["dialects"] = s.dialects;
	if(!s.glosses.empty())			 result["glosses"]  = to_json(s.glosses);
	if(!s.examples.empty())			result["examples"] = to_json(s.examples);
	return result;
}

nlohmann::json to_json(jmdict::entry const& entry) {
	nlohmann::json result;
	result["sequence"] = entry.sequence;
	for(auto& k : entry.kanji)
		result["kanji"].push_back(to_json(k));
	for(auto& r : entry.readings)
		result["readings"].push_back(to_json(r));
	for(auto& s : entry.senses)
		result["senses"].push_back(to_json(s));
	return result;
}

} // namespace jmdict

