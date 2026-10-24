#pragma once
#include <cstdint>
#include <string>
namespace cling::cil {
enum class Opcode : std::uint8_t {
 ConstantInt, ConstantString, LoadLocal, StoreLocal, AddInt, SubtractInt,
 MultiplyInt, DivideInt, CompareEqual, CompareLess, Jump, JumpIfFalse,
 Call, Return, Print, Halt
};
struct Instruction {
 Opcode opcode = Opcode::Halt;
 std::int64_t operand = 0;
 std::string text;
};
} // namespace cling::cil
