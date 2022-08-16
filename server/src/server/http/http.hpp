#pragma once

#include <functional>
#include <string>
#include <string_view>
#include <span>
#include <map>

namespace jdict::http {

enum method {
	Get, Head, Post, Put, Delete, Connect, Options, Trace, Patch
};
const char* to_string(method);

enum response_code {
	Continue = 100,
	Switching_Protocols = 101,
	Processing = 102,
	EarlyHints = 103,

	Ok = 200,
	Created = 201,
	Accepted = 202,
	NonAuthoritativeInformation = 203,
	NoContent = 204,
	ResetContent = 205,
	PartialContent = 206,
	MultiStatus = 207,
	AlreadyReported = 208,
	ImUsed = 226,

	MultipleChoices = 300,
	MovedPermanently = 301,
	Found = 302,
	SeeOther = 303,
	NotModified = 304,
	UseProxy = 305,
	TemporaryRedirect = 307,
	PermanentRedirect = 308,

	BadRequest = 400,
	Unauthorized = 401,
	PaymentRequired = 402,
	Forbidden = 403,
	NotFound = 404,
	MethodNotAllowed = 405,

	InternalServerError = 500,
};

struct location {
	std::string path;
	std::map<std::string, std::string, std::less<>> query;

	location() = default;
	location(std::string_view sv);

	std::string_view try_get_param(std::string_view name, std::string_view fallback = {}) const;
	int			  try_get_param(std::string_view name, int			  fallback) const;
	double		   try_get_param(std::string_view name, double		   fallback) const;
};

struct request {
	const unsigned handle;

	std::string requestText;

	method method;
	location location;
	std::multimap<std::string_view, std::string_view, std::less<>> headers;

	unsigned content_length() const;
	std::string read_content(unsigned length = 0);

	request(unsigned handle) : handle(handle) {}
	request(request&&) = delete;
	request(request const&) = delete;
};

struct response {
	const unsigned handle;
	enum class Stage { Status, Headers, Sent } stage = Stage::Status;

	response& status(int status, std::string_view message);

	response& header(std::string_view name, std::string_view value);

	void send();
	void send(std::string_view mimeType, std::span<char const> data);
	void send(std::string_view mimeType, std::string_view	  data) { send(mimeType, std::span<char const>(data.data(), data.size())); }
	void send(std::string_view mimeType, std::string const&	data) { send(mimeType, std::string_view(data)); }
	void send(std::string_view mimeType, const char*		   data) { send(mimeType, std::string_view(data)); }
	void send_file(std::string const& path, std::string_view mimeType = {});

	response(unsigned handle) : handle(handle) {}
	response(response&&) = delete;
	response(response const&) = delete;
private:
	void end_headers() { write_raw("\r\n"); }
	void start_content(std::string_view mimeType, unsigned dataSize);
	void write_raw(std::span<char const> data);
	void write_raw(std::string_view	  data) { write_raw(std::span<char const>(data.data(), data.size())); }
	void write_raw(std::string const&	data) { write_raw(std::string_view(data)); }
	void write_raw(const char*		   data) { write_raw(std::string_view(data)); }
};

using request_handler = std::function<void(request&, response&)>;

std::string_view mimetype_from_path(std::string_view filename) noexcept;

[[noreturn]]
void listen(int port, request_handler const& handler);

} // namespace jdict::http
