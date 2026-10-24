#include "cling/Parse/Lexer.hpp"

#include <cstdlib>
#include <iostream>
#include <string>

namespace {

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << message << '\n';
        std::exit(1);
    }
}

}

int main() {
    const std::string source =
        "func main() {\n"
        "  let count: Int = 42; // ignored\n"
        "  print(\"hello\\nworld\");\n"
        "  if (count >= 2 && true) { pass; }\n"
        "}";

    const auto tokens = cling::parse::Lexer(source).tokenize();
    using cling::parse::TokenKind;

    require(tokens.front().kind == TokenKind::Func, "first token must be func");
    require(tokens[0].span.begin.line == 1, "first token line must be one");

    bool sawInteger = false;
    bool sawString = false;
    bool sawAnd = false;
    bool sawGreaterEqual = false;
    bool sawEof = false;

    for (const auto& token : tokens) {
        if (token.kind == TokenKind::Integer) {
            sawInteger = token.integerValue == 42;
        } else if (token.kind == TokenKind::String) {
            sawString = token.decodedString == "hello\nworld";
        } else if (token.kind == TokenKind::AndAnd) {
            sawAnd = true;
        } else if (token.kind == TokenKind::GreaterEqual) {
            sawGreaterEqual = true;
        } else if (token.kind == TokenKind::EndOfFile) {
            sawEof = true;
        }
    }

    require(sawInteger, "integer literal must be decoded");
    require(sawString, "string escapes must be decoded");
    require(sawAnd, "logical and token must be recognized");
    require(sawGreaterEqual, "greater-or-equal token must be recognized");
    require(sawEof, "token stream must end with EOF");

    try {
        (void)cling::parse::Lexer("\"bad\\q\"").tokenize();
        require(false, "unsupported escape must fail");
    } catch (const cling::parse::LexError& error) {
        require(error.span().begin.line == 1, "lex error must include source location");
    }

    try {
        (void)cling::parse::Lexer("9223372036854775808").tokenize();
        require(false, "integer overflow must fail");
    } catch (const cling::parse::LexError&) {
    }

    std::cout << "Lexer tests passed\n";
    return 0;
}
