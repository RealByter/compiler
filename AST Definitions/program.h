#pragma once
#include "function.h"

class Program
{
private:
    Function function_definition;

public:
    Program(const Function &function_definition) : function_definition(function_definition) {}
};