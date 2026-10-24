#include "cling/Lex/TokenKind.h"

namespace cling {

std::string_view tokenKindName(TokenKind kind) noexcept {
    switch (kind) {
    case TokenKind::EndOfFile: return "end of file";
    case TokenKind::Identifier: return "identifier";
    case TokenKind::IntegerLiteral: return "integer literal";
    case TokenKind::StringLiteral: return "string literal";
    case TokenKind::KwFunc: return "'func'";
    case TokenKind::KwLet: return "'let'";
    case TokenKind::KwVar: return "'var'";
    case TokenKind::KwIf: return "'if'";
    case TokenKind::KwElse: return "'else'";
    case TokenKind::KwWhile: return "'while'";
    case TokenKind::KwReturn: return "'return'";
    case TokenKind::KwTrue: return "'true'";
    case TokenKind::KwFalse: return "'false'";
    case TokenKind::KwPrint: return "'print'";
    case TokenKind::LeftParen: return "'('";
    case TokenKind::RightParen: return "')'";
    case TokenKind::LeftBrace: return "'{'";
    case TokenKind::RightBrace: return "'}'";
    case TokenKind::LeftBracket: return "'['";
    case TokenKind::RightBracket: return "']'";
    case TokenKind::Comma: return "','";
    case TokenKind::Colon: return "':'";
    case TokenKind::Semicolon: return "';'";
    case TokenKind::Dot: return "'.'";
    case TokenKind::Arrow: return "'->'";
    case TokenKind::Plus: return "'+'";
    case TokenKind::Minus: return "'-'";
    case TokenKind::Star: return "'*'";
    case TokenKind::Slash: return "'/'";
    case TokenKind::Percent: return "'%'";
    case TokenKind::Equal: return "'='";
    case TokenKind::EqualEqual: return "'=='";
    case TokenKind::Bang: return "'!'";
    case TokenKind::BangEqual: return "'!='";
    case TokenKind::Less: return "'<'";
    case TokenKind::LessEqual: return "'<='";
    case TokenKind::Greater: return "'>'";
    case TokenKind::GreaterEqual: return "'>='";
    case TokenKind::AmpAmp: return "'&&'";
    case TokenKind::PipePipe: return "'||'";
    case TokenKind::Unknown: return "unknown token";
    }
    return "unknown token";
}

} // namespace cling
