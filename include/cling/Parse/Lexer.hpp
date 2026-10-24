#pragma once

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

namespace cling::parse {

struct SourceLocation {
    std::size_t offset = 0;
    std::size_t line = 1;
    std::size_t column = 1;
};

struct SourceSpan {
    SourceLocation begin;
    SourceLocation end;
};

enum class TokenKind {
    Identifier,
    Integer,
    String,
    Func,
    Let,
    Var,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Pass,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    Semicolon,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    EndOfFile
};

struct Token {
    TokenKind kind = TokenKind::EndOfFile;
    std::string lexeme;
    std::string decodedString;
    std::int64_t integerValue = 0;
    SourceSpan span;
};

class LexError final : public std::runtime_error {
public:
    LexError(std::string message, SourceSpan span);

    const SourceSpan& span() const noexcept;

private:
    SourceSpan span_;
};

class Lexer final {
public:
    explicit Lexer(std::string_view source);

    std::vector<Token> tokenize();

private:
    std::string_view source_;
    std::size_t offset_ = 0;
    std::size_t line_ = 1;
    std::size_t column_ = 1;
    std::size_t tokenStart_ = 0;
    SourceLocation tokenLocation_;

    char peek(std::size_t lookahead = 0) const noexcept;
    char advance() noexcept;
    bool match(char expected) noexcept;
    bool atEnd() const noexcept;
    SourceLocation location() const noexcept;
    SourceSpan currentSpan() const noexcept;
    void scanToken(std::vector<Token>& tokens);
    void scanNumber(std::vector<Token>& tokens);
    void scanIdentifier(std::vector<Token>& tokens);
    void scanString(std::vector<Token>& tokens);
    void skipLineComment() noexcept;
    void emit(std::vector<Token>& tokens, TokenKind kind, std::string decoded = {});
    [[noreturn]] void fail(std::string message) const;
};

const char* tokenKindName(TokenKind kind) noexcept;

}
