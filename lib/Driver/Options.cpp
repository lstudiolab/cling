#include "cling/Driver/Options.h"

#include <string_view>

namespace cling::driver {
namespace {
bool takeValue(int& index, int argc, const char* const* argv, std::string& value,
               std::vector<std::string>& errors, std::string_view option) {
    if (index + 1 >= argc) {
        errors.push_back("option '" + std::string(option) + "' requires a value");
        return false;
    }
    value = argv[++index];
    return true;
}
}

OptionParseResult parseOptions(int argc, const char* const* argv) {
    OptionParseResult result;
    bool hasInput = false;
    for (int index = 1; index < argc; ++index) {
        const std::string argument(argv[index]);
        if (argument == "--help" || argument == "-h") {
            result.options.showHelp = true;
        } else if (argument == "--version") {
            result.options.showVersion = true;
        } else if (argument == "--verbose" || argument == "-v") {
            result.options.verbose = true;
        } else if (argument == "--emit") {
            std::string value;
            if (!takeValue(index, argc, argv, value, result.errors, "--emit")) continue;
            if (value == "run") result.options.emit = EmitKind::Run;
            else if (value == "tokens") result.options.emit = EmitKind::Tokens;
            else if (value == "ast") result.options.emit = EmitKind::AST;
            else if (value == "cil") result.options.emit = EmitKind::CIL;
            else if (value == "c") result.options.emit = EmitKind::C;
            else if (value == "exe") result.options.emit = EmitKind::Executable;
            else result.errors.push_back("unknown emission mode '" + value + "'");
        } else if (argument == "--target") {
            (void)takeValue(index, argc, argv, result.options.target, result.errors, "--target");
        } else if (argument == "-o" || argument == "--output") {
            std::string value;
            if (takeValue(index, argc, argv, value, result.errors, argument))
                result.options.output = value;
        } else if (!argument.empty() && argument[0] == '-') {
            result.errors.push_back("unknown option '" + argument + "'");
        } else if (hasInput) {
            result.errors.push_back("only one input file is accepted in this bootstrap");
        } else {
            result.options.input = argument;
            hasInput = true;
        }
    }
    if (!result.options.showHelp && !result.options.showVersion && !hasInput)
        result.errors.push_back("no input file was provided");
    if (result.options.target != "host")
        result.errors.push_back("target '" + result.options.target + "' has no implemented backend");
    return result;
}

std::string usageText(const char* executable) {
    return std::string("Usage: ") + executable + " [options] <file.cling>\n"
        "Options:\n"
        "  --emit <run|tokens|ast|cil|c|exe>  Select compiler output mode\n"
        "  --target <host>                    Select the implemented target\n"
        "  -o, --output <path>                Write output to a file\n"
        "  -v, --verbose                      Print additional driver information\n"
        "  --version                          Print compiler version\n"
        "  -h, --help                         Show this help\n";
}
} // namespace cling::driver
