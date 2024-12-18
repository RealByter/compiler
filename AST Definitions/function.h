#include "function_definition.h"
#include "statement.h"
#include "../Tokens/identifier_token.h"

class Function : public FunctionDefinition
{
private:
    IdentifierToken identifier;
    Statement statement;

public:
    Function(const IdentifierToken &identifier, const Statement &statement) : identifier(identifier), statement(statement) {}
};