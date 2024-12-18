#include "token.h"
#include <unordered_map>
#include "tokens.h"

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
    default:
        return std::make_unique<Token>(type);
    }
}