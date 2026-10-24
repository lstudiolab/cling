#pragma once
#include <string>
#include <vector>
namespace cling {
struct OptimizationRemark {
 std::string passName;
 std::string message;
};
class PassManager {
public:
 void addPassName(std::string name);
 [[nodiscard]] const std::vector<std::string>& passNames() const noexcept;
 [[nodiscard]] const std::vector<OptimizationRemark>& remarks() const noexcept;
private:
 std::vector<std::string> passNames_;
 std::vector<OptimizationRemark> remarks_;
};
} // namespace cling
