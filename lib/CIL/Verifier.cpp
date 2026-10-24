#include "cling/CIL/Verifier.h"

namespace cling::cil {

std::vector<VerificationError> Verifier::verify(const Module& module) const {
    std::vector<VerificationError> errors;
    for (const auto& function : module.functions) {
        const auto count = function.instructions.size();
        for (std::size_t index = 0; index < count; ++index) {
            const auto& instruction = function.instructions[index];
            auto report = [&](std::string message) {
                errors.push_back(VerificationError{function.name, index, std::move(message)});
            };
            switch (instruction.opcode) {
            case Opcode::LoadLocal:
            case Opcode::StoreLocal:
                if (instruction.operand < 0 ||
                    static_cast<unsigned long long>(instruction.operand) >= function.localCount)
                    report("local slot index is outside the function's local frame");
                break;
            case Opcode::Jump:
            case Opcode::JumpIfFalse:
                if (instruction.operand < 0 ||
                    static_cast<unsigned long long>(instruction.operand) >= count)
                    report("branch target is outside the function's instruction range");
                break;
            case Opcode::Call:
                if (instruction.operand < 0 ||
                    static_cast<unsigned long long>(instruction.operand) >= module.functions.size())
                    report("call target is outside the module's function table");
                break;
            case Opcode::ConstantInt:
            case Opcode::ConstantString:
            case Opcode::AddInt:
            case Opcode::SubtractInt:
            case Opcode::MultiplyInt:
            case Opcode::DivideInt:
            case Opcode::CompareEqual:
            case Opcode::CompareLess:
            case Opcode::Return:
            case Opcode::Print:
            case Opcode::Halt:
                break;
            }
        }
        if (function.instructions.empty())
            errors.push_back(VerificationError{function.name, 0, "function has no instructions"});
    }
    return errors;
}

} // namespace cling::cil
