#pragma once
#include "cling/CIL/Instruction.h"
#include <string>
#include <vector>
namespace cling::cil {
struct Function {
 std::string name;
 unsigned parameterCount = 0;
 unsigned localCount = 0;
 std::vector<Instruction> instructions;
};
struct Module {
 std::string sourceName;
 std::vector<Function> functions;
};
} // namespace cling::cil
