#include "../server/http/http.tests.hpp"
#include "../server/util/utf8_sliding_window.hpp"
#include "../server/util/full_text_index.hpp"

#include <cstdio>
#include <cstring>
#include <cassert>

#define TEST(EXPR) \
	if(!(EXPR)) fprintf(stderr, "Test '%s' failed\n", #EXPR)

void test_utf8() {
	auto text = std::string_view((const char*) u8"a字じ");
	TEST('a'    == jdict::utf8::decode_and_snip(text));
	TEST(0x5b57 == jdict::utf8::decode_and_snip(text));
	TEST(0x3058 == jdict::utf8::decode_and_snip(text));
	TEST(text.empty());
}

void test_ngram() {
	auto strat = jdict::ngram_indexing_strategy {
		.n_alpha = 2,
		.n_kanji = 1,
		.n_kana = 2,
		.n_unicode = 1
	};

	std::vector<std::string_view> result;
	strat.get_fragments(
		(const char*) u8"abcd 4242.ひらがな漢字",
		[&](std::string_view s) {
			result.push_back(s);
			// printf("'%.*s'\n", (int) s.size(), s.data());
			assert(result.size() < 20);
		}
	);
	int i = 0;
	TEST(result.size() > i && result[i++] == (const char*) u8"ab");
	TEST(result.size() > i && result[i++] == (const char*) u8"bc");
	TEST(result.size() > i && result[i++] == (const char*) u8"cd");
	TEST(result.size() > i && result[i++] == (const char*) u8"ひら");
	TEST(result.size() > i && result[i++] == (const char*) u8"らが");
	TEST(result.size() > i && result[i++] == (const char*) u8"がな");
	TEST(result.size() > i && result[i++] == (const char*) u8"漢");
	TEST(result.size() > i && result[i++] == (const char*) u8"字");
	TEST(result.size() == i);
}

int main(int argc, char const* argv[]) {
	http::run_tests();
	test_utf8();
	test_ngram();
	return 0;
}
