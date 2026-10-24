#pragma once
#include "cling/Basic/Diagnostic.h"
#include "cling/Sema/Type.h"
#include <string>
#include <unordered_map>
#include <vector>
namespace cling {
class Program;
struct FunctionSignature {
 std::vector<Type> parameters;
 Type result;
};
class TypeChecker {
public:
 explicit TypeChecker(DiagnosticEngine& diagnostics);
 [[nodiscard]] bool check(const Program& program);
 [[nodiscard]] const std::unordered_map<std::string, FunctionSignature>& functions() const noexcept;
private:
 DiagnosticEngine& diagnostics_;
 std::unordered_map<std::string, FunctionSignature> functions_;
};
} // namespace cling
