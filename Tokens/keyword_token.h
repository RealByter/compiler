#include "token.h"

enum KeywordType
{
    KInt,
    KVoid,
    KReturn,
};

class KeywordToken : public Token
{
private:
    KeywordType type;

public:
    KeywordToken(const KeywordType &type) : Token(TKeyword), type(type) {}
};