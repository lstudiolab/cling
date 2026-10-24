#include "cling/AST/ASTContext.h"
#include "cling/AST/Decl.h"
#include "cling/AST/Expr.h"
#include "cling/AST/Program.h"
#include "cling/AST/Stmt.h"
#include "cling/CIL/Instruction.h"
#include "cling/CIL/Module.h"
#include "cling/CIL/Verifier.h"
#include "cling/CodeGen/TargetInfo.h"
#include "cling/Driver/Options.h"
#include "cling/Optimizer/PassManager.h"
#include "cling/Parse/Parser.h"
#include "cling/Sema/Type.h"
#include "cling/Sema/TypeChecker.h"
int main() {
    cling::ASTContext context;
    auto* literal = context.make<cling::IntegerExpr>(42);
    cling::Program program;
    cling::cil::Module module;
    cling::PassManager passes;
    passes.addPassName("header-smoke");
    return literal->kind() == cling::ASTNodeKind::IntegerExpr &&
           program.functions.empty() && module.functions.empty() &&
           passes.passNames().size() == 1 ? 0 : 1;
}
