#pragma once

#include "./http.hpp"

namespace jdict::http {

inline bool serve_static_files(
	std::string_view path_prefix,
	std::string_view directory,
	request& req, response& res)
{
	if(!req.location.path.starts_with(path_prefix))
		return false;

	std::string_view newPath = std::string_view(req.location.path).substr(path_prefix.size());
	if(newPath == "/") {
		newPath = "/index.html";
	}
	res.send_file(std::string(directory) + std::string(newPath));
	return true;
}

} // namespace jdict::http
