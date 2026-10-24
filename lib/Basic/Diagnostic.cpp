#include "cling/Basic/Diagnostic.h"

#include <ostream>
#include <utility>

namespace cling {

void DiagnosticEngine::report(DiagnosticSeverity severity, SourceLocation location,
                              std::string message) {
    diagnostics_.push_back(Diagnostic{severity, location, std::move(message)});
}

void DiagnosticEngine::error(SourceLocation location, std::string message) {
    report(DiagnosticSeverity::Error, location, std::move(message));
}

void DiagnosticEngine::warning(SourceLocation location, std::string message) {
    report(DiagnosticSeverity::Warning, location, std::move(message));
}

void DiagnosticEngine::note(SourceLocation location, std::string message) {
    report(DiagnosticSeverity::Note, location, std::move(message));
}

bool DiagnosticEngine::hasErrors() const noexcept {
    for (const auto& diagnostic : diagnostics_) {
        if (diagnostic.severity == DiagnosticSeverity::Error) {
            return true;
        }
    }
    return false;
}

const std::vector<Diagnostic>& DiagnosticEngine::diagnostics() const noexcept {
    return diagnostics_;
}

void DiagnosticEngine::print(std::ostream& output, const SourceBuffer& source) const {
    for (const auto& diagnostic : diagnostics_) {
        const char* label = "error";
        if (diagnostic.severity == DiagnosticSeverity::Warning) {
            label = "warning";
        } else if (diagnostic.severity == DiagnosticSeverity::Note) {
            label = "note";
        }
        output << source.name() << ':' << diagnostic.location.line << ':'
               << diagnostic.location.column << ": " << label << ": "
               << diagnostic.message << '\n';
    }
}

void DiagnosticEngine::clear() noexcept { diagnostics_.clear(); }

} // namespace cling
