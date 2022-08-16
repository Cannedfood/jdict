#include "./jmdict.hpp"
#include "./jmdict_index.hpp"
#include "./jmdict_json.hpp"

#include "./http/http.hpp"
#include "./http/http.staticfiles.hpp"

#include "./timer.hpp"

#include <algorithm>
#include <cstdlib>
#include <exception>
#include <span>
#include <stdio.h>
#include <set>
#include <vector>

using namespace jdict;

template<class T>
std::vector<T> applyPaging(int page, int pageSize, std::vector<T> const& v) {
    unsigned start = page * pageSize;
    unsigned end   = start + pageSize;

    if(start == 0 && end >= v.size()) return v;

    std::vector<T> result;
    result.reserve(pageSize);
    for(size_t i = start; i < end && i < v.size(); i++) {
        result.push_back(v[i]);
    }
    return result;
}

int main(int argc, char** argv) {
    auto const dict  = jmdict::parse_file("JMdict.xml");
    auto const index = jmdict_index(dict);

    printf("Loaded %zu dictionary entries.\n", dict.entries.size());
    printf("\nStart listening at http://localhost:8080\n");
    http::listen(8080, [&](http::request& req, http::response& res) {
        if(req.method != http::Get) {
            res.status(http::MethodNotAllowed, "Only GET requests are accepted").send();
            return;
        }

        printf("%s %s\n", to_string(req.method), req.location.path.c_str());

        if(req.location.path.starts_with("/api/search")) {
            auto searchTerm = req.location.query["searchTerm"];
            auto pageSize   = req.location.try_get_param("pageSize", 200);
            auto page       = req.location.try_get_param("page", 0);

            if(searchTerm.empty()) {
                res.status(http::BadRequest, "The query must include a non-empty searchTerm parameter").send();
                return;
            }

            // TODO: cache index search result, so paging is more performant
            auto pagedEntries = applyPaging(page, pageSize, index.search(searchTerm));

            res.send(http::mimetype_from_path(".json"), to_string(to_json(pagedEntries)));
        }
        else if(http::serve_static_files("", "../dist/", req, res))
            return;
    });
    return EXIT_SUCCESS;
}