#pragma once

#include "cling/Basic/Diagnostic.h"
#include "cling/Lex/Token.h"

#include <cstddef>
#include <vector>

namespace cling {

class ASTContext;
class Program;

class Parser {
public:
    Parser(const std::vector<Token>& tokens, DiagnosticEngine& diagnostics, ASTContext& context);
    [[nodiscard]] Program* parseProgram();

private:
    [[nodiscard]] const Token& current() const;
    [[nodiscard]] const Token& previous() const;
    bool at(TokenKind kind) const;
    bool consume(TokenKind kind);
    bool expect(TokenKind kind, const char* message);
    void synchronize();

    const std::vector<Token>& tokens_;
    DiagnosticEngine& diagnostics_;
    ASTContext& context_;
    std::size_t cursor_ = 0;
};

} // namespace cling
