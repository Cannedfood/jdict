#include "./jmdict.hpp"
#include "./jmdict_index.hpp"
#include "./jmdict_json.hpp"

#include "./http/http.hpp"
#include "./http/http.staticfiles.hpp"

#include "./timer.hpp"
#include "cache.hpp"

#include <algorithm>
#include <cstdlib>
#include <exception>
#include <span>
#include <stdio.h>
#include <set>
#include <vector>

using namespace jdict;

template<class T>
std::vector<T> applyPaging(int skip, int take, std::vector<T> const& v) {
	int start = skip;
	int end = start + take;

	if(start == 0 && end >= v.size()) return v;

	std::vector<T> result;
	result.reserve(take);
	for(size_t i = start; i < end && i < v.size(); i++) {
		result.push_back(v[i]);
	}
	return result;
}

int main(int argc, char** argv) {
	auto const dict  = jmdict::parse_file("JMdict.xml");
	auto const index = jmdict_index(dict);
	auto       cache = jdict::cache<std::string, std::vector<jmdict::entry const*>>(1024);

	printf("Loaded %zu dictionary entries.\n", dict.entries.size());
	printf("\nStart listening at http://localhost:8080\n");
	http::listen(8080, [&](http::request& req, http::response& res) {
		debug::timer _("handling request");
		if(req.method != http::Get) {
			res.status(http::MethodNotAllowed, "Only GET requests are accepted").send();
			return;
		}

		printf("%s %s\n", to_string(req.method), req.location.path.c_str());

		if(req.location.path.starts_with("/api/search")) {
			auto& searchTerm = req.location.query["searchTerm"];
			auto  start = req.location.try_get_param("skip", 0);
			auto  limit = req.location.try_get_param("take", 50);

			if(searchTerm.empty()) {
				res.status(http::BadRequest, "The query must include a non-empty searchTerm parameter").send();
				return;
			}

			auto timeStart = std::chrono::high_resolution_clock::now();

			auto allResults = cache.get_or_create(searchTerm, [&]() {
				return index.search(searchTerm);
			});
			auto pagedEntries = applyPaging(start, limit, allResults);

			nlohmann::json responseBody;
			responseBody["resultsTotal"] = allResults.size();
			responseBody["results"] = to_json(pagedEntries);

			auto timeEnd = std::chrono::high_resolution_clock::now();
			responseBody["time"] = std::to_string(duration_cast<std::chrono::milliseconds>(timeEnd - timeStart).count()) + "ms";

			res.send(http::mimetype_from_path(".json"), to_string(responseBody));
		}
		else if(http::serve_static_files("", "../dist/", req, res))
			return;
	});
	return EXIT_SUCCESS;
}
