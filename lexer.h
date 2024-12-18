#include <iostream>
#include <vector>

enum TokenType
{
    TIdentifier,
    TConstant,
    TKeyword,
    TOpenParenthesis,
    TCloseParenthesis,
    TOpenBrace,
    TCloseBrace,
    TSemicolon
};

enum KeywordType {
    KInt,
    KVoid,
    KReturn,
};

struct Token
{
    TokenType token;
    union
    {
        size_t constant;
        std::string identifier;
        KeywordType keyword;
    };

    ~Token() { // check why its needed and why simply changing to class doesnt work

    }
};

std::vector<Token> tokenize(std::string input);