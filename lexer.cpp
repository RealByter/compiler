#include "lexer.h"
#include "LexerException.h"
#include <regex>
#include <vector>
#include <unordered_map>
#include <memory>

const std::vector<std::pair<std::regex, TokenType>> TOKEN_PATTERNS = {
    {std::regex("[a-zA-Z_]\\w*\\b"), TIdentifier},
    {std::regex("[0-9]+\\b"), TConstant},
    {std::regex("\\("), TOpenParenthesis},
    {std::regex("\\)"), TCloseParenthesis},
    {std::regex("\\{"), TOpenBrace},
    {std::regex("\\}"), TCloseBrace},
    {std::regex(";"), TSemicolon}};

std::vector<std::unique_ptr<Token>> tokenize(std::string input)
{
    std::vector<std::unique_ptr<Token>> tokens;

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
        for (const auto &[regex, type] : TOKEN_PATTERNS)
        {
            if (std::regex_search(input, match, regex))
            {
                if (match.length() > 0 && match.position(0) == 0 && match[0].length() >= longest_length)
                {
                    longest_match = input.substr(0, match.length(0));
                    longest_length = longest_match.length();
                    token_type = type;
                }
            }
        }

        if (longest_length == 0)
        {
            throw LexerException("Invalid token: " + input);
        }

        input = input.substr(longest_length);

        tokens.push_back(Token::createToken(token_type, longest_match));
    }

    return tokens;
}