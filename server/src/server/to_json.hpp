#include "./database/jmdict/jmdict.hpp"
#include "./database/kanjidic/kanjidic.hpp"

#include <nlohmann/json.hpp>
#include <type_traits>
#include <utility>

namespace jdict {

nlohmann::json to_json(jmdict::sense_t::example::sentence const&);

nlohmann::json to_json(jmdict::sense_t::example const&);
nlohmann::json to_json(jmdict::sense_t::gloss const&);
nlohmann::json to_json(jmdict::sense_t::source_language const&);

nlohmann::json to_json(jmdict::kanji_t const&);
nlohmann::json to_json(jmdict::reading_t const&);
nlohmann::json to_json(jmdict::sense_t const&);
nlohmann::json to_json(jmdict::entry_t const&);

nlohmann::json to_json(std::pair<jmdict::entry_t const*, int> const&);


nlohmann::json to_json(kanjidic::query_code_t const&);
nlohmann::json to_json(kanjidic::radical_t    const&);
nlohmann::json to_json(kanjidic::misc_t       const&);
nlohmann::json to_json(kanjidic::variant_t  const&);
nlohmann::json to_json(kanjidic::codepoint_t  const&);
nlohmann::json to_json(kanjidic::character_t  const&);
nlohmann::json to_json(kanjidic::rm_group_t  const&);
nlohmann::json to_json(kanjidic::reading_t  const&);
nlohmann::json to_json(kanjidic::meaning_t  const&);


template<class T>
nlohmann::json to_json(std::vector<T> const& j) {
	auto result = nlohmann::json::value_type::array();
	if constexpr(std::is_pointer_v<T>)
		for(auto& e : j) result.push_back(to_json(*e));
	if constexpr(!std::is_pointer_v<T>)
		for(auto& e : j) result.push_back(to_json(e));
	return result;
}

} // namespace jdict
