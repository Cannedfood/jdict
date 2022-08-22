#pragma once

#include <functional>
#include <initializer_list>
#include <string>
#include <string_view>
#include <span>
#include <map>
#include <type_traits>

#define HTTP_HANDLER [&](http::request& req, http::response& res) -> void

namespace http {

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
	NotAcceptable = 406,
	ProxyAuthenticationRequired = 407,
	RequestTimeout = 408,
	Conflict = 409,
	Gone = 410,
	LengthRequired = 411,
	PreconditionFailed = 412,
	PayloadTooLarge = 413,
	URITooLong = 414,
	UnsupportedMediaType = 415,
	RangeNotSatisfiable = 416,
	ExpectationFailed = 417,
	ImATeapot = 418,

	InternalServerError = 500,
	NotImplemented = 501,
	BadGateway = 502,
	ServiceUnavailable = 503,
	GatewayTimeout = 504,
	HttpVersionNotSupported = 505,
	NotExtended = 510,
	NetworkAuthenticationRequired = 511
};


struct value_map {
	std::map<std::string, std::string, std::less<>> entries;

	std::string_view get(std::string_view name, std::string_view fallback = {}) const;
	int			     get(std::string_view name, int			     fallback) const;
	double		     get(std::string_view name, double		     fallback) const;

	std::string_view operator[](std::string_view key) const { return entries.find(key)->second; }
};

struct request {
	const unsigned handle;

	std::string requestText;

	method      method;
	std::string url;
	std::string path;
	value_map   query;
	value_map   route;
	value_map   headers;

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
	void send(std::string_view mimeType, std::string_view	   data) { send(mimeType, std::span<char const>(data.data(), data.size())); }
	void send(std::string_view mimeType, std::string const&	   data) { send(mimeType, std::string_view(data)); }
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

using request_handler      = std::function<void(request&, response&)>; //!< Handles incoming http requests
using cond_request_handler = std::function<void(request&, response&)>; //!< Handles an incoming http request and returns true or returns false otherwise

std::string_view mimetype_from_filending(std::string_view filename) noexcept;

enum listen_flags {
	Default,
};
inline static listen_flags operator|(listen_flags a, listen_flags b) noexcept { return (listen_flags) ((int) a | (int) b); }
inline static listen_flags operator&(listen_flags a, listen_flags b) noexcept { return (listen_flags) ((int) a & (int) b); }
inline static listen_flags operator^(listen_flags a, listen_flags b) noexcept { return (listen_flags) ((int) a ^ (int) b); }

[[noreturn]]
int listen(int port, request_handler const& handler, listen_flags flags = Default);

} // namespace http
