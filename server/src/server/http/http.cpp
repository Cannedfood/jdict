#include "./http.hpp"

#include <stdexcept>
#include <cstdio>
#include <string>
#include <cstring>
#include <cassert>
#include <fstream>
#include <array>

// Many thanks for https://stackoverflow.com/a/28031039/3841944

#ifdef _WIN32
	/* See http://stackoverflow.com/questions/12765743/getaddrinfo-on-win32 */
	#ifndef _WIN32_WINNT
		#define _WIN32_WINNT 0x0501  /* Windows XP. */
	#endif
	#include <winsock2.h>
	#include <Ws2tcpip.h>

	#define sockValid(S) ((S) != INVALID_SOCKET)
#else
	/* Assume that any non-Windows platform uses POSIX-style sockets instead. */
	#include <sys/socket.h>
	#include <arpa/inet.h>
	#include <netdb.h>  /* Needed for getaddrinfo() and freeaddrinfo() */
	#include <unistd.h> /* Needed for close() */

	using SOCKET = int;
	#define sockValid(S) ((S) >= 0)
#endif

namespace {

int sockInit(void) {
	#ifdef _WIN32
		WSADATA wsa_data;
		return WSAStartup(MAKEWORD(1,1), &wsa_data);
	#else
		return 0;
	#endif
}

int sockQuit(void) {
	#ifdef _WIN32
		return WSACleanup();
	#else
		return 0;
	#endif
}

int sockClose(SOCKET sock) {
	int status = 0;

	#ifdef _WIN32
		status = shutdown(sock, SD_BOTH);
		if (status == 0) { status = closesocket(sock); }
	#else
		status = shutdown(sock, SHUT_RDWR);
		if (status == 0) { status = close(sock); }
	#endif

	return status;
}

static inline std::string_view snip(std::string_view& s, int at, int extra = 0) {
	auto result = s.substr(0, at);
	s = s.substr(at + extra);
	return result;
}

static inline bool snipPrefix(std::string_view& s, std::string_view startswith) {
	if(s.starts_with(startswith)) {
		snip(s, startswith.size());
		return true;
	}
	return false;
}

static std::string_view trim_left(std::string_view s) {
	while(!s.empty() && s.front() <= ' ') s.remove_prefix(1);
	return s;
}

static std::string_view trim_right(std::string_view s) {
	while(!s.empty() && s.back() <= ' ') s.remove_prefix(1);
	return s;
}
static std::string_view trim(std::string_view s) { return trim_left(trim_right(s)); }

template<typename T>
static inline std::string_view snipUntil(std::string_view& s, T pred, int extra = 1) {
	for(size_t i = 0; i < s.size(); i++) {
		if(pred(s[i]))
			return snip(s, i, extra);
	}
	return snip(s, s.size());
}

static inline std::string_view snipUntil(std::string_view& s, char c, int extra = 1) {
	return snipUntil(s, [c](char cc) { return cc == c; }, extra);
}

static inline std::string_view snipToken(std::string_view& s) {
	auto result = snipUntil(s, [](char c) { return c <= ' '; });
	s = trim_left(s);
	return result;
}

static std::string_view snipLine(std::string_view& s) {
	for(size_t i = 0; i < s.size(); i++) {
		if(s.substr(i).starts_with("\r\n"))
			return snip(s, i, 2);
		if(s.substr(i).starts_with("\n"))
			return snip(s, i, 1);
	}
	return snip(s, s.size());
}

static std::string urlDecode(std::string_view src) {
	auto hexdigit = [](char c) {
		if(c >= 'a' && c <= 'z')
			return c - 'a' + 10;
		if(c >= 'A' && c <= 'Z')
			return c - 'A' + 10;
		if(c >= '0' && c <= '9')
			return c - '0';
		throw std::runtime_error("URL encoded text has non-hex character after % symbol");
	};

	std::string result;
	for(size_t i = 0; i < src.size(); i++) {
		if(src[i] == '%') {
			if(i + 2 >= src.size())
				throw std::runtime_error("URL encoded text ends unexpectedly");
			result.push_back(hexdigit(src[i+1]) << 4 | hexdigit(src[i+2]));
			i += 2;
		}
		else if(src[i] == '+')
			result.push_back(' ');
		else
			result.push_back(src[i]);
	}
	return result;
}

static void replace(std::string& s, std::string_view pattern, std::string_view with) {
	while(true) {
		auto pos = s.find(pattern);
		if(pos == std::string_view::npos)
			break;
		s.replace(pos, pattern.size(), "/");
	}
}

template<class Callback>
struct scoped {
	std::remove_cvref_t<Callback> cb;
	scoped(Callback&& cb) : cb(std::forward<Callback>(cb)) {}
	~scoped() { cb(); }
};

} // namespace

namespace jdict::http {

location::location(std::string_view s) {
	this->path = urlDecode(snipUntil(s, [](char c) { return c == '#' || c == '?'; }, 0));
	replace(this->path, "./", "/");
	replace(this->path, ".\\", "\\");

	if(s.starts_with('?')) {
		do {
			s.remove_prefix(1);
			auto query = snipUntil(s, [](char c) { return c == '&' || c == '#'; }, 0);

			auto name  = urlDecode(trim(snipUntil(query, '=')));
			auto value = urlDecode(trim(query));
			this->query.emplace(std::move(name), std::move(value));
		} while(s.starts_with('&'));
	}
}

std::string_view location::try_get_param(std::string_view name, std::string_view fallback) const {
	auto iter = this->query.find(name);
	if(iter != this->query.end())
		return iter->second;
	return fallback;
}
int location::try_get_param(std::string_view name, int			  fallback) const {
	auto iter = this->query.find(name);
	if(iter != this->query.end())
		return std::stoi(iter->second);
	return fallback;
}
double location::try_get_param(std::string_view name, double		   fallback) const {
	auto iter = this->query.find(name);
	if(iter != this->query.end())
		return std::stod(iter->second);
	return fallback;
}

unsigned request::content_length() const {
	auto [first, end] = headers.equal_range("Content-Length");
	if(first == end)
		return {};
	return std::stoul(std::string(first->second));
}

std::string request::read_content(unsigned length) {
	if(length == 0)
		length = content_length();

	auto result = std::string(length, ' ');
	if (::recv(handle, result.data(), length, 0) != length) {
		throw std::runtime_error("Failed reading from client socket");
	}
	return result;
}


response& response::status(int status, std::string_view message) {
	assert(stage == Stage::Status);
	write_raw("HTTP/1.1 ");
	write_raw(std::to_string(status));
	write_raw(" ");
	write_raw(message);
	write_raw("\r\n");
	stage = Stage::Headers;
	return *this;
}
response& response::header(std::string_view name, std::string_view value) {
	if(stage == Stage::Status)
		status(200, "OK");
	assert(stage == Stage::Headers);
	write_raw(name);
	write_raw(": ");
	write_raw(value);
	write_raw("\r\n");
	return *this;
}
void response::send() {
	if(stage == Stage::Status)
		status(200, "OK");
	end_headers();
	stage = Stage::Sent;
}
void response::send(std::string_view mimeType, std::span<char const> data) {
	if(stage == Stage::Status)
		status(200, "OK");
	assert(stage == Stage::Headers);

	start_content(mimeType, data.size());
	write_raw(data);
	stage = Stage::Sent;
}
void response::send_file(std::string const& path, std::string_view mimeType) {
	if(mimeType.empty())
		mimeType = mimetype_from_path(path);

	auto file = std::ifstream(path, std::ios::binary | std::ios::ate);
	if(!file) {
		if(stage == Stage::Status) {
			status(NotFound, "Not Found").send();
			return;
		}
		else {
			throw std::runtime_error("Failed opening specified file");
		}
	}

	std::size_t fileSize = (std::size_t) file.tellg();
	file.seekg(0, std::ios::beg);

	start_content(mimeType, fileSize);
	std::array<char, 512> buf;
	while(!file.eof()) {
		file.read(buf.data(), buf.size());
		write_raw(std::span<const char>{buf.data(), (size_t) file.gcount()});
	}
}

void response::start_content(std::string_view mimeType, unsigned dataSize) {
	if(stage == Stage::Status)
		status(200, "OK");
	assert(stage == Stage::Headers);

	header("Content-Type", mimeType);
	header("Content-Length", std::to_string(dataSize));
	end_headers();
}

void response::write_raw(std::span<char const> data) {
	if (::send(handle, data.data(), data.size_bytes(), 0) != data.size_bytes()) {
		auto e = std::string(strerror(errno));
		throw std::runtime_error("Failed writing to socket: " + e);
	}
}

static http::method snipHttpMethod(std::string_view& s) {
	if(snipPrefix(s, "GET "))	 return method::Get;
	if(snipPrefix(s, "HEAD "))	return method::Head;
	if(snipPrefix(s, "POST "))	return method::Post;
	if(snipPrefix(s, "PUT "))	 return method::Put;
	if(snipPrefix(s, "DELETE "))  return method::Delete;
	if(snipPrefix(s, "CONNECT ")) return method::Connect;
	if(snipPrefix(s, "OPTIONS ")) return method::Options;
	if(snipPrefix(s, "TRACE "))   return method::Trace;
	if(snipPrefix(s, "PATCH "))   return method::Patch;
	throw std::runtime_error("Unknown HTTP method at request start '"+std::string(s)+"'. Only GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, and PATCH are supported");
}

static void parseFirstLine(std::string_view line, request* req_out) {
	req_out->method = snipHttpMethod(line);
	req_out->location = snipToken(line);
	if(trim(line) != "HTTP/1.1")
		throw std::runtime_error("Only HTTP/1.1 is supported");
}

static void readUntilDoubleNewline(SOCKET client, std::string* buffer) {
	buffer->clear();
	while(true) {
		char c;
		switch(recv(client, &c, 1, 0)) {
			case 0: throw std::runtime_error("Failed recv'ing: Client closed the connection unexpectedly?");
			case -1: throw std::runtime_error("Failed recv'ing from socket");
			default: break;
		}
		buffer->push_back(c);
		if(buffer->ends_with("\r\n\r\n")) {
			buffer->pop_back();
			buffer->pop_back();
			buffer->pop_back();
			buffer->pop_back();
			break;
		}
		if(buffer->ends_with("\n\n")) {
			buffer->pop_back();
			buffer->pop_back();
			break;
		}
	}
}

static void parseRequestHeaders(SOCKET client, request* req_out) {
	readUntilDoubleNewline(client, &req_out->requestText);
	assert(req_out->requestText.size() > 0);

	std::string_view lines = req_out->requestText;
	parseFirstLine(snipLine(lines), req_out);

	req_out->headers.clear();
	while(true) {
		auto line = trim(snipLine(lines));
		if(line.empty()) break;

		auto name = snipUntil(line, ':');
		auto value = trim(line);
		req_out->headers.emplace(name, value);
	}
}

void listen(int port, request_handler const& handler) {
	assert(handler);

	sockInit();
	scoped _a([]() { sockQuit(); });

	SOCKET serverSocket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
	if(!sockValid(serverSocket)) {
		auto e = std::string(strerror(errno));
		throw std::runtime_error("Failed creating the socket: " + e);
	}
	scoped _b([&] { sockClose(serverSocket); });

	sockaddr_in addr = {};
	addr.sin_family = AF_INET;
	addr.sin_port = htons(port);
	addr.sin_addr.s_addr = { inet_addr("127.0.0.1") };
	if(::bind(serverSocket, (sockaddr const*) &addr, sizeof(addr)) == -1) {
		auto e = std::string(strerror(errno));
		throw std::runtime_error("Failed binding server socket to port: " + e);
	}
	if (::listen(serverSocket, 4) == -1) {
		auto e = std::string(strerror(errno));
		throw std::runtime_error("Failed listen'ing to incoming connections" + e);
	}

	while(true) {
		sockaddr  addr = {};
		socklen_t addr_len = sizeof(addr);
		SOCKET	clientSocket = ::accept(serverSocket, (sockaddr*) &addr, &addr_len);
		if(!sockValid(clientSocket)) {
			auto e = std::string(strerror(errno));
			throw std::runtime_error("Failed accepting connection: " + e);
		}
		scoped _d([=]() { sockClose(clientSocket); });

		http::request  req { (unsigned) clientSocket };
		http::response res { (unsigned) clientSocket };
		try {
			parseRequestHeaders(clientSocket, &req);
			handler(req, res);
			switch (res.stage) {
			case response::Stage::Status: res.status(NotFound, "Not Found"); [[fallthrough]];
			case response::Stage::Headers: res.send(); [[fallthrough]];
			case response::Stage::Sent: break;
			}
		}
		catch(std::exception& e) {
			res.status(500, "Internal server error: " + std::string(e.what())).send();
			puts(e.what());
			throw;
		}
	}
}

const char* to_string(method m) {
	switch (m) {
	case Get: return "GET";
	case Head: return "HEAD";
	case Post: return "POST";
	case Put: return "PUT";
	case Delete: return "DELETE";
	case Connect: return "CONNECT";
	case Options: return "OPTIONS";
	case Trace: return "TRACE";
	case Patch: return "PATCH";
	default: return "???";
	}
}

std::string_view mimetype_from_path(std::string_view filename) noexcept {
	if(filename.ends_with(".aac")) return"audio/aac";
	if(filename.ends_with(".abw")) return"application/x-abiword";
	if(filename.ends_with(".arc")) return "application/x-freearc";
	if(filename.ends_with(".avif")) return "image/avif";
	if(filename.ends_with(".avi")) return "video/x-msvideo";
	if(filename.ends_with(".azw")) return "application/vnd.amazon.ebook";
	if(filename.ends_with(".bin")) return "application/octet-stream";
	if(filename.ends_with(".bmp")) return "image/bmp";
	if(filename.ends_with(".bz")) return "application/x-bzip";
	if(filename.ends_with(".bz2")) return "application/x-bzip2";
	if(filename.ends_with(".cda")) return "application/x-cdf";
	if(filename.ends_with(".csh")) return "application/x-csh";
	if(filename.ends_with(".css")) return "text/css";
	if(filename.ends_with(".csv")) return "text/csv";
	if(filename.ends_with(".doc")) return "application/msword";
	if(filename.ends_with(".docx")) return "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
	if(filename.ends_with(".eot")) return "application/vnd.ms-fontobject";
	if(filename.ends_with(".epub")) return "application/epub+zip";
	if(filename.ends_with(".gz")) return "application/gzip";
	if(filename.ends_with(".gif")) return "image/gif";
	if(filename.ends_with(".htm") || filename.ends_with(".html")) return "text/html";
	if(filename.ends_with(".ico")) return "image/vnd.microsoft.icon";
	if(filename.ends_with(".ics")) return "text/calendar";
	if(filename.ends_with(".jar")) return "application/java-archive";
	if(filename.ends_with(".jpeg") || filename.ends_with(".jpg")) return "image/jpeg";
	if(filename.ends_with(".js")) return "text/javascript (Specifications: HTML and RFC 9239)";
	if(filename.ends_with(".json")) return "application/json";
	if(filename.ends_with(".jsonld")) return "application/ld+json";
	if(filename.ends_with(".mid") || filename.ends_with(".midi")) return "audio/midi audio/x-midi";
	if(filename.ends_with(".mjs")) return "text/javascript";
	if(filename.ends_with(".mp3")) return "audio/mpeg";
	if(filename.ends_with(".mp4")) return "video/mp4";
	if(filename.ends_with(".mpeg")) return "video/mpeg";
	if(filename.ends_with(".mpkg")) return "application/vnd.apple.installer+xml";
	if(filename.ends_with(".odp")) return "application/vnd.oasis.opendocument.presentation";
	if(filename.ends_with(".ods")) return "application/vnd.oasis.opendocument.spreadsheet";
	if(filename.ends_with(".odt")) return "application/vnd.oasis.opendocument.text";
	if(filename.ends_with(".oga")) return "audio/ogg";
	if(filename.ends_with(".ogv")) return "video/ogg";
	if(filename.ends_with(".ogx")) return "application/ogg";
	if(filename.ends_with(".opus")) return "audio/opus";
	if(filename.ends_with(".otf")) return "font/otf";
	if(filename.ends_with(".png")) return "image/png";
	if(filename.ends_with(".pdf")) return "application/pdf";
	if(filename.ends_with(".php")) return "application/x-httpd-php";
	if(filename.ends_with(".ppt")) return "application/vnd.ms-powerpoint";
	if(filename.ends_with(".pptx")) return "application/vnd.openxmlformats-officedocument.presentationml.presentation";
	if(filename.ends_with(".rar")) return "application/vnd.rar";
	if(filename.ends_with(".rtf")) return "application/rtf";
	if(filename.ends_with(".sh")) return "application/x-sh";
	if(filename.ends_with(".svg")) return "image/svg+xml";
	if(filename.ends_with(".swf")) return "application/x-shockwave-flash";
	if(filename.ends_with(".tar")) return "application/x-tar";
	if(filename.ends_with(".tif") || filename.ends_with(".tiff")) return "image/tiff";
	if(filename.ends_with(".ts")) return "video/mp2t";
	if(filename.ends_with(".ttf")) return "font/ttf";
	if(filename.ends_with(".txt")) return "text/plain";
	if(filename.ends_with(".vsd")) return "application/vnd.visio";
	if(filename.ends_with(".wav")) return "audio/wav";
	if(filename.ends_with(".weba")) return "audio/webm";
	if(filename.ends_with(".webm")) return "video/webm";
	if(filename.ends_with(".webp")) return "image/webp";
	if(filename.ends_with(".woff")) return "font/woff";
	if(filename.ends_with(".woff2")) return "font/woff2";
	if(filename.ends_with(".xhtml")) return "application/xhtml+xml";
	if(filename.ends_with(".xls")) return "application/vnd.ms-excel";
	if(filename.ends_with(".xlsx")) return "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
	if(filename.ends_with(".xml")) return "application/xml";
	if(filename.ends_with(".xul")) return "application/vnd.mozilla.xul+xml";
	if(filename.ends_with(".zip")) return "application/zip";
	if(filename.ends_with(".3gp")) return "video/3gpp; audio/3gpp if it doesn't contain video";
	if(filename.ends_with(".3g2")) return "video/3gpp2; audio/3gpp2 if it doesn't contain video";
	if(filename.ends_with(".7z")) return "application/x-7z-compressed";
	return "application/octet-stream";
}

} // namespace jdict::http
