#include "cling/Basic/SourceLocation.h"
#include <cstdlib>
#include <iostream>
int main() {
    const cling::SourceBuffer source("locations.cling", "a\nbc");
    const auto first = source.locationAt(0);
    const auto secondLine = source.locationAt(2);
    const auto end = source.locationAt(4);
    if (first.line != 1 || first.column != 1 ||
        secondLine.line != 2 || secondLine.column != 1 ||
        end.line != 2 || end.column != 3 || source.at(4) != '\0') {
        std::cerr << "SourceLocation regression test failed\n";
        return EXIT_FAILURE;
    }
    std::cout << "Cling source location tests passed\n";
    return EXIT_SUCCESS;
}
