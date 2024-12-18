#include "lexer.h"
#include <regex>

const std::regex REG_IDENTIFIER("[a-zA-Z_]\\w*\\b");
const std::regex REG_CONSTANT("[0-9]+\\b");
const std::regex REG_KEYWORD("(?!)"); // temp fix until i find a clean way to connect between the regex and the type
const std::regex REG_OPEN_PARENTHESIS("\\(");
const std::regex REG_CLOSE_PARENTHESIS("\\)");
const std::regex REG_OPEN_BRACE("\\{");
const std::regex REG_CLOSE_BRACE("\\}");
const std::regex REG_SEMICOLON(";");
const std::regex REGEXS[] = {REG_IDENTIFIER, REG_CONSTANT, REG_KEYWORD, REG_OPEN_PARENTHESIS, REG_CLOSE_PARENTHESIS, REG_OPEN_BRACE, REG_CLOSE_BRACE, REG_SEMICOLON};
const size_t REGEX_COUNT = sizeof(REGEXS) / sizeof(std::regex);

std::vector<Token> tokenize(std::string input)
{
    std::vector<Token> tokens;

    while (!input.empty())
    {
        size_t start = input.find_first_not_of(" \t\r\n");
        input = (start == std::string::npos) ? "" : input.substr(start);
        if (input.empty())
        {
            break;
        }

        TokenType token_type;
        std::string longest_match;
        size_t longest_length = 0;
        std::smatch match;
        for (int i = 0; i < REGEX_COUNT; i++)
        {
            if (std::regex_search(input, match, REGEXS[i]))
            {
                if (match.length() > 0 && match.position(0) == 0 && match[0].length() >= longest_length)
                {
                    longest_match = input.substr(0, match.length(0));
                    longest_length = longest_match.length();
                    token_type = TokenType(i);
                }
            }
        }
        if (longest_length == 0)
        {
            throw std::string("Invalid token: ") + input; // temporary
        }

        input = input.substr(longest_length);
        std::cout << "longest: " << longest_match << ";" << longest_length << std::endl;
        std::cout << "remaining: " << input << std::endl;

        Token token;
        token.type = token_type;
        switch (token_type)
        {
        case TIdentifier:
            if (longest_match == "int")
            {
                token.type = TKeyword;
                token.value = KInt;
            }
            else if (longest_match == "void")
            {
                token.type = TKeyword;
                token.value = KVoid;
            }
            else if (longest_match == "return")
            {
                token.type = TKeyword;
                token.value = KReturn;
            }
            else
            {
                token.value = longest_match;
            }
            break;
        case TConstant:
            token.value = (size_t)atoi(longest_match.c_str());
            break;
        }

        tokens.push_back(token);
    }

    return tokens;
}