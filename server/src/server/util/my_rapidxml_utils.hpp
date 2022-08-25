#pragma once

#include "rapidxml_iterators.hpp"
#include <rapidxml.hpp>
#include <rapidxml_utils.hpp>

namespace jdict {

using xml_node      = rapidxml::xml_node<char>;
using xml_attribute = rapidxml::xml_attribute<char>;
using xml_iterator  = rapidxml::node_iterator<char>;
using xml_document  = rapidxml::xml_document<char>;

struct children {
	xml_node* node;
	children(xml_node* node) noexcept : node(node) {}
	children(xml_node& node) noexcept : node(&node) {}
	using iterator = xml_iterator;
	iterator begin() { return { node }; }
	iterator end()   { return {}; }
	size_t count() const noexcept { return rapidxml::count_children(node); }
};
static inline std::string value(xml_node& n) {
	return std::string(n.value(), n.value_size());
}
static inline std::string value(xml_attribute& n) {
	return std::string(n.value(), n.value_size());
}
static inline auto value(xml_attribute* n) {
	assert(n);
	return value(*n);
}
static inline auto value_or_empty(xml_attribute* n) {
	if(!n) return std::string{};
	return value(*n);
}

#define UNHANDLED_NODE(PARENT, N) throw std::runtime_error("Unhandled element in <" PARENT ">: " + std::string((N).name()));

} // namespace jdict
