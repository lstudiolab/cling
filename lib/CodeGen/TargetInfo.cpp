#include "cling/CodeGen/TargetInfo.h"
namespace cling::codegen {
std::string_view architectureName(Architecture architecture) noexcept {
 switch (architecture) {
 case Architecture::Host: return "host";
 case Architecture::X86: return "x86";
 case Architecture::X86_64: return "x86_64";
 case Architecture::ARM: return "arm";
 case Architecture::AArch64: return "aarch64";
 case Architecture::RISC_V: return "risc-v";
 case Architecture::WebAssembly: return "wasm";
 }
 return "unknown";
}
std::string_view objectFormatName(ObjectFormat format) noexcept {
 switch (format) {
 case ObjectFormat::Host: return "host";
 case ObjectFormat::ELF: return "ELF";
 case ObjectFormat::MachO: return "Mach-O";
 case ObjectFormat::COFF: return "COFF";
 case ObjectFormat::Wasm: return "WebAssembly";
 }
 return "unknown";
}
} // namespace cling::codegen
