#pragma once
#include "token.h"

enum KeywordType
{
    KInt,
    KVoid,
    KReturn,
};

class KeywordToken : public Token
{
private:
    KeywordType value;

public:
    KeywordToken(const KeywordType &type) : Token(TKeyword), value(value) {}

    virtual bool compareValue(const Token& other) const override;
};