#include <iostream>
#include <queue>
#include <variant>
#include "Tokens/token.h"

std::queue<std::unique_ptr<Token>> tokenize(std::string input);