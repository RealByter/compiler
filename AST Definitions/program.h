#include "function_definition.h"

class Program
{
private:
    FunctionDefinition function_definition;

public:
    Program(const FunctionDefinition &function_definition) : function_definition(function_definition) {}
};