#pragma once

#include <variant>
#include <string>
#include <memory>

enum TokenType
{
    TIdentifier,
    TConstant,
    TKeyword,
    TOpenParenthesis,
    TCloseParenthesis,
    TOpenBrace,
    TCloseBrace,
    TSemicolon,
};

class Token
{
public:
    TokenType getType() const { return type; }

    Token(const TokenType &type) : type(type) {}
    virtual ~Token() = default;

    static std::unique_ptr<Token> createToken(TokenType type, const std::string &value);

protected:

private:
    TokenType type;
};