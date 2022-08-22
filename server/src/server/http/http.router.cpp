#include "./http.router.hpp"

#include <cassert>

namespace http {

router& router::handle(method_mask m, std::string path, http::request_handler&& handler) noexcept {
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

router& router::files(std::string path, std::string directory) noexcept {
    return get(path, static_files(path, directory));
}
router& router::swagger(std::string uiPath, std::string jsonPath) noexcept {
    if(jsonPath.empty())
        jsonPath = uiPath + ".json";

    get(jsonPath, [&](request& req, response& res) {
        res.send(mimetype_from_filending(".json"), swaggerJson());
    });
    get(uiPath, [jsonPath](request& req, response& res) {
        // TODO: minify html
        auto html = R"(
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
                            url: ')"+jsonPath+R"(',
                            dom_id: '#swagger-ui',
                            presets: [SwaggerUIBundle.presets.apis,SwaggerUIStandalonePreset],layout: "StandaloneLayout",
                        });
                    };
                </script>
            </body>
            </html>
        )";

        res.send(mimetype_from_filending(".html"), html);
    });
    return *this;
}

bool router::operator()(http::request& req, http::response& res) noexcept {
    for(auto& e : entries) {
        if(!e.methods.test(req.method)) continue;

        if(match_route(e.path, req.path, &req.route)) {
            e.handler(req, res);
            return true;
        }
    }
    return false;
}

bool router::match_route(std::string_view route, std::string_view request_path, value_map* route_params_out) noexcept {
    auto snip_until = [](char c, std::string_view& s) noexcept -> std::string_view {
        auto result = s.substr(0, s.find(c));
        s.remove_prefix(result.size());
        return result;
    };

    route_params_out->entries.clear();

    while(!route.empty()) {
        assert(route.front() == '/');
        route.remove_prefix(1);
        
        if(request_path.empty())
            return false;
        request_path.remove_prefix(1); // Remove leading slash

        if(route.starts_with("*")) {
            route.remove_prefix(1);
            snip_until('/', request_path);
        }
        else if(route.starts_with(":")) {
            route.remove_prefix(1);
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
        assert(route.empty() || route.starts_with('/'));
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