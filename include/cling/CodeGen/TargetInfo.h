#pragma once
#include <string>
#include <string_view>
namespace cling::codegen {
enum class Architecture { Host, X86, X86_64, ARM, AArch64, RISC_V, WebAssembly };
enum class ObjectFormat { Host, ELF, MachO, COFF, Wasm };
struct TargetInfo {
 Architecture architecture = Architecture::Host;
 ObjectFormat objectFormat = ObjectFormat::Host;
 std::string triple;
 [[nodiscard]] bool isHostTarget() const noexcept { return architecture == Architecture::Host; }
};
[[nodiscard]] std::string_view architectureName(Architecture architecture) noexcept;
[[nodiscard]] std::string_view objectFormatName(ObjectFormat format) noexcept;
} // namespace cling::codegen
