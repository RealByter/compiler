#include <iostream>
#include <vector>
#include <variant>
#include "Tokens/token.h"

std::vector<std::unique_ptr<Token>> tokenize(std::string input);