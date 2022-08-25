#pragma once

#include <vector>

namespace jdict {

/// Reads a file into a vector. Should support .zip files in the future
std::vector<char> read_file_to_vector(const char* path, bool append_null_terminator = false);

} // namespace jdict
