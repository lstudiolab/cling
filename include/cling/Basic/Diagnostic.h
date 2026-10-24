#pragma once

#include "cling/Basic/SourceLocation.h"

#include <iosfwd>
#include <string>
#include <vector>

namespace cling {

enum class DiagnosticSeverity { Note, Warning, Error };

struct Diagnostic {
    DiagnosticSeverity severity = DiagnosticSeverity::Error;
    SourceLocation location;
    std::string message;
};

class DiagnosticEngine {
public:
    void report(DiagnosticSeverity severity, SourceLocation location, std::string message);
    void error(SourceLocation location, std::string message);
    void warning(SourceLocation location, std::string message);
    void note(SourceLocation location, std::string message);

    [[nodiscard]] bool hasErrors() const noexcept;
    [[nodiscard]] const std::vector<Diagnostic>& diagnostics() const noexcept;
    void print(std::ostream& output, const SourceBuffer& source) const;
    void clear() noexcept;

private:
    std::vector<Diagnostic> diagnostics_;
};

} // namespace cling
