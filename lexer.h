#include <iostream>
#include <vector>
#include <variant>

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

enum KeywordType
{
    KInt,
    KVoid,
    KReturn,
};

class Token
{
public:
    TokenType type;
    std::variant<size_t, std::string, KeywordType> value;

    Token() {
        
    }

    ~Token() {

    }
};

std::vector<Token> tokenize(std::string input);