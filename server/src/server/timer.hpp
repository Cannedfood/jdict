#include <bits/chrono.h>
#include <chrono>
#include <string>

namespace jdict::debug {

template<typename Period, typename Rep>
std::string to_string(std::chrono::duration<Period, Rep> d) {
	auto micros = duration_cast<std::chrono::microseconds>(d).count();
	if(micros < 1000)
		return std::to_string(micros) + "μs";
	if(micros < 1000 * 1000)
		return std::to_string(micros / 1000.0) + "ms";
	if(micros < 60 * 1000 * 1000)
		return std::to_string(micros / 1000000.0) + "s";

	auto full_minutes = micros / (1000 * 1000 * 60);
	auto seconds = micros / 1000000.0 - (full_minutes * 60);
	return std::to_string(full_minutes) + "m " + std::to_string(seconds) + "s";
}

struct timer {
	using clock		= std::chrono::high_resolution_clock;
	using time_point   = clock::time_point;
	using microseconds = std::chrono::microseconds;

	const char* msg;
	time_point start;
	timer(const char* msg) :
		msg(msg),
		start(clock::now())
	{}
	~timer() {
		auto end = clock::now();
		auto duration = end - start;

		printf("%s took %s\n", msg, to_string(duration).c_str());
	}
};

} // namespace jdict::debug
