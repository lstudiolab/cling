#include "cling/Sema/Type.h"
namespace cling {
std::string_view typeName(Type type) noexcept {
 switch (type.kind) {
 case TypeKind::Error: return "<error>";
 case TypeKind::Void: return "Void";
 case TypeKind::Bool: return "Bool";
 case TypeKind::Int: return "Int";
 case TypeKind::String: return "String";
 case TypeKind::Function: return "Function";
 case TypeKind::Unknown: return "<unknown>";
 }
 return "<unknown>";
}
} // namespace cling
