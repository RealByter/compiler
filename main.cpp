#include <cstdlib>
#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include "lexer.h"
#include "parser.h"
#include "LexerException.h"

int preprocess(const std::string &input_file, const std::string &preprocessed_file);
int generate_assembly(const std::string &preprocessed_file, const std::string &assembly_file);
int compile_executable(const std::string &assembly_file, const std::string &output_file);

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
		std::queue<std::unique_ptr<Token>> tokens = tokenize(input);
		Program program = parse_program(tokens);
		// for (const std::unique_ptr<Token>& token : tokens)
		// {
		// 	std::cout << token.get()->getType() << ", ";
		// }
		// std::cout << std::endl;
	}
	catch (LexerException exp)
	{
		std::cout << exp.what() << std::endl;
		return -1;
	}

	std::string preprocessed_file = base_name + ".i";
	std::string assembly_file = base_name + ".s";
	std::string executable_file = base_name + ".out";

	if (preprocess(input_file, preprocessed_file))
		return -1;
	if (stop_at == "--parse")
		return 0;
	if (generate_assembly(preprocessed_file, assembly_file))
		return -1;
	if (stop_at == "--codegen")
		return 0;
	if (compile_executable(assembly_file, executable_file))
		return -1;

	return 0;
}

int preprocess(const std::string &input_file, const std::string &preprocessed_file)
{
	return system((std::string("gcc -E -P ") + input_file + " -o " + preprocessed_file).c_str());
}

int generate_assembly(const std::string &preprocessed_file, const std::string &assembly_file)
{
	int err = system((std::string("gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none ") + preprocessed_file + " -o " + assembly_file).c_str());
	system((std::string("rm ") + preprocessed_file).c_str());
	return err;
}

int compile_executable(const std::string &assembly_file, const std::string &executable_file)
{
	int err = system((std::string("gcc ") + assembly_file + " -o " + executable_file).c_str());
	system((std::string("rm ") + assembly_file).c_str());
	return err;
}