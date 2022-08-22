#include "./http.swagger.hpp"
#include "http.hpp"

namespace http {

std::string generate_swagger_json(router const& r) noexcept {
	return R"({
		"openapi": "3.0.3",
		"info": {
			"title": "api",
			"description": "description",
			"termsOfService": "",
			"contact": {
				"name": "Benno Straub",
				"url": "www.github.com/Cannedfood",
				"email": "benno.straub@outlook.de"
			},
			"license": {
				"name": "Apache 2.0",
				"url": "https://www.apache.org/licenses/LICENSE-2.0.html"
			},
			"version": "0.1.0"
		},
		"servers": [],
	})";
}
std::string generate_swagger_html(std::string swaggerJsonURL) noexcept {
	return R"(
		<!DOCTYPE html>
		<html lang="en">
		<head>
			<meta charset="utf-8" />
			<meta name="viewport" content="width=device-width, initial-scale=1" />
			<meta name="description" content="SwaggerUI" />
			<title>SwaggerUI</title>
			<link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css" />
		</head>
		<body>
			<div id="swagger-ui"></div>
			<script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js" crossorigin></script>
			<script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-standalone-preset.js" crossorigin></script>
			<script>
				window.onload = () => {
					window.ui = SwaggerUIBundle({
						url: ')"+swaggerJsonURL+R"(',
						dom_id: '#swagger-ui',
						presets: [SwaggerUIBundle.presets.apis,SwaggerUIStandalonePreset],layout: "StandaloneLayout",
					});
				};
			</script>
		</body>
		</html>
	)";
}

void serve_swagger(router& r, std::string swaggerJsonRoute, std::string swaggerUiRoute) noexcept {
	r.get(swaggerJsonRoute, [=](request& req, response& res) {
		res.send(mimetype_from_filending(".html"), generate_swagger_json(r));
	});
	r.get(swaggerUiRoute, [=](request& req, response& res) {
		res.send(mimetype_from_filending(".html"), generate_swagger_html(swaggerJsonRoute));
	});
}

} // namespace http
