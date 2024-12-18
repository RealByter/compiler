#include "token.h"

class Semicolon : public Token
{
public:
    Semicolon() : Token(TSemicolon) {}
};