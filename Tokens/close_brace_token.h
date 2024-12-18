#include "token.h"

class CloseBraceToken: public Token 
{
    CloseBraceToken() : Token(TCloseBrace) {}
};