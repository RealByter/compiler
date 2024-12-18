#include "token.h"

class ConstantToken : public Token
{
private:
    int value;

public:
    ConstantToken(const int &value) : Token(TConstant), value(value) {}
};