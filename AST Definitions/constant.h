#pragma once
#include "expression.h"
#include "../Tokens/constant_token.h"

class Constant : public Expression
{
public:
    ConstantToken constant;
    Constant(const ConstantToken &constant) : constant(constant) {}
};