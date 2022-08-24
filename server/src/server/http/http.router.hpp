#pragma once

#include "./http.hpp"

#include <bitset>

namespace http {

class router {
public:
	using method_mask = std::bitset<32>;
	struct entry {
		method_mask           methods;
		std::string           path;
		http::request_handler handler;
	};
	std::vector<entry> entries;

	router& handle(method_mask m, std::string path, http::request_handler&& handler) noexcept;
	router& handle(std::initializer_list<http::method> m, std::string path, http::request_handler&& handler) noexcept;
	router& handle(http::method m, std::string path, http::request_handler&& handler) noexcept;

	router& get    (std::string path, http::request_handler&& handler) noexcept;
	router& head   (std::string path, http::request_handler&& handler) noexcept;
	router& post   (std::string path, http::request_handler&& handler) noexcept;
	router& put    (std::string path, http::request_handler&& handler) noexcept;
	router& del    (std::string path, http::request_handler&& handler) noexcept;
	router& connect(std::string path, http::request_handler&& handler) noexcept;
	router& options(std::string path, http::request_handler&& handler) noexcept;
	router& trace  (std::string path, http::request_handler&& handler) noexcept;
	router& patch  (std::string path, http::request_handler&& handler) noexcept;

	router& files(std::string path, std::string directory) noexcept;

	bool operator()(http::request& req, http::response& res) noexcept;

	static bool match_route(std::string_view route, std::string_view request_path, value_map* routeParamsOut) noexcept;
};

struct static_files {
	std::string path_prefix, directory;
	static_files(std::string_view path_prefix, std::string_view directory) : path_prefix(path_prefix), directory(directory) {}
	static_files() = default;
	bool operator()(request& req, response& res);
};

bool serve_static_files(
	std::string path_prefix,
	std::string directory,
	request& req, response& res);

} // namespace http
