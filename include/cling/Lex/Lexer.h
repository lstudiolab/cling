#pragma once

#include "cling/Basic/Diagnostic.h"
#include "cling/Basic/SourceLocation.h"
#include "cling/Lex/Token.h"

#include <vector>

namespace cling {

class Lexer {
public:
    Lexer(const SourceBuffer& source, DiagnosticEngine& diagnostics);

    [[nodiscard]] std::vector<Token> lex();

private:
    [[nodiscard]] char current() const noexcept;
    [[nodiscard]] char peek(std::size_t distance = 1) const noexcept;
    char advance() noexcept;
    bool consume(char expected) noexcept;
    void skipTrivia();
    [[nodiscard]] Token nextToken();
    [[nodiscard]] Token makeToken(TokenKind kind, std::size_t start,
                                  SourceLocation begin) const;
    [[nodiscard]] Token identifierOrKeyword(std::size_t start, SourceLocation begin);
    [[nodiscard]] Token number(std::size_t start, SourceLocation begin);
    [[nodiscard]] Token string(std::size_t start, SourceLocation begin);
    [[nodiscard]] Token punctuation(std::size_t start, SourceLocation begin);

    const SourceBuffer& source_;
    DiagnosticEngine& diagnostics_;
    std::size_t offset_ = 0;
    std::size_t line_ = 1;
    std::size_t column_ = 1;
};

} // namespace cling
