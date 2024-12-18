#pragma once
#include "token.h"

class OpenBraceToken: public Token 
{
    OpenBraceToken() : Token(TOpenBrace) {}
};