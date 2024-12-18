#pragma once
#include <string>

class Identifier
{
private:
    std::string value;

public:
    Identifier(const std::string &value) : value(value) {}
};