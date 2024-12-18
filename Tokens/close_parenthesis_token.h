#pragma once
#include "token.h"

class CloseParenthesisToken: public Token 
{
    CloseParenthesisToken() : Token(TCloseParenthesis) {}
};