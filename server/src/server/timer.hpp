#include <chrono>

namespace jdict::debug {

struct timer {
    using clock        = std::chrono::high_resolution_clock;
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

        auto micros = duration_cast<microseconds>(duration).count();
        printf("%s took %fms\n", msg, micros / 1000.0);
    }
};

} // namespace jdict::debug