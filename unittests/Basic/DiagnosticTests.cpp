#include "cling/Basic/Diagnostic.h"
#include "cling/Basic/SourceLocation.h"
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>
int main() {
    const cling::SourceBuffer source("diagnostics.cling", "xy");
    cling::DiagnosticEngine diagnostics;
    diagnostics.error(cling::SourceLocation{1, 1, 2}, "unexpected token");
    diagnostics.warning(cling::SourceLocation{2, 1, 3}, "unused value");
    std::ostringstream output;
    diagnostics.print(output, source);
    const auto text = output.str();
    if (!diagnostics.hasErrors() ||
        text.find("diagnostics.cling:1:2: error: unexpected token") == std::string::npos ||
        text.find("diagnostics.cling:1:3: warning: unused value") == std::string::npos) {
        std::cerr << "DiagnosticEngine regression test failed\n";
        return EXIT_FAILURE;
    }
    diagnostics.clear();
    if (diagnostics.hasErrors() || !diagnostics.diagnostics().empty()) {
        std::cerr << "DiagnosticEngine clear() failed\n";
        return EXIT_FAILURE;
    }
    std::cout << "Cling diagnostic tests passed\n";
    return EXIT_SUCCESS;
}
