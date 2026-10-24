# Diagnostics

Diagnostics are structured data, not preformatted strings. Each record carries a severity, source location, and message. The driver decides where to print diagnostics, while individual compiler phases report errors through the shared `DiagnosticEngine`.

## Rules

- User mistakes should produce source-located errors instead of process crashes.
- Warnings must not be counted as errors.
- Error recovery should continue when doing so is safe and useful.
- Diagnostics should describe the problem and, when practical, explain the expected form.
- Tests should assert severity, source location, and stable message fragments rather than depending on unrelated formatting.

The current implementation supports error, warning, and note severities and renders locations as `path:line:column`.
