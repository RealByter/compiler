#include <queue>
#include "Tokens/token.h"
#include "AST Definitions/program.h"

Program parse_program(std::queue<std::unique_ptr<Token>>& tokens);