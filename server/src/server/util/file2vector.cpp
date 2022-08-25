#include "./file2vector.hpp"

#include "./timer.hpp"
#include <string>

namespace jdict {

std::vector<char> read_file_to_vector(const char* path, bool append_null_terminator) {
	debug::timer _("reading " + std::string(path));

	auto* file = fopen(path, "rb");
	if(!file)
		throw std::runtime_error("Failed opening file '" + std::string(path) + "'");

	fseek(file, 0, SEEK_END);
	unsigned size = ftell(file);
	fseek(file, 0, SEEK_SET);

	auto data = std::vector<char>(size + (append_null_terminator? 1 : 0));
	if(append_null_terminator)
		data.back() = '\0';
	fread(data.data(), size, 1, file);

	fclose(file);

	return data;
}


} // namespace jdict
