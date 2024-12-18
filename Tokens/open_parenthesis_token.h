#pragma once
#include "token.h"

class OpenParenthesisToken: public Token 
{
    OpenParenthesisToken() : Token(TOpenParenthesis) {}
};