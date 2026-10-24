#pragma once

#include <string_view>

namespace cling {

enum class TokenKind {
    EndOfFile,
    Identifier,
    IntegerLiteral,
    StringLiteral,

    KwFunc,
    KwLet,
    KwVar,
    KwIf,
    KwElse,
    KwWhile,
    KwReturn,
    KwTrue,
    KwFalse,
    KwPrint,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Dot,
    Arrow,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpAmp,
    PipePipe,

    Unknown
};

[[nodiscard]] std::string_view tokenKindName(TokenKind kind) noexcept;

} // namespace cling
