#pragma once

#include "./http.router.hpp"

namespace http {

std::string generate_swagger_json(router const& r) noexcept;
std::string generate_swagger_html(std::string swaggerJsonURL) noexcept;

void serve_swagger(router& r, std::string swaggerJsonRoute, std::string swaggerUiRoute) noexcept;

} // namespace http
