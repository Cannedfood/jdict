#pragma once

#include "./jmdict.hpp"

#include <functional>
#include <string_view>
#include <map>

namespace jdict {

class jmdict_index {
public:
	jmdict_index(jmdict const& dict);
	std::vector<jmdict::entry const*> search(std::string_view query) const;
private:
	using TextIndex     = std::map<std::string, jmdict::entry const*, std::less<>>;
	using TextViewIndex = std::map<std::string_view, jmdict::entry const*, std::less<>>;
	using ResultWeights = std::map<jmdict::entry const*, unsigned>;

	jmdict const* const dict;
	TextViewIndex bySequenceNumber;
	TextIndex     byReading;
	TextViewIndex byTranslation;

	void findBySequenceNumber(ResultWeights& results_out, int baseWeight, std::string_view query) const;
	void findByReading       (ResultWeights& results_out, int baseWeight, std::string_view query) const;
	void findByTranslation   (ResultWeights& results_out, int baseWeight, std::string_view query) const;

	static std::vector<jmdict::entry const*>  sortResults(ResultWeights&& weights);
};

} // namespace jdict
