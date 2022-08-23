#include "./jmdict.hpp"
#include "./jmdict_index.hpp"
#include "./jmdict_json.hpp"

#include "./http/http.router.hpp"

#include "./util/timer.hpp"
#include "./util/cache.hpp"

#include <algorithm>
#include <chrono>
#include <cstdlib>
#include <exception>
#include <span>
#include <stdio.h>
#include <set>
#include <vector>
#include <future>

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
	std::string distDir = "../dist";
	std::string jdictXML = "JMdict.xml";
	int port = 8080;
	if(auto* c = std::getenv("JDICT_DIST_DIR")) distDir = c;
	if(auto* c = std::getenv("JDICT_XML")) jdictXML = c;
	if(auto* c = std::getenv("JDICT_PORT")) port = atoi(c);

	auto dict  = jmdict();
	auto index = jmdict_index();
	auto cache = jdict::cache<std::string, std::vector<jmdict::entry const*>>(1024);

	auto dictionary_loaded = std::async(std::launch::async, [&] {
		dict = jmdict::parse_file(jdictXML.c_str());
		dict.generate_romaji();
		index = jmdict_index(dict);
		printf("Loaded %zu dictionary entries.\n", dict.entries.size());
	});

	auto router = http::router();
	router.get("/api/search", [&](http::request& req, http::response& res) {
		auto searchTerm = req.query.get("searchTerm");
		auto start      = req.query.get("skip", 0);
		auto limit      = req.query.get("take", 50);

		if(searchTerm.empty()) {
			res.status(http::BadRequest, "The query must include a non-empty searchTerm parameter").send();
			return;
		}

		if(dictionary_loaded.wait_for(std::chrono::seconds(2)) == std::future_status::timeout) {
			res.status(http::ServiceUnavailable, "Service Unavailable - dictionary not loaded yet");
			return;
		}

		auto timeStart = std::chrono::high_resolution_clock::now();

		auto allResults = cache.get_or_create(std::string(searchTerm), [&] {
			return index.search(searchTerm);
		});
		auto pagedEntries = applyPaging(start, limit, allResults);

		nlohmann::json responseBody;
		responseBody["resultsTotal"] = allResults.size();
		responseBody["results"] = to_json(pagedEntries);

		auto timeEnd = std::chrono::high_resolution_clock::now();
		responseBody["time"] = debug::to_string(timeEnd - timeStart);

		res.send(http::mimetype_from_filending(".json"), to_string(responseBody));
	});
	// router.get("/api/stats", [&](http::request& req, http::response& res) {
	// 	if(dictionary_loaded.wait_for(std::chrono::seconds(2)) == std::future_status::timeout) {
	// 		res.status(http::ServiceUnavailable, "Service Unavailable - dictionary not loaded yet");
	// 		return;
	// 	}
	// 	nlohmann::json responseBody;
	// 	{
	// 		nlohmann::json indices;
	// 		for(auto const* idx : index.)

	// 		responseBody["indices"] = indices;
	// 	}
	// 	res.send(http::mimetype_from_filending(".json"), to_string(responseBody));
	// });
	router.files("/**", "../dist/");

	printf("\nStart listening at http://localhost:%i\n", port);
	http::listen(port, router);
	return EXIT_SUCCESS;
}
