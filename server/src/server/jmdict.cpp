#include "./jmdict.hpp"

#include "./kana_util.hpp"
#include "./timer.hpp"

#include <rapidxml.hpp>
#include <rapidxml_iterators.hpp>

#include <cstdlib>
#include <ratio>
#include <stdexcept>
#include <cstdio>
#include <string_view>


using namespace rapidxml;
using namespace std::string_view_literals;

using jdict::debug::timer;

namespace jdict {
namespace {

static std::vector<char> readZip(const char* path) {
	throw std::runtime_error("Loading zip files is not implemented");
}

static std::vector<char> readTextFile(const char* path) {
	timer _("reading jmdict.xml");

	auto* file = fopen(path, "rb");
	if(!file)
		throw std::runtime_error("Failed opening file '" + std::string(path) + "'");

	fseek(file, 0, SEEK_END);
	unsigned size = ftell(file);
	fseek(file, 0, SEEK_SET);

	auto data = std::vector<char>(size + 1);
	data.back() = '\0';
	fread(data.data(), size, 1, file);

	fclose(file);

	return data;
}


struct children {
	xml_node<char>* node;
	children(xml_node<char>* node) noexcept : node(node) {}
	children(xml_node<char>& node) noexcept : node(&node) {}
	using iterator = node_iterator<char>;
	iterator begin() { return { node }; }
	iterator end()   { return {}; }
};
static inline std::string value(xml_node<char>& n) {
	return std::string(n.value(), n.value_size());
}
static inline std::string value(xml_attribute<char>& n) {
	return std::string(n.value(), n.value_size());
}
static inline auto value(xml_attribute<char>* n) {
	assert(n);
	return value(*n);
}
static inline auto value_or_empty(xml_attribute<char>* n) {
	if(!n) return std::string{};
	return value(*n);
}

#define UNHANDLED_NODE(PARENT, N) throw std::runtime_error("Unhandled element in <" PARENT ">: " + std::string((N).name()));

static jmdict::kanji parseKanjiElement(xml_node<char>& node) {
	auto result = jmdict::kanji();
	for(auto& child : children(node)) {
		if	 (child.name() == "keb"sv)	result.value = value(child);
		else if(child.name() == "ke_inf"sv) result.infos.push_back(value(child));
		else if(child.name() == "ke_pri"sv) result.priorities.push_back(value(child));
		else UNHANDLED_NODE("k_ele", child);
	}
	return result;
}

static jmdict::reading parseReadingElement(xml_node<char>& node) {
	auto result = jmdict::reading();
	for(auto& child : children(node)) {
		if	 (child.name() == "reb"sv)		result.value = value(child);
		else if(child.name() == "re_nokanji"sv) result.not_actual_reading = true;
		else if(child.name() == "re_restr"sv)   result.restrict_kanji.push_back(value(child));
		else if(child.name() == "re_inf"sv)	 result.infos.push_back(value(child));
		else if(child.name() == "re_pri"sv)	 result.priorities.push_back(value(child));
		else UNHANDLED_NODE("r_ele", child);
	}
	return result;
}

static jmdict::sense::gloss parseGlossElement(xml_node<char>& node) {
	return jmdict::sense::gloss {
		.content   = value(node),
		.language  = value_or_empty(node.first_attribute("xml:lang")),
		.gender	= value_or_empty(node.first_attribute("g_type")),
		.highlight = node.first_node("pri") != nullptr,
	};
}

static jmdict::sense::example parseExampleElement(xml_node<char>& node) {
	auto result = jmdict::sense::example();
	for(auto& child : children(node)) {
		if	 (child.name() == "ex_srce"sv) result.source = value(child);
		else if(child.name() == "ex_text"sv) result.form_in_example = value(child);
		else if(child.name() == "ex_sent"sv) {
			result.sentences.push_back(jmdict::sense::example::sentence {
				.language = value_or_empty(child.first_attribute("xml:lang")),
				.value = value(child),
			});
		}
		else UNHANDLED_NODE("example", child);
	}
	return result;
}

static jmdict::sense::source_language parseSourceLanguageElement(xml_node<char>& node) {
	return jmdict::sense::source_language {
		.word = value(node),
		.language = value_or_empty(node.first_attribute("xml:lang")),
		.partial = "part"sv == value_or_empty(node.first_attribute("ls_type")),
		.waseieigo = node.first_attribute("ls_wasei") != nullptr
	};
}

static jmdict::sense parseSenseElement(xml_node<char>& node) {
	auto result = jmdict::sense();
	for(auto& child : children(node)) {
		if	 (child.name() == "stagk"sv)   result.restrict_kanji  .push_back(value(child));
		else if(child.name() == "stagr"sv)   result.restrict_reading.push_back(value(child));
		else if(child.name() == "pos"sv)	 result.part_of_speech_tags  .push_back(value(child));
		else if(child.name() == "xref"sv)	result.cross_references	 .push_back(value(child));
		else if(child.name() == "ant"sv)	 result.antonyms			 .push_back(value(child));
		else if(child.name() == "field"sv)   result.fields .push_back(value(child));
		else if(child.name() == "misc"sv)	result.misc_info   .push_back(value(child));
		else if(child.name() == "s_inf"sv)   result.sense_info	.push_back(value(child));
		else if(child.name() == "lsource"sv) result.lang_origin	   .push_back(parseSourceLanguageElement(child));
		else if(child.name() == "dial"sv)	result.dialects			 .push_back(value(child));
		else if(child.name() == "gloss"sv)   result.glosses			  .push_back(parseGlossElement(child));
		else if(child.name() == "example"sv) result.examples			 .push_back(parseExampleElement(child));
		else UNHANDLED_NODE("r_ele", child);
	}
	return result;
}

static jmdict::entry parseEntry(xml_node<char>& node) {
	auto result = jmdict::entry();

	for(auto& n : children(node)) {
		if	 (n.name() == "ent_seq"sv) result.sequence = value(n);
		else if(n.name() == "k_ele"sv) result.kanji.push_back(parseKanjiElement(n));
		else if(n.name() == "r_ele"sv) result.readings.push_back(parseReadingElement(n));
		else if(n.name() == "sense"sv) result.senses.push_back(parseSenseElement(n));
		else UNHANDLED_NODE("entry", n);
	}

	return result;
}

} // namespace

jmdict jmdict::parse_file(const char* path) {
	std::vector<char> text;
	if(std::string_view(path).ends_with(".zip"))
		text = readZip(path);
	else
		text = readTextFile(path);

	constexpr auto PARSE_FLAGS = parse_trim_whitespace | parse_normalize_whitespace | parse_no_data_nodes;

	auto result = jmdict();

	{
		timer _("parsing jmdict.xml");
		xml_document<char> doc;
		{
			timer __("building DOM from jdmict xml");
			doc.parse<PARSE_FLAGS>(text.data());
		}

		auto root = doc.first_node();
		if(!root)
			throw std::runtime_error("Expected 'JMdict' as a root node, got NULL");
		if(root->name() != "JMdict"sv)
			throw std::runtime_error("Expected 'JMdict' as a root node, got " + std::string(root->name()));

		for(auto& entry : children(root)) {
			if(entry.name() == "entry"sv) result.entries.push_back(parseEntry(entry));
			else UNHANDLED_NODE("JMdict", entry);
		}
	}

	return result;
}

void jmdict::generate_romaji() {
	timer _("generating romaji");
	for(auto& e : entries) {
		for(auto& r : e.readings) {
			r.romaji = to_romaji(r.value);
		}
	}
}

} // namespace jdict
