#include "cling/CIL/Module.h"
#include "cling/CIL/Verifier.h"
#include <cstdlib>
#include <iostream>
#include <string>
#include <utility>
int main() {
    cling::cil::Module module;
    module.sourceName = "verifier-test.cling";
    cling::cil::Function function;
    function.name = "main";
    function.localCount = 1;
    function.instructions.push_back({cling::cil::Opcode::LoadLocal, 0, {}});
    function.instructions.push_back({cling::cil::Opcode::Jump, 9, {}});
    function.instructions.push_back({cling::cil::Opcode::Return, 0, {}});
    module.functions.push_back(std::move(function));
    const auto errors = cling::cil::Verifier{}.verify(module);
    if (errors.size() != 1 || errors.front().message.find("branch target") == std::string::npos) {
        std::cerr << "CIL verifier failed to reject an invalid branch target\n";
        return EXIT_FAILURE;
    }
    std::cout << "Cling CIL verifier tests passed\n";
    return EXIT_SUCCESS;
}
