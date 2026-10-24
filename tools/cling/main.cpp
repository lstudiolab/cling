#include "cling/Basic/Diagnostic.h"
#include "cling/Basic/SourceLocation.h"
#include "cling/Driver/Options.h"
#include "cling/Lex/Lexer.h"
#include "cling/Lex/TokenKind.h"
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
int main(int argc, char** argv) {
 const auto parsed = cling::driver::parseOptions(argc, const_cast<const char* const*>(argv));
 if (!parsed.success()) {
  for (const auto& error : parsed.errors) std::cerr << "error: " << error << '\n';
  std::cerr << cling::driver::usageText(argv[0]);
  return 2;
 }
 if (parsed.options.showHelp) { std::cout << cling::driver::usageText(argv[0]); return 0; }
 if (parsed.options.showVersion) { std::cout << "Cling C++ bootstrap 0.2.0\n"; return 0; }
 if (parsed.options.emit != cling::driver::EmitKind::Tokens) {
  std::cerr << "error: this C++ bootstrap currently implements only '--emit tokens'\n";
  return 2;
 }
 std::ifstream input(parsed.options.input, std::ios::binary);
 if (!input) {
  std::cerr << "error: cannot open source file '" << parsed.options.input.string() << "'\n";
  return 1;
 }
 const std::string contents((std::istreambuf_iterator<char>(input)), std::istreambuf_iterator<char>());
 const cling::SourceBuffer source(parsed.options.input.string(), contents);
 cling::DiagnosticEngine diagnostics;
 cling::Lexer lexer(source, diagnostics);
 for (const auto& token : lexer.lex()) {
  std::cout << token.range.begin.line << ':' << token.range.begin.column << ' '
            << cling::tokenKindName(token.kind) << " \"" << token.text << "\"\n";
 }
 diagnostics.print(std::cerr, source);
 return diagnostics.hasErrors() ? 1 : 0;
}
