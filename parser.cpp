#include "./AST Definitions/program.h"
#include "./AST Definitions/function.h"
#include "./AST Definitions/statement.h"
#include "./AST Definitions/expression.h"
#include "./AST Definitions/constant.h"
#include "./AST Definitions/return.h"
#include "./Tokens/tokens.h"
#include "LexerException.h"
#include <sstream>
#include <queue>
#include <memory>

Function parse_function(std::queue<std::unique_ptr<Token>> &tokens);
Return parse_return(std::queue<std::unique_ptr<Token>> &tokens);
Constant parse_expression(std::queue<std::unique_ptr<Token>> &tokens);
Identifier parse_identifier(std::queue<std::unique_ptr<Token>> &tokens);
bool expect(const std::unique_ptr<Token> &expected, std::queue<std::unique_ptr<Token>> &tokens);

Program parse_program(std::queue<std::unique_ptr<Token>> &tokens)
{
    return Program(parse_function(tokens));
}

Function parse_function(std::queue<std::unique_ptr<Token>> &tokens)
{
    expect(Token::createToken(TKeyword, "int"), tokens);
    Identifier identifier = parse_identifier(tokens);
    expect(Token::createToken(TOpenParenthesis, ""), tokens);
    expect(Token::createToken(TKeyword, "void"), tokens);
    expect(Token::createToken(TCloseParenthesis, ""), tokens);
    expect(Token::createToken(TOpenBrace, ""), tokens);
    Statement statement = parse_return(tokens);
    expect(Token::createToken(TCloseBrace, ""), tokens);
    return Function(identifier, statement);
}

Return parse_return(std::queue<std::unique_ptr<Token>> &tokens)
{
    expect(Token::createToken(TKeyword, "return"), tokens);
    Expression return_val = parse_expression(tokens);
    expect(Token::createToken(TSemicolon, ""), tokens);
    return Return(return_val);
}

Constant parse_expression(std::queue<std::unique_ptr<Token>> &tokens)
{
    const std::unique_ptr<Token> &token = tokens.front();
    if (token.get()->getType() != TConstant)
    {
        throw LexerException("Invalid token. Expected a constant got: " + token.get()->getType());
    }
    Constant constant = Constant(dynamic_cast<ConstantToken *>(token.get())->getValue());
    tokens.pop();
    return constant;
}

Identifier parse_identifier(std::queue<std::unique_ptr<Token>> &tokens)
{
    const std::unique_ptr<Token> &token = tokens.front();
    if (token.get()->getType() != TIdentifier)
    {
        throw LexerException("Invalid token. Expected an identifier got: " + token.get()->getType());
    }
    Identifier identifier = Identifier(dynamic_cast<IdentifierToken *>(token.get())->getValue());
    tokens.pop();
    return identifier;
}

bool expect(const Token expected, std::queue<std::unique_ptr<Token>> &tokens)
{
    if(tokens.empty())
    {
        throw LexerException("Unexpected end of tokens");
    }

    const std::unique_ptr<Token> &token = tokens.front();
    if (Token::compareTokens(*token, expected))
    {
        tokens.pop();
        return true;
    }
    else
    {
        std::stringstream stream;
        stream << "Invalid token. Expected: ";
        stream << expected.getType();
        stream << " got: ";
        stream << token.get()->getType();
        throw LexerException(stream.str());
    }
}