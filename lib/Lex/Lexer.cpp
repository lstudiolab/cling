#include "cling/Lex/Lexer.h"

#include <cctype>
#include <unordered_map>
#include <utility>

namespace cling {
namespace {
const std::unordered_map<std::string, TokenKind> Keywords = {
    {"func", TokenKind::KwFunc}, {"let", TokenKind::KwLet},
    {"var", TokenKind::KwVar}, {"if", TokenKind::KwIf},
    {"else", TokenKind::KwElse}, {"while", TokenKind::KwWhile},
    {"return", TokenKind::KwReturn}, {"true", TokenKind::KwTrue},
    {"false", TokenKind::KwFalse}, {"print", TokenKind::KwPrint},
};
} // namespace

Lexer::Lexer(const SourceBuffer& source, DiagnosticEngine& diagnostics)
    : source_(source), diagnostics_(diagnostics) {}

char Lexer::current() const noexcept { return source_.at(offset_); }
char Lexer::peek(std::size_t distance) const noexcept { return source_.at(offset_ + distance); }

char Lexer::advance() noexcept {
    const char value = current();
    if (!source_.isEOF(offset_)) {
        ++offset_;
        if (value == '\n') {
            ++line_;
            column_ = 1;
        } else {
            ++column_;
        }
    }
    return value;
}

bool Lexer::consume(char expected) noexcept {
    if (current() != expected) return false;
    advance();
    return true;
}

std::vector<Token> Lexer::lex() {
    std::vector<Token> tokens;
    for (;;) {
        skipTrivia();
        Token token = nextToken();
        const bool finished = token.kind == TokenKind::EndOfFile;
        tokens.push_back(std::move(token));
        if (finished) break;
    }
    return tokens;
}

void Lexer::skipTrivia() {
    for (;;) {
        while (std::isspace(static_cast<unsigned char>(current())) != 0) advance();
        if (current() == '/' && peek() == '/') {
            while (!source_.isEOF(offset_) && current() != '\n') advance();
            continue;
        }
        if (current() == '/' && peek() == '*') {
            const SourceLocation begin{offset_, line_, column_};
            advance();
            advance();
            std::size_t depth = 1;
            while (depth != 0 && !source_.isEOF(offset_)) {
                if (current() == '/' && peek() == '*') {
                    advance(); advance(); ++depth;
                } else if (current() == '*' && peek() == '/') {
                    advance(); advance(); --depth;
                } else {
                    advance();
                }
            }
            if (depth != 0) diagnostics_.error(begin, "unterminated block comment");
            continue;
        }
        return;
    }
}

Token Lexer::nextToken() {
    const std::size_t start = offset_;
    const SourceLocation begin{offset_, line_, column_};
    if (source_.isEOF(offset_)) return makeToken(TokenKind::EndOfFile, start, begin);

    const unsigned char value = static_cast<unsigned char>(current());
    if (std::isalpha(value) != 0 || current() == '_') {
        advance();
        while (std::isalnum(static_cast<unsigned char>(current())) != 0 || current() == '_') advance();
        return identifierOrKeyword(start, begin);
    }
    if (std::isdigit(value) != 0) {
        advance();
        while (std::isdigit(static_cast<unsigned char>(current())) != 0) advance();
        return number(start, begin);
    }
    if (current() == '"') return string(start, begin);
    return punctuation(start, begin);
}

Token Lexer::makeToken(TokenKind kind, std::size_t start, SourceLocation begin) const {
    return Token{kind, SourceRange{begin, SourceLocation{offset_, line_, column_}},
                 std::string(source_.contents().substr(start, offset_ - start))};
}

Token Lexer::identifierOrKeyword(std::size_t start, SourceLocation begin) {
    std::string text(source_.contents().substr(start, offset_ - start));
    const auto found = Keywords.find(text);
    return makeToken(found == Keywords.end() ? TokenKind::Identifier : found->second, start, begin);
}

Token Lexer::number(std::size_t start, SourceLocation begin) {
    return makeToken(TokenKind::IntegerLiteral, start, begin);
}

Token Lexer::string(std::size_t start, SourceLocation begin) {
    advance();
    bool terminated = false;
    while (!source_.isEOF(offset_)) {
        if (current() == '"') {
            advance();
            terminated = true;
            break;
        }
        if (current() == '\n') {
            diagnostics_.error(SourceLocation{offset_, line_, column_},
                               "newline in string literal; escape it as \\n");
            break;
        }
        if (current() == '\\') {
            advance();
            if (source_.isEOF(offset_)) break;
            const char escaped = advance();
            if (escaped != 'n' && escaped != 'r' && escaped != 't' &&
                escaped != '\\' && escaped != '"') {
                diagnostics_.warning(SourceLocation{offset_ - 1, line_, column_ - 1},
                                     "unknown escape sequence");
            }
        } else {
            advance();
        }
    }
    if (!terminated) diagnostics_.error(begin, "unterminated string literal");
    return makeToken(TokenKind::StringLiteral, start, begin);
}

Token Lexer::punctuation(std::size_t start, SourceLocation begin) {
    const char value = advance();
    TokenKind kind = TokenKind::Unknown;
    switch (value) {
    case '(': kind = TokenKind::LeftParen; break;
    case ')': kind = TokenKind::RightParen; break;
    case '{': kind = TokenKind::LeftBrace; break;
    case '}': kind = TokenKind::RightBrace; break;
    case '[': kind = TokenKind::LeftBracket; break;
    case ']': kind = TokenKind::RightBracket; break;
    case ',': kind = TokenKind::Comma; break;
    case ':': kind = TokenKind::Colon; break;
    case ';': kind = TokenKind::Semicolon; break;
    case '.': kind = TokenKind::Dot; break;
    case '+': kind = TokenKind::Plus; break;
    case '*': kind = TokenKind::Star; break;
    case '%': kind = TokenKind::Percent; break;
    case '-': kind = consume('>') ? TokenKind::Arrow : TokenKind::Minus; break;
    case '=': kind = consume('=') ? TokenKind::EqualEqual : TokenKind::Equal; break;
    case '!': kind = consume('=') ? TokenKind::BangEqual : TokenKind::Bang; break;
    case '<': kind = consume('=') ? TokenKind::LessEqual : TokenKind::Less; break;
    case '>': kind = consume('=') ? TokenKind::GreaterEqual : TokenKind::Greater; break;
    case '&': kind = consume('&') ? TokenKind::AmpAmp : TokenKind::Unknown; break;
    case '|': kind = consume('|') ? TokenKind::PipePipe : TokenKind::Unknown; break;
    case '/': kind = TokenKind::Slash; break;
    default: break;
    }
    if (kind == TokenKind::Unknown) {
        diagnostics_.error(begin, "unexpected character");
    }
    return makeToken(kind, start, begin);
}

} // namespace cling
