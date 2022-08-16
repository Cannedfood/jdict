#include "./jmdict.hpp"

#include <nlohmann/json.hpp>
#include <type_traits>

namespace jdict {

nlohmann::json to_json(jmdict::sense::example::sentence const&);

nlohmann::json to_json(jmdict::sense::example const&);
nlohmann::json to_json(jmdict::sense::gloss const&);
nlohmann::json to_json(jmdict::sense::source_language const&);

nlohmann::json to_json(jmdict::kanji const&);
nlohmann::json to_json(jmdict::reading const&);
nlohmann::json to_json(jmdict::sense const&);
nlohmann::json to_json(jmdict::entry const&);

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