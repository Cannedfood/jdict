#include "./http.router.hpp"
#include "http.hpp"
#include "http.swagger.hpp"

#include <cassert>
#include <cstdio>

namespace http {

router& router::handle(method_mask m, std::string path, http::request_handler&& handler) noexcept {
	assert(handler);
	entries.push_back(entry { .methods = std::move(m), .path = std::move(path), .handler = std::move(handler), });
	return *this;
}

router& router::handle(std::initializer_list<http::method> m, std::string path, http::request_handler&& handler) noexcept {
	method_mask mask;
	for(auto& mm : m)
		mask.set(mm);
	return handle(mask, std::move(path), std::move(handler));
}

router& router::handle(http::method m, std::string path, http::request_handler&& handler) noexcept { return handle(std::initializer_list<http::method>{m}, std::move(path), std::move(handler)); }

router& router::get    (std::string path, http::request_handler&& handler) noexcept { return handle(http::Get,     std::move(path), std::move(handler)); }
router& router::head   (std::string path, http::request_handler&& handler) noexcept { return handle(http::Head,    std::move(path), std::move(handler)); }
router& router::post   (std::string path, http::request_handler&& handler) noexcept { return handle(http::Post,    std::move(path), std::move(handler)); }
router& router::put    (std::string path, http::request_handler&& handler) noexcept { return handle(http::Put,     std::move(path), std::move(handler)); }
router& router::del    (std::string path, http::request_handler&& handler) noexcept { return handle(http::Delete,  std::move(path), std::move(handler)); }
router& router::connect(std::string path, http::request_handler&& handler) noexcept { return handle(http::Connect, std::move(path), std::move(handler)); }
router& router::options(std::string path, http::request_handler&& handler) noexcept { return handle(http::Options, std::move(path), std::move(handler)); }
router& router::trace  (std::string path, http::request_handler&& handler) noexcept { return handle(http::Trace,   std::move(path), std::move(handler)); }
router& router::patch  (std::string path, http::request_handler&& handler) noexcept { return handle(http::Patch,   std::move(path), std::move(handler)); }

router& router::files(std::string path, std::string directory) noexcept { return get(path, static_files(path, directory)); }

bool router::operator()(http::request& req, http::response& res) noexcept {
	// printf("Matching: %s %.*s\n", to_string(req.method), (int) req.path.size(), req.path.data());
	for(auto& e : entries) {
		if(!e.methods.test(req.method)) continue;

		if(match_route(e.path, req.path, &req.route)) {
			e.handler(req, res);
			return true;
		}
	}
	// printf("Couldn't match request.\n");
	return false;
}

bool router::match_route(std::string_view route, std::string_view request_path, value_map* route_params_out) noexcept {
	auto snip_until = [](char c, std::string_view& s) noexcept -> std::string_view {
		auto result = s.substr(0, s.find(c));
		s.remove_prefix(result.size());
		return result;
	};
	auto left_trim = [](char c, std::string_view& s) noexcept -> int {
		int n = 0;
		while(s.starts_with(c)) { s.remove_prefix(1); n++; }
		return n;
	};
	auto right_trim = [](char c, std::string_view& s) noexcept -> int {
		int n = 0;
		while(s.ends_with(c)) { s.remove_suffix(1); n++; }
		return n;
	};

	route_params_out->entries.clear();

	right_trim('/', route);
	right_trim('/', request_path);

	while(!route.empty()) {
		if(route == "**") {
			return true;
		}
		else if(route.starts_with("*")) {
			route.remove_prefix(1);
			snip_until('/', request_path);
		}
		else if(route.starts_with(":")) {
			route.remove_prefix(1);
			if(request_path.empty())
				return false;
			auto name = snip_until('/', route);
			auto value = snip_until('/', request_path);
			route_params_out->entries.emplace(name, value);
		}
		else {
			auto fragment = snip_until('/', route);
			if(!request_path.starts_with(fragment))
				return false;
			request_path.remove_prefix(fragment.size());
		}
		left_trim('/', request_path);
		left_trim('/', route);
	}

	return request_path.empty();
}

bool static_files::operator()(request& req, response& res) {
	if(!req.path.starts_with(path_prefix))
		return false;

	std::string_view newPath = std::string_view(req.path).substr(path_prefix.size());
	if(newPath == "/")
		newPath = "/index.html";
	res.send_file(std::string(directory) + std::string(newPath));
	return true;
}

} // namespace http
