#include <iostream>
#include <string>

class LexerException : public std::exception
{
public:
    explicit LexerException(const std::string &message) : msg(message) {}
    const char *what() const noexcept override { return msg.c_str(); }

private:
    std::string msg;
};
