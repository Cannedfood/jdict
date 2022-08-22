#include "./http.router.hpp"
#include <map>

#define TEST(EXPR) \
	if(!(EXPR)) fprintf(stderr, "Test '%s' failed\n", #EXPR)

namespace http {

inline void run_tests() {
	//
	// Url parsing
	//
	// TODO

	//
	// Route matching
	//

	http::value_map params;
	// Basic matching
	TEST(http::router::match_route("/",    "/",     &params) && params.entries.empty());
	TEST(http::router::match_route("/abc", "/abc",  &params) && params.entries.empty());
	TEST(http::router::match_route("/abc", "/abc/", &params) && params.entries.empty());
	TEST(http::router::match_route("/abc/", "/abc/", &params) && params.entries.empty());
	TEST(http::router::match_route("/abc/", "/abc", &params) && params.entries.empty());

	TEST(!http::router::match_route("/abc", "/",        &params) && params.entries.empty());
	TEST(!http::router::match_route("/abc", "/abc/def", &params) && params.entries.empty());

	// Wildcards
	TEST(http::router::match_route("/*",         "/abc",         &params) && params.entries.empty());
	TEST(http::router::match_route("/*/",        "/abc",         &params) && params.entries.empty());
	TEST(http::router::match_route("/abc/*",     "/abc/def",     &params) && params.entries.empty());
	TEST(http::router::match_route("/abc/*/ghi", "/abc/def/ghi", &params) && params.entries.empty());

	TEST(!http::router::match_route("/abc/*/ghi", "/abc/def",     &params) && params.entries.empty());
	TEST(!http::router::match_route("/abc/*",     "/abc/def/ghi", &params) && params.entries.empty());

	// Parameters
	TEST(http::router::match_route("/:test",         "/abc",         &params) && params["test"] == "abc");
	TEST(http::router::match_route("/:test/",        "/abc",         &params) && params["test"] == "abc");
	TEST(http::router::match_route("/abc/:test",     "/abc/def",     &params) && params["test"] == "def");
	TEST(http::router::match_route("/abc/:test/ghi", "/abc/def/ghi", &params) && params["test"] == "def");

	TEST(!http::router::match_route("/abc/:test/ghi", "/abc/def",     &params) && params["test"] == "def");
	TEST(!http::router::match_route("/abc/:test",     "/abc/def/ghi", &params) && params["test"] == "def");

	// Catch all
	TEST(http::router::match_route("/**", "/abc", &params) && params.entries.empty());
	TEST(http::router::match_route("/abc/**", "/abc/def", &params) && params.entries.empty());
	TEST(http::router::match_route("/abc/**", "/abc/def/ghi", &params) && params.entries.empty());

	// Common usages/Corner cases
	TEST( http::router::match_route("/",       "/", &params) && params.entries.empty());
	TEST( http::router::match_route("/*",      "/", &params) && params.entries.empty());
	TEST( http::router::match_route("/**",     "/", &params) && params.entries.empty());
	TEST(!http::router::match_route("/:thing", "/", &params) && params.entries.empty());
}

} // namespace http

#undef TEST
