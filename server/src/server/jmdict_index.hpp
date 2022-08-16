#pragma once

#include "./jmdict.hpp"

#include <functional>
#include <string_view>
#include <map>

namespace jdict {

class jmdict_index {
	jmdict const* const dict;
	std::map<std::string_view, jmdict::entry const*, std::less<>> bySequenceNumber;
	std::map<std::string, jmdict::entry const*, std::less<>> byReading;

public:
	jmdict_index(jmdict const& dict);
	std::vector<jmdict::entry const*> search(std::string_view query) const;
};

} // namespace jdict
