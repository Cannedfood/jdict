#include "./database/jmdict/jmdict_index.hpp"
#include "./database/kanjidic/kanjidic_index.hpp"

#include "./to_json.hpp"

#include "./http/http.router.hpp"

#include "./util/timer.hpp"
#include "./util/cache.hpp"
#include "server/util/utf8.hpp"

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
	std::string distDir = "../dist/";
	std::string jdictXML = "JMdict.xml";
	std::string kanjidicXML = "kanjidic2.xml";
	int port = 8080;
	if(auto* c = std::getenv("JDICT_DIST_DIR")) distDir = c;
	if(auto* c = std::getenv("JDICT_JMDICT_XML")) jdictXML = c;
	if(auto* c = std::getenv("JDICT_KANJIDIC_XML")) kanjidicXML = c;
	if(auto* c = std::getenv("JDICT_PORT")) port = atoi(c);

	auto dict  = jmdict();
	auto dict_idx = jmdict_index();

	auto kanji = kanjidic();
	auto kanji_idx = kanjidic_index();

	auto cache = jdict::cache<std::string, jmdict_index::results_t>(1024);

	auto dictionary_loaded = std::async(std::launch::async, [&] {
		dict = jmdict::parse_file(jdictXML.c_str());
		dict.generate_romaji();

		kanji = kanjidic::parse_file(kanjidicXML.c_str());

		dict_idx = jmdict_index(dict);
		kanji_idx = kanjidic_index(kanji);

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
			return dict_idx.search(searchTerm);
		});
		auto pagedEntries = applyPaging(start, limit, allResults);

		nlohmann::json responseBody;
		responseBody["resultsTotal"] = allResults.size();
		responseBody["results"] = to_json(pagedEntries);
		responseBody["kanji"] = to_json(kanji_idx.search(searchTerm));

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
	router.files("/**", distDir);

	printf("\nStart listening at http://localhost:%i\n", port);
	http::listen(port, router);
	return EXIT_SUCCESS;
}
