#include "token.h"
#include <unordered_map>
#include "tokens.h"
#include <iostream>

const std::unordered_map<std::string, KeywordType> KEYWORDS = {
    {"int", KInt},
    {"void", KVoid},
    {"return", KReturn}};

std::unique_ptr<Token> Token::createToken(TokenType type, const std::string &value)
{
    switch (type)
    {
    case TIdentifier:
        if (KEYWORDS.find(value) != KEYWORDS.end())
        {
            return std::make_unique<KeywordToken>(KEYWORDS.at(value));
        }
        else
        {
            return std::make_unique<IdentifierToken>(value);
        }
    case TConstant:
        return std::make_unique<ConstantToken>(atoi(value.c_str()));
    case TKeyword:
        return std::make_unique<KeywordToken>(KEYWORDS.at(value));
    default:
        return std::make_unique<Token>(type);
    }
}

bool Token::compareTokens(const Token &token1, const Token &token2)
{
    if (token1.getType() != token2.getType())
    {
        return false;
    }
    else
    {
        return token1.compareValue(token2);
    }
}