cmake_minimum_required(VERSION 3.16)
project(Cling LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(cling_parse STATIC
    lib/Parse/Lexer.cpp
)
target_include_directories(cling_parse PUBLIC include)
target_compile_options(cling_parse PRIVATE -Wall -Wextra -Wpedantic)

add_executable(cling src/cling.cpp)
target_link_libraries(cling PRIVATE cling_parse)
target_compile_options(cling PRIVATE -Wall -Wextra -Wpedantic)

add_executable(cling_lexer_tests test/lexer_tests.cpp)
target_link_libraries(cling_lexer_tests PRIVATE cling_parse)
target_compile_options(cling_lexer_tests PRIVATE -Wall -Wextra -Wpedantic)

install(TARGETS cling RUNTIME DESTINATION bin)

enable_testing()
add_test(NAME lexer COMMAND cling_lexer_tests)
add_test(NAME hello COMMAND cling ${CMAKE_CURRENT_SOURCE_DIR}/examples/hello.cling)
set_tests_properties(hello PROPERTIES PASS_REGULAR_EXPRESSION "Hello, World!")
add_test(NAME control_flow COMMAND cling ${CMAKE_CURRENT_SOURCE_DIR}/examples/control_flow.cling)
set_tests_properties(control_flow PROPERTIES PASS_REGULAR_EXPRESSION "CIL execution succeeded")
