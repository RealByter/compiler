#include <variant>
#include <string>

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

    Token(const TokenType& type, std::string value);
};