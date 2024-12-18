#pragma once
#include "expression.h"
#include "statement.h"

class Return : public Statement
{
public:
    Expression expression;
    Return(const Expression &expression) : expression(expression) {}
};