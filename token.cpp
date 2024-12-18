#include "token.h"
#include <unordered_map>

const std::unordered_map<std::string, KeywordType> KEYWORDS = {
    {"int", KInt},
    {"void", KVoid},
    {"return", KReturn}};

Token::Token(const TokenType& type, std::string value) : type(type)
{
    switch (type)
        {
        case TIdentifier:
            if (KEYWORDS.find(value) != KEYWORDS.end())
            {
                this->type = TKeyword;
                this->value = KEYWORDS.at(value);
            }
            else
            {
                this->value = value;
            }
            break;
        case TConstant:
            this->value = (size_t)atoi(value.c_str());
            break;
        }
}