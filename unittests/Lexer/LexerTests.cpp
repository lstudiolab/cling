#include "cling/Basic/Diagnostic.h"
#include "cling/Basic/SourceLocation.h"
#include "cling/Lex/Lexer.h"
#include "cling/Lex/TokenKind.h"

#include <cstdlib>
#include <iostream>
#include <string>
#include <vector>

namespace {
void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << "FAILED: " << message << '\n';
        std::exit(1);
    }
}
} // namespace

int main() {
    const std::string program =
        "func main() {\n"
        "  let answer = 40 + 2; // trailing comment\n"
        "  print(\"Hello\\nworld\");\n"
        "}\n";
    const cling::SourceBuffer source("lexer-test.cling", program);
    cling::DiagnosticEngine diagnostics;
    cling::Lexer lexer(source, diagnostics);
    const auto tokens = lexer.lex();

    require(!diagnostics.hasErrors(), "valid source should lex without errors");
    require(tokens.size() > 15, "lexer should produce a useful token stream");
    require(tokens.front().kind == cling::TokenKind::KwFunc, "func must be a keyword");
    require(tokens.back().kind == cling::TokenKind::EndOfFile, "stream must end with EOF");

    bool sawInteger = false;
    bool sawString = false;
    bool sawArrow = false;
    for (const auto& token : tokens) {
        sawInteger = sawInteger || token.kind == cling::TokenKind::IntegerLiteral;
        sawString = sawString || token.kind == cling::TokenKind::StringLiteral;
        sawArrow = sawArrow || token.kind == cling::TokenKind::Arrow;
    }
    require(sawInteger, "integer literals must be recognized");
    require(sawString, "string literals must be recognized");
    require(!sawArrow, "source does not contain an arrow token");

    const cling::SourceBuffer invalid("invalid.cling", "print(\"oops\n");
    cling::DiagnosticEngine invalidDiagnostics;
    cling::Lexer invalidLexer(invalid, invalidDiagnostics);
    (void)invalidLexer.lex();
    require(invalidDiagnostics.hasErrors(), "unterminated strings must be diagnosed");

    std::cout << "Cling lexer tests passed\n";
    return 0;
}
