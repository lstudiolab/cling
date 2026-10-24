#include "cling/Basic/Diagnostic.h"
#include "cling/Basic/SourceLocation.h"
#include "cling/Lex/Lexer.h"
#include "cling/Lex/TokenKind.h"

#include <fstream>
#include <iostream>
#include <iterator>
#include <string>

namespace {
void printUsage(const char* executable) {
    std::cerr << "Usage: " << executable << " [--emit tokens] <file.cling>\n"
              << "The C++ bootstrap currently exposes its lexer inspection mode.\n";
}
} // namespace

int main(int argc, char** argv) {
    bool emitTokens = false;
    std::string inputPath;

    for (int index = 1; index < argc; ++index) {
        const std::string argument(argv[index]);
        if (argument == "--help" || argument == "-h") {
            printUsage(argv[0]);
            return 0;
        }
        if (argument == "--version") {
            std::cout << "Cling bootstrap 0.2.0 (C++20)\n";
            return 0;
        }
        if (argument == "--emit") {
            if (index + 1 >= argc || std::string(argv[index + 1]) != "tokens") {
                std::cerr << "error: this bootstrap currently supports only '--emit tokens'\n";
                return 2;
            }
            emitTokens = true;
            ++index;
            continue;
        }
        if (!inputPath.empty()) {
            std::cerr << "error: unexpected extra input '" << argument << "'\n";
            return 2;
        }
        inputPath = argument;
    }

    if (inputPath.empty()) {
        printUsage(argv[0]);
        return 2;
    }
    if (!emitTokens) {
        std::cerr << "error: parser and code generation are being ported to the C++ bootstrap; "
                     "use '--emit tokens' for the currently implemented phase\n";
        return 2;
    }

    std::ifstream input(inputPath, std::ios::binary);
    if (!input) {
        std::cerr << "error: cannot open source file '" << inputPath << "'\n";
        return 1;
    }
    const std::string contents((std::istreambuf_iterator<char>(input)),
                               std::istreambuf_iterator<char>());
    const cling::SourceBuffer source(inputPath, contents);
    cling::DiagnosticEngine diagnostics;
    cling::Lexer lexer(source, diagnostics);
    for (const auto& token : lexer.lex()) {
        std::cout << token.range.begin.line << ':' << token.range.begin.column << ' '
                  << cling::tokenKindName(token.kind) << " \"" << token.text << "\"\n";
    }
    diagnostics.print(std::cerr, source);
    return diagnostics.hasErrors() ? 1 : 0;
}
