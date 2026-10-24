#include "cling/Parse/Lexer.hpp"

#include <charconv>
#include <cctype>
#include <limits>
#include <system_error>
#include <unordered_map>
#include <utility>

namespace cling::parse {

LexError::LexError(std::string message, SourceSpan span)
    : std::runtime_error(std::move(message)), span_(span) {}

const SourceSpan& LexError::span() const noexcept {
    return span_;
}

Lexer::Lexer(std::string_view source) : source_(source) {}

bool Lexer::atEnd() const noexcept {
    return offset_ >= source_.size();
}

char Lexer::peek(std::size_t lookahead) const noexcept {
    const auto index = offset_ + lookahead;
    return index < source_.size() ? source_[index] : '\0';
}

char Lexer::advance() noexcept {
    if (atEnd()) {
        return '\0';
    }

    const char value = source_[offset_++];
    if (value == '\n') {
        ++line_;
        column_ = 1;
    } else {
        ++column_;
    }
    return value;
}

bool Lexer::match(char expected) noexcept {
    if (peek() != expected || atEnd()) {
        return false;
    }
    advance();
    return true;
}

SourceLocation Lexer::location() const noexcept {
    return {offset_, line_, column_};
}

SourceSpan Lexer::currentSpan() const noexcept {
    return {tokenLocation_, location()};
}

void Lexer::emit(std::vector<Token>& tokens, TokenKind kind, std::string decoded) {
    tokens.push_back(Token{
        kind,
        std::string(source_.substr(tokenStart_, offset_ - tokenStart_)),
        std::move(decoded),
        0,
        currentSpan()
    });
}

[[noreturn]] void Lexer::fail(std::string message) const {
    throw LexError(std::move(message), currentSpan());
}

void Lexer::skipLineComment() noexcept {
    while (!atEnd() && peek() != '\n') {
        advance();
    }
}

void Lexer::scanNumber(std::vector<Token>& tokens) {
    while (std::isdigit(static_cast<unsigned char>(peek())) != 0) {
        advance();
    }

    const auto text = source_.substr(tokenStart_, offset_ - tokenStart_);
    std::int64_t value = 0;
    const auto result = std::from_chars(text.data(), text.data() + text.size(), value);
    if (result.ec == std::errc::result_out_of_range) {
        fail("integer literal is outside the signed 64-bit range");
    }
    if (result.ec != std::errc{} || result.ptr != text.data() + text.size()) {
        fail("invalid integer literal");
    }

    tokens.push_back(Token{
        TokenKind::Integer,
        std::string(text),
        {},
        value,
        currentSpan()
    });
}

void Lexer::scanIdentifier(std::vector<Token>& tokens) {
    const auto isIdentifierStart = [](unsigned char c) {
        return c == '_' || std::isalpha(c) != 0 || c >= 0x80;
    };
    const auto isIdentifierContinue = [](unsigned char c) {
        return c == '_' || std::isalnum(c) != 0 || c >= 0x80;
    };

    while (!atEnd() && isIdentifierContinue(static_cast<unsigned char>(peek()))) {
        advance();
    }

    const std::string text(source_.substr(tokenStart_, offset_ - tokenStart_));
    static const std::unordered_map<std::string, TokenKind> keywords{
        {"func", TokenKind::Func},
        {"let", TokenKind::Let},
        {"var", TokenKind::Var},
        {"if", TokenKind::If},
        {"else", TokenKind::Else},
        {"while", TokenKind::While},
        {"return", TokenKind::Return},
        {"true", TokenKind::True},
        {"false", TokenKind::False},
        {"pass", TokenKind::Pass}
    };

    const auto keyword = keywords.find(text);
    if (keyword != keywords.end()) {
        emit(tokens, keyword->second);
    } else if (isIdentifierStart(static_cast<unsigned char>(text.front()))) {
        emit(tokens, TokenKind::Identifier);
    } else {
        fail("invalid identifier");
    }
}

void Lexer::scanString(std::vector<Token>& tokens) {
    std::string decoded;
    while (!atEnd() && peek() != '"') {
        const char value = advance();
        if (value == '\n' || value == '\r') {
            fail("newline in string literal");
        }
        if (value != '\\') {
            decoded.push_back(value);
            continue;
        }

        if (atEnd()) {
            fail("unterminated escape sequence");
        }

        switch (advance()) {
            case 'n': decoded.push_back('\n'); break;
            case 'r': decoded.push_back('\r'); break;
            case 't': decoded.push_back('\t'); break;
            case '0': decoded.push_back('\0'); break;
            case '"': decoded.push_back('"'); break;
            case '\\': decoded.push_back('\\'); break;
            default: fail("unsupported escape sequence");
        }
    }

    if (!match('"')) {
        fail("unterminated string literal");
    }

    emit(tokens, TokenKind::String, std::move(decoded));
}

void Lexer::scanToken(std::vector<Token>& tokens) {
    const char value = advance();
    switch (value) {
        case ' ':
        case '\r':
        case '\t':
        case '\n':
            return;
        case '(' : emit(tokens, TokenKind::LeftParen); return;
        case ')' : emit(tokens, TokenKind::RightParen); return;
        case '{' : emit(tokens, TokenKind::LeftBrace); return;
        case '}' : emit(tokens, TokenKind::RightBrace); return;
        case ':' : emit(tokens, TokenKind::Colon); return;
        case ',' : emit(tokens, TokenKind::Comma); return;
        case ';' : emit(tokens, TokenKind::Semicolon); return;
        case '+' : emit(tokens, TokenKind::Plus); return;
        case '*' : emit(tokens, TokenKind::Star); return;
        case '%' : emit(tokens, TokenKind::Percent); return;
        case '/':
            if (match('/')) {
                skipLineComment();
            } else {
                emit(tokens, TokenKind::Slash);
            }
            return;
        case '-':
            emit(tokens, match('>') ? TokenKind::Arrow : TokenKind::Minus);
            return;
        case '!':
            emit(tokens, match('=') ? TokenKind::BangEqual : TokenKind::Bang);
            return;
        case '=':
            emit(tokens, match('=') ? TokenKind::EqualEqual : TokenKind::Equal);
            return;
        case '<':
            emit(tokens, match('=') ? TokenKind::LessEqual : TokenKind::Less);
            return;
        case '>':
            emit(tokens, match('=') ? TokenKind::GreaterEqual : TokenKind::Greater);
            return;
        case '&':
            if (match('&')) {
                emit(tokens, TokenKind::AndAnd);
                return;
            }
            fail("expected '&' after '&'");
        case '|':
            if (match('|')) {
                emit(tokens, TokenKind::OrOr);
                return;
            }
            fail("expected '|' after '|'");
        case '"':
            scanString(tokens);
            return;
        default:
            break;
    }

    if (std::isdigit(static_cast<unsigned char>(value)) != 0) {
        scanNumber(tokens);
        return;
    }

    const auto byte = static_cast<unsigned char>(value);
    if (value == '_' || std::isalpha(byte) != 0 || byte >= 0x80) {
        scanIdentifier(tokens);
        return;
    }

    fail(std::string("unexpected character '") + value + "'");
}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;
    tokens.reserve(source_.size() / 3 + 1);

    while (!atEnd()) {
        tokenStart_ = offset_;
        tokenLocation_ = location();
        scanToken(tokens);
    }

    const auto end = location();
    tokens.push_back(Token{
        TokenKind::EndOfFile,
        {},
        {},
        0,
        {end, end}
    });
    return tokens;
}

const char* tokenKindName(TokenKind kind) noexcept {
    switch (kind) {
        case TokenKind::Identifier: return "identifier";
        case TokenKind::Integer: return "integer";
        case TokenKind::String: return "string";
        case TokenKind::Func: return "func";
        case TokenKind::Let: return "let";
        case TokenKind::Var: return "var";
        case TokenKind::If: return "if";
        case TokenKind::Else: return "else";
        case TokenKind::While: return "while";
        case TokenKind::Return: return "return";
        case TokenKind::True: return "true";
        case TokenKind::False: return "false";
        case TokenKind::Pass: return "pass";
        case TokenKind::LeftParen: return "(";
        case TokenKind::RightParen: return ")";
        case TokenKind::LeftBrace: return "{";
        case TokenKind::RightBrace: return "}";
        case TokenKind::Colon: return ":";
        case TokenKind::Comma: return ",";
        case TokenKind::Semicolon: return ";";
        case TokenKind::Arrow: return "->";
        case TokenKind::Plus: return "+";
        case TokenKind::Minus: return "-";
        case TokenKind::Star: return "*";
        case TokenKind::Slash: return "/";
        case TokenKind::Percent: return "%";
        case TokenKind::Bang: return "!";
        case TokenKind::Equal: return "=";
        case TokenKind::EqualEqual: return "==";
        case TokenKind::BangEqual: return "!=";
        case TokenKind::Less: return "<";
        case TokenKind::LessEqual: return "<=";
        case TokenKind::Greater: return ">";
        case TokenKind::GreaterEqual: return ">=";
        case TokenKind::AndAnd: return "&&";
        case TokenKind::OrOr: return "||";
        case TokenKind::EndOfFile: return "end of file";
    }
    return "unknown token";
}

}
