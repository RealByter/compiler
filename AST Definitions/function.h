#pragma once
#include "function_definition.h"
#include "statement.h"
#include "identifier.h"

class Function : public FunctionDefinition
{
private:
    Identifier identifier;
    Statement statement;

public:
    Function(const Identifier &identifier, const Statement &statement) : identifier(identifier), statement(statement) {}
};