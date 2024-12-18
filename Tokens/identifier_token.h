#include "token.h"

class IdentifierToken : public Token
{
private:
    std::string value;

public:
    IdentifierToken(const std::string& value) : Token(TIdentifier), value(value) {}
};