#include "./kanjidic.hpp"

#include <server/util/file2vector.hpp>
#include <server/util/my_rapidxml_utils.hpp>
#include <server/util/timer.hpp>

#include <bitset>
#include <rapidxml.hpp>
#include <stdexcept>
#include <string>
#include <vector>

using namespace std::string_view_literals;

namespace jdict {

kanjidic::reading_t parse_reading(xml_node& reading) {
	kanjidic::reading_t result;

	result.value = value(reading);

	if(auto* r_type = reading.first_attribute("r_type")) {
		if     (r_type->value() == "pinyin"sv)   result.type = kanjidic::reading_type_t::pinyin;
		else if(r_type->value() == "korean_r"sv) result.type = kanjidic::reading_type_t::korean_r;
		else if(r_type->value() == "korean_h"sv) result.type = kanjidic::reading_type_t::korean_h;
		else if(r_type->value() == "vietnam"sv)  result.type = kanjidic::reading_type_t::vietnam;
		else if(r_type->value() == "ja_on"sv)    result.type = kanjidic::reading_type_t::ja_on;
		else if(r_type->value() == "ja_kun"sv)   result.type = kanjidic::reading_type_t::ja_kun;
		else throw std::runtime_error("Unhandled r_type value: " + value(r_type));
	}
	else throw std::runtime_error("<reading> element is missing 'r_type' attribute");

	if(auto on_type = reading.first_attribute("on_type")) {
		if     (on_type->value() == "kan"sv)     result.on_type = kanjidic::on_type_t::kan;
		else if(on_type->value() == "go"sv)      result.on_type = kanjidic::on_type_t::go;
		else if(on_type->value() == "tou"sv)     result.on_type = kanjidic::on_type_t::tou;
		else if(on_type->value() == "kan'you"sv) result.on_type = kanjidic::on_type_t::kanyou;
		else throw std::runtime_error("Unhandled on_type: " + value(on_type));
	}

	if(auto r_status = reading.first_attribute("r_status"))
		result.approved_for_joyou_kanji = r_status->value() == "jy"sv;

	return result;
}

kanjidic::meaning_t parse_meaning(xml_node& meaning) {
	kanjidic::meaning_t result;
	result.value = value(meaning);
	if(auto* lang = meaning.first_attribute("m_lang"))
		result.lang = value(lang);
	return result;
}

kanjidic::rm_group_t parse_rm_group(xml_node& rm_group) {
	kanjidic::rm_group_t result;
	for(auto& child : children(rm_group)) {
		if(child.name() == "meaning"sv)
			result.meanings.push_back(parse_meaning(child));
		else if(child.name() == "reading"sv)
			result.readings.push_back(parse_reading(child));
		else UNHANDLED_NODE("rm_group", child);
	}
	return result;
}

namespace {

struct reading_meaning_t {
	std::vector<kanjidic::rm_group_t> groups;
	std::vector<std::string> nanori;
};

} // namespace

static reading_meaning_t parse_reading_meaning(xml_node& reading_meaning) {
	reading_meaning_t result;
	for(auto& child : children(reading_meaning)) {
		if(child.name() == "nanori"sv)
			result.nanori.push_back(value(child));
		else if(child.name() == "rmgroup"sv)
			result.groups.push_back(parse_rm_group(child));
		else UNHANDLED_NODE("reading_meaning", child);
	}
	return result;
}

static std::vector<kanjidic::query_code_t> parse_query_code(xml_node& query_code) {
	std::vector<kanjidic::query_code_t> result;
	for(auto& child : children(query_code)) {
		if(child.name() != "q_code"sv)
			UNHANDLED_NODE("query_code", child);

		auto subresult = kanjidic::query_code_t();

		subresult.value = value(child);
		if(auto* skip_misclass = child.first_attribute("skip_misclass")) {
			if     (skip_misclass->value() == "posn"sv)            subresult.skip_misclass = kanjidic::skip_misclass_t::posn;
			else if(skip_misclass->value() == "stroke_count"sv)    subresult.skip_misclass = kanjidic::skip_misclass_t::stroke_count;
			else if(skip_misclass->value() == "stroke_and_posn"sv) subresult.skip_misclass = kanjidic::skip_misclass_t::stroke_and_posn;
			else if(skip_misclass->value() == "stroke_diff"sv)     subresult.skip_misclass = kanjidic::skip_misclass_t::stroke_diff;
			else throw std::runtime_error("Unhandled skip_misclass value: " + value(skip_misclass));
		}

		if(auto* qc_type = child.first_attribute("qc_type")) {
			if     (qc_type->value() == "skip"sv)        subresult.type = kanjidic::query_code_type_t::skip;
			else if(qc_type->value() == "sh_desc"sv)     subresult.type = kanjidic::query_code_type_t::sh_desc;
			else if(qc_type->value() == "four_corner"sv) subresult.type = kanjidic::query_code_type_t::four_corner;
			else if(qc_type->value() == "deroo"sv)       subresult.type = kanjidic::query_code_type_t::deroo;
			else if(qc_type->value() == "misclass"sv)    subresult.type = kanjidic::query_code_type_t::misclass;
			else throw std::runtime_error("Unhandled qc_type value: " + value(qc_type));
		}
		else throw std::runtime_error("<q_code> node is missing a 'qc_type' attribute");

		result.emplace_back(std::move(subresult));
	}
	return result;
}

static std::vector<kanjidic::dic_ref_t> parse_dic_number(xml_node& dic_number) {
	std::vector<kanjidic::dic_ref_t> result;
	for(auto& child : children(dic_number)) {
		if(child.name() != "dic_ref"sv)
			UNHANDLED_NODE("dic_number", child);



		result.emplace_back();

		result.back().index_number = value(child);

		if(auto dr_type = child.first_attribute("dr_type")) {
			if     (dr_type->value() == "nelson_c"sv)         result.back().type = kanjidic::dic_ref_type_t::nelson_c;
			else if(dr_type->value() == "nelson_n"sv)         result.back().type = kanjidic::dic_ref_type_t::nelson_n;
			else if(dr_type->value() == "halpern_njecd"sv)    result.back().type = kanjidic::dic_ref_type_t::halpern_njecd;
			else if(dr_type->value() == "halpern_kkd"sv)      result.back().type = kanjidic::dic_ref_type_t::halpern_kkd;
			else if(dr_type->value() == "halpern_kkld"sv)     result.back().type = kanjidic::dic_ref_type_t::halpern_kkld;
			else if(dr_type->value() == "halpern_kkld_2ed"sv) result.back().type = kanjidic::dic_ref_type_t::halpern_kkld_2ed;
			else if(dr_type->value() == "heisig"sv)           result.back().type = kanjidic::dic_ref_type_t::heisig;
			else if(dr_type->value() == "heisig6"sv)          result.back().type = kanjidic::dic_ref_type_t::heisig6;
			else if(dr_type->value() == "gakken"sv)           result.back().type = kanjidic::dic_ref_type_t::gakken;
			else if(dr_type->value() == "oneill_names"sv)     result.back().type = kanjidic::dic_ref_type_t::oneill_names;
			else if(dr_type->value() == "oneill_kk"sv)        result.back().type = kanjidic::dic_ref_type_t::oneill_kk;
			else if(dr_type->value() == "moro"sv)             result.back().type = kanjidic::dic_ref_type_t::moro;
			else if(dr_type->value() == "henshall"sv)         result.back().type = kanjidic::dic_ref_type_t::henshall;
			else if(dr_type->value() == "sh_kk"sv)            result.back().type = kanjidic::dic_ref_type_t::sh_kk;
			else if(dr_type->value() == "sh_kk2"sv)           result.back().type = kanjidic::dic_ref_type_t::sh_kk2;
			else if(dr_type->value() == "sakade"sv)           result.back().type = kanjidic::dic_ref_type_t::sakade;
			else if(dr_type->value() == "jf_cards"sv)         result.back().type = kanjidic::dic_ref_type_t::jf_cards;
			else if(dr_type->value() == "henshall3"sv)        result.back().type = kanjidic::dic_ref_type_t::henshall3;
			else if(dr_type->value() == "tutt_cards"sv)       result.back().type = kanjidic::dic_ref_type_t::tutt_cards;
			else if(dr_type->value() == "crowley"sv)          result.back().type = kanjidic::dic_ref_type_t::crowley;
			else if(dr_type->value() == "kanji_in_context"sv) result.back().type = kanjidic::dic_ref_type_t::kanji_in_context;
			else if(dr_type->value() == "busy_people"sv)      result.back().type = kanjidic::dic_ref_type_t::busy_people;
			else if(dr_type->value() == "kodansha_compact"sv) result.back().type = kanjidic::dic_ref_type_t::kodansha_compact;
			else if(dr_type->value() == "maniette"sv)         result.back().type = kanjidic::dic_ref_type_t::maniette;
			else throw std::runtime_error("Unhandle dr_type value: " + value(dr_type));
		}
		else throw std::runtime_error("<dic_ref> node is missing a 'dr_type' attribute");

		if(auto* m_vol = child.first_attribute("m_vol"))
			result.back().moro_volume = std::stoul(value(m_vol));
		if(auto* m_page = child.first_attribute("m_page"))
			result.back().moro_volume = std::stoul(value(m_page));
	}
	return result;
}

static kanjidic::variant_t parse_variant(xml_node& variant) {
	kanjidic::variant_t result;

	auto var_type = variant.first_attribute("var_type");
	if(!var_type) throw std::runtime_error("<variant> node is missing a 'var_type' attribute");
	else if(var_type->value() == "jis208"sv)   result.type = kanjidic::variant_type_t::jis208;
	else if(var_type->value() == "jis212"sv)   result.type = kanjidic::variant_type_t::jis212;
	else if(var_type->value() == "jis213"sv)   result.type = kanjidic::variant_type_t::jis213;
	else if(var_type->value() == "deroo"sv)    result.type = kanjidic::variant_type_t::deroo;
	else if(var_type->value() == "njecd"sv)    result.type = kanjidic::variant_type_t::njecd;
	else if(var_type->value() == "s_h"sv)      result.type = kanjidic::variant_type_t::s_h;
	else if(var_type->value() == "nelson_c"sv) result.type = kanjidic::variant_type_t::nelson_c;
	else if(var_type->value() == "oneill"sv)   result.type = kanjidic::variant_type_t::oneill;
	else if(var_type->value() == "ucs"sv)      result.type = kanjidic::variant_type_t::ucs;
	else throw std::runtime_error("Unhandled variant var_type " + value(var_type));

	result.value = value(variant);

	return result;
}

static kanjidic::misc_t parse_misc(xml_node& misc) {
	kanjidic::misc_t result;
	for(auto& child : children(misc)) {
		if     (child.name() == "grade"sv)        result.grade = std::stoul(value(child));
		else if(child.name() == "stroke_count"sv) result.stroke_count.push_back(std::stoul(value(child)));
		else if(child.name() == "variant"sv)      result.variant.push_back(parse_variant(child));
		else if(child.name() == "freq"sv)         result.freq = std::stoul(value(child));
		else if(child.name() == "rad_name"sv)     result.rad_name.push_back(value(child));
		else if(child.name() == "jlpt"sv)         result.jlpt = std::stoul(value(child));
		else UNHANDLED_NODE("misc", child);
	}
	return result;
}

static kanjidic::radical_t parse_radical(xml_node& radical) {
	kanjidic::radical_t result;
	for(auto& child : children(radical)) {
		if(child.name() != "rad_value"sv)
			UNHANDLED_NODE("radical", child);

		auto* rad_type = child.first_attribute("rad_type");
		if(!rad_type)
			throw std::runtime_error("<radical> node is missing 'rad_type' attribute");

		if(rad_type->value() == "classical"sv)
			result.classical = std::stoul(value(child));
		else if(rad_type->value() == "nelson_c"sv)
			result.nelson_c = std::stoul(value(child));
		else
			throw std::runtime_error("Unhandled rad_type: " + value(rad_type));
	}
	return result;
}

static kanjidic::codepoint_t parse_codepoint(xml_node& codepoint) {
	kanjidic::codepoint_t result;
	for(auto& child : children(codepoint)) {
		if(child.name() != "cp_value"sv)
			UNHANDLED_NODE("codepoint", child);

		auto cp_type = child.first_attribute("cp_type");
		if(!cp_type)
			throw std::runtime_error("<cp_value> is missing a cp_type");

		auto cp_type_v = std::string_view(cp_type->value(), cp_type->value_size());
		if     (cp_type_v == "jis208"sv) result.jis208 = value(child);
		else if(cp_type_v == "jis212"sv) result.jis212 = value(child);
		else if(cp_type_v == "jis213"sv) result.jis213 = value(child);
		else if(cp_type_v == "ucs"sv)    result.ucs    = value(child);
		else throw std::runtime_error("Unhandled cp_type in codepoint: " + std::string(cp_type_v));
	}
	return result;
}

static kanjidic::character_t parse_character(xml_node& character) {
	kanjidic::character_t result;
	for(auto& child : children(character)) {
		if     (child.name() == "literal"sv)         result.literal         = value(child);
		else if(child.name() == "codepoint"sv)       result.codepoint       = parse_codepoint(child);
		else if(child.name() == "radical"sv)         result.radical         = parse_radical(child);
		else if(child.name() == "misc"sv)            result.misc            = parse_misc(child);
		else if(child.name() == "dic_number"sv)      result.dic_number      = parse_dic_number(child);
		else if(child.name() == "query_code"sv)      result.query_code      = parse_query_code(child);
		else if(child.name() == "reading_meaning"sv) {
			auto [rm_groups, nanori] = parse_reading_meaning(child);
			result.reading_meaning_groups = std::move(rm_groups);
			result.nanori = std::move(nanori);
		}
		else UNHANDLED_NODE("character", child);
	}
	return result;
}

static kanjidic::header_t parse_header(xml_node& node) {
	kanjidic::header_t result;
	for(auto& child : children(node)) {
		if     (child.name() == "file_version"sv)     result.file_version     = value(child);
		else if(child.name() == "database_version"sv) result.database_version = value(child);
		else if(child.name() == "date_of_creation"sv) result.date_of_creation = value(child);
		else UNHANDLED_NODE("header", child);
	}
	return result;
}

kanjidic kanjidic::parse_file(const char* path) {
	auto file = read_file_to_vector(path);

	debug::timer _("parsing " + std::string(path));

	constexpr auto PARSE_FLAGS =
		rapidxml::parse_trim_whitespace |
		rapidxml::parse_normalize_whitespace |
		rapidxml::parse_no_data_nodes;

	xml_document doc;
	doc.parse<PARSE_FLAGS>(file.data());

	auto root = doc.first_node();
	if(!root)
		throw std::runtime_error("Expected 'kanjidic2' as a root node, got NULL");
	if(root->name() != "kanjidic2"sv)
		throw std::runtime_error("Expected 'kanjidic2' as a root node, got " + std::string(root->name()));

	auto header = root->first_node("header");
	if(!header)
		throw std::runtime_error("kanjidic2 does not contain a 'header' node");

	kanjidic result;
	for(auto& entry : children(root)) {
		if(entry.name() == "character"sv) result.characters.push_back(parse_character(entry));
		else if(entry.name() == "header"sv) result.header = parse_header(entry);
		else UNHANDLED_NODE("JMdict", entry);
	}
	return result;
}

const char* to_string(kanjidic::variant_type_t v) noexcept {
	switch (v) {
	case kanjidic::variant_type_t::jis208: return "jis208";
	case kanjidic::variant_type_t::jis212: return "jis212";
	case kanjidic::variant_type_t::jis213: return "jis213";
	case kanjidic::variant_type_t::deroo: return "deroo";
	case kanjidic::variant_type_t::njecd: return "njecd";
	case kanjidic::variant_type_t::s_h: return "s_h";
	case kanjidic::variant_type_t::nelson_c: return "nelson_c";
	case kanjidic::variant_type_t::oneill: return "oneill";
	case kanjidic::variant_type_t::ucs: return "ucs";
	}
}
const char* to_string(kanjidic::dic_ref_type_t v) noexcept {
	switch (v) {
	case kanjidic::dic_ref_type_t::nelson_c: return "nelson_c";
	case kanjidic::dic_ref_type_t::nelson_n: return "nelson_n";
	case kanjidic::dic_ref_type_t::halpern_njecd: return "halpern_njecd";
	case kanjidic::dic_ref_type_t::halpern_kkd: return "halpern_kkd";
	case kanjidic::dic_ref_type_t::halpern_kkld: return "halpern_kkld";
	case kanjidic::dic_ref_type_t::halpern_kkld_2ed: return "halpern_kkld_2ed";
	case kanjidic::dic_ref_type_t::heisig: return "heisig";
	case kanjidic::dic_ref_type_t::heisig6: return "heisig6";
	case kanjidic::dic_ref_type_t::gakken: return "gakken";
	case kanjidic::dic_ref_type_t::oneill_names: return "oneill_names";
	case kanjidic::dic_ref_type_t::oneill_kk: return "oneill_kk";
	case kanjidic::dic_ref_type_t::moro: return "moro";
	case kanjidic::dic_ref_type_t::henshall: return "henshall";
	case kanjidic::dic_ref_type_t::sh_kk: return "sh_kk";
	case kanjidic::dic_ref_type_t::sh_kk2: return "sh_kk2";
	case kanjidic::dic_ref_type_t::sakade: return "sakade";
	case kanjidic::dic_ref_type_t::jf_cards: return "jf_cards";
	case kanjidic::dic_ref_type_t::henshall3: return "henshall3";
	case kanjidic::dic_ref_type_t::tutt_cards: return "tutt_cards";
	case kanjidic::dic_ref_type_t::crowley: return "crowley";
	case kanjidic::dic_ref_type_t::kanji_in_context: return "kanji_in_context";
	case kanjidic::dic_ref_type_t::busy_people: return "busy_people";
	case kanjidic::dic_ref_type_t::kodansha_compact: return "kodansha_compact";
	case kanjidic::dic_ref_type_t::maniette: return "maniette";
	}
}
const char* to_string(kanjidic::query_code_type_t v) noexcept {
	switch (v) {
	case kanjidic::query_code_type_t::skip: return "skip";
	case kanjidic::query_code_type_t::sh_desc: return "sh_desc";
	case kanjidic::query_code_type_t::four_corner: return "four_corner";
	case kanjidic::query_code_type_t::deroo: return "deroo";
	case kanjidic::query_code_type_t::misclass: return "misclass";
	}
}
const char* to_string(kanjidic::skip_misclass_t v) noexcept {
	switch (v) {
	case kanjidic::skip_misclass_t::none: return "none";
	case kanjidic::skip_misclass_t::posn: return "posn";
	case kanjidic::skip_misclass_t::stroke_count: return "stroke_count";
	case kanjidic::skip_misclass_t::stroke_and_posn: return "stroke_and_posn";
	case kanjidic::skip_misclass_t::stroke_diff: return "stroke_diff";
	}
}
const char* to_string(kanjidic::reading_type_t v) noexcept {
	switch (v) {
	case kanjidic::reading_type_t::pinyin: return "pinyin";
	case kanjidic::reading_type_t::korean_r: return "korean_r";
	case kanjidic::reading_type_t::korean_h: return "korean_h";
	case kanjidic::reading_type_t::vietnam: return "vietnam";
	case kanjidic::reading_type_t::ja_on: return "ja_on";
	case kanjidic::reading_type_t::ja_kun: return "ja_kun";
	}
}
const char* to_string(kanjidic::on_type_t v) noexcept {
	switch (v) {
	case kanjidic::on_type_t::none: return "none";
	case kanjidic::on_type_t::kan: return "kan";
	case kanjidic::on_type_t::go: return "go";
	case kanjidic::on_type_t::tou: return "tou";
	case kanjidic::on_type_t::kanyou: return "kan'you";
	}
}

} // namespace jdict
