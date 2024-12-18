#include "identifier_token.h"

bool IdentifierToken::compareValue(const Token& other) const
{
    const IdentifierToken* constToken = dynamic_cast<const IdentifierToken*>(&other);
    return constToken && this->value == constToken->value;
}

std::string IdentifierToken::getValue() const
{
    return this->value;
}