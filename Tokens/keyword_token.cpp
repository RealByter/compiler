#include "keyword_token.h"

bool KeywordToken::compareValue(const Token& other) const
{
    const KeywordToken* constToken = dynamic_cast<const KeywordToken*>(&other);
    return constToken && this->value == constToken->value;
}