cmake_minimum_required(VERSION 3.20)
project(Cling VERSION 0.2.0 LANGUAGES CXX)

option(CLING_BUILD_TESTS "Build C++ tests" ON)
option(CLING_WARNINGS_AS_ERRORS "Treat warnings as errors" OFF)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(ClingWarnings)

add_library(ClingBasic
    lib/Basic/Diagnostic.cpp
    lib/Basic/SourceLocation.cpp
)
target_include_directories(ClingBasic PUBLIC include)
cling_set_warnings(ClingBasic)

add_library(ClingLex
    lib/Lex/TokenKind.cpp
    lib/Lex/Lexer.cpp
)
target_link_libraries(ClingLex PUBLIC ClingBasic)
target_include_directories(ClingLex PUBLIC include)
cling_set_warnings(ClingLex)

add_library(ClingSema lib/Sema/Type.cpp)
target_link_libraries(ClingSema PUBLIC ClingBasic)
target_include_directories(ClingSema PUBLIC include)
cling_set_warnings(ClingSema)

add_library(ClingOptimizer lib/Optimizer/PassManager.cpp)
target_include_directories(ClingOptimizer PUBLIC include)
cling_set_warnings(ClingOptimizer)

add_library(ClingCodeGen lib/CodeGen/TargetInfo.cpp)
target_include_directories(ClingCodeGen PUBLIC include)
cling_set_warnings(ClingCodeGen)

add_library(ClingAST INTERFACE)
target_include_directories(ClingAST INTERFACE include)

add_executable(cling tools/cling/main.cpp)
target_link_libraries(cling PRIVATE ClingLex ClingBasic)
cling_set_warnings(cling)

if(CLING_BUILD_TESTS)
    enable_testing()
    add_executable(cling-lexer-tests unittests/Lexer/LexerTests.cpp)
    target_link_libraries(cling-lexer-tests PRIVATE ClingLex ClingBasic)
    cling_set_warnings(cling-lexer-tests)
    add_test(NAME ClingLexerTests COMMAND cling-lexer-tests)
endif()
