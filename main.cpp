#include <cstdlib>
#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include "lexer.h"

int main(int argc, char *argv[])
{
	int err = 0;
	std::string stop_at;
	if (argc != 2 && argc != 3)
	{
		std::cout << "Invalid args. Should be: \"program.exe <input_file> [--lex|--parse|--codegen]\"" << std::endl;
		return -1;
	}
	if (argc == 3)
	{
		stop_at = std::string(argv[2]);
	}

	std::string input_file = argv[1];
	size_t dot_pos = input_file.find_last_of('.');
	std::string base_name = (dot_pos == std::string::npos) ? input_file : input_file.substr(0, dot_pos);

	std::ifstream file(input_file);
	if (!file)
	{
		return -1;
	}

	std::ostringstream buffer;
	buffer << file.rdbuf();
	std::string input = buffer.str();

	try
	{
		std::vector<Token> tokens = tokenize(input);
		for(Token token : tokens)
		{
			switch(token.type)
			{
				case TIdentifier: std::cout << std::get<std::string>(token.value) << std::endl; break;
				case TConstant: std::cout << std::get<size_t>(token.value) << std::endl; break;
				case TKeyword: std::cout << "Type: " << std::get<KeywordType>(token.value) << std::endl; break;
				case TOpenParenthesis: std::cout << "Open Parenthesis" << std::endl; break;
				case TCloseParenthesis: std::cout << "Close Parenthesis" << std::endl; break;
				case TOpenBrace: std::cout << "Open Brace" << std::endl; break;
				case TCloseBrace: std::cout << "Close Brace" << std::endl; break;
				case TSemicolon: std::cout << "Semicolon" << std::endl; break;
			}
		}
	}
	catch (std::string exp)
	{
		std::cout << exp << std::endl;
		return -1;
	}

	std::string preprocessed_file = base_name + ".i";
	err = system((std::string("gcc -E -P ") + input_file + " -o " + preprocessed_file).c_str());
	if (err)
	{
		return -1;
	}
	if (!stop_at.empty() && stop_at == "--parse")
	{
		return 0;
	}

	std::string assembly_file = base_name + ".s";
	err = system((std::string("gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none ") + preprocessed_file + " -o " + assembly_file).c_str());
	system((std::string("rm ") + preprocessed_file).c_str());
	if (err)
	{
		return -1;
	}
	if (!stop_at.empty() && stop_at == "--codegen")
	{
		return 0;
	}

	std::string executable_file = base_name + ".out";
	err = system((std::string("gcc ") + assembly_file + " -o " + executable_file).c_str());
	system((std::string("rm ") + assembly_file).c_str());
	if (err)
	{
		return -1;
	}
	return 0;
}
