#pragma once

#include "./JMDict.hpp"
#include <functional>
#include <map>
#include <vcruntime.h>

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