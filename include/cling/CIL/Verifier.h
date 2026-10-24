#pragma once
#include "cling/CIL/Module.h"
#include <string>
#include <vector>
namespace cling::cil {
struct VerificationError {
 std::string functionName;
 std::size_t instructionIndex = 0;
 std::string message;
};
class Verifier {
public:
 [[nodiscard]] std::vector<VerificationError> verify(const Module& module) const;
};
} // namespace cling::cil
