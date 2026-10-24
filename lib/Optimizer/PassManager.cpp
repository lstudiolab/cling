#include "cling/Optimizer/PassManager.h"
#include <utility>
namespace cling {
void PassManager::addPassName(std::string name) { passNames_.push_back(std::move(name)); }
const std::vector<std::string>& PassManager::passNames() const noexcept { return passNames_; }
const std::vector<OptimizationRemark>& PassManager::remarks() const noexcept { return remarks_; }
} // namespace cling
