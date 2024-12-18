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

    virtual bool compareValue(const Token& other) const { return true; };

    static std::unique_ptr<Token> createToken(TokenType type, const std::string &value);
    static bool compareTokens(const Token& token1, const Token& token2);

protected:
private:
    TokenType type;
};