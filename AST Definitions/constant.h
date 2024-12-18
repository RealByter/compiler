#include "expression.h"

class Constant : public Expression
{
public:
    int value;
    Constant(const int &value) : value(value) {}
};