#pragma once
#include <filesystem>
#include <string>
#include <vector>
namespace cling::driver {
enum class EmitKind { Run, Tokens, AST, CIL, C, Executable };
struct Options {
 EmitKind emit = EmitKind::Run;
 std::filesystem::path input;
 std::filesystem::path output;
 std::string target = "host";
 bool showVersion = false;
 bool showHelp = false;
 bool verbose = false;
};
struct OptionParseResult {
 Options options;
 std::vector<std::string> errors;
 [[nodiscard]] bool success() const noexcept { return errors.empty(); }
};
[[nodiscard]] OptionParseResult parseOptions(int argc, const char* const* argv);
[[nodiscard]] std::string usageText(const char* executable);
} // namespace cling::driver
