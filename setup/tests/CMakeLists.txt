cmake_minimum_required(VERSION 3.10)

project(cjson_rust_tests LANGUAGES C)

enable_testing()

if(NOT IS_ABSOLUTE "${CJSON_RUST_LIBRARY}")
    message(FATAL_ERROR "CJSON_RUST_LIBRARY must name the absolute path to the translated Rust library")
endif()
if(NOT EXISTS "${CJSON_RUST_LIBRARY}")
    message(FATAL_ERROR "Translated Rust library not found: ${CJSON_RUST_LIBRARY}")
endif()

add_library(cjson SHARED IMPORTED GLOBAL)
set_target_properties(cjson PROPERTIES
    IMPORTED_LOCATION "${CJSON_RUST_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_SOURCE_DIR}/..")

get_filename_component(CJSON_RUST_LIBRARY_DIR "${CJSON_RUST_LIBRARY}" DIRECTORY)
set(CMAKE_BUILD_RPATH "${CJSON_RUST_LIBRARY_DIR}")

add_library(unity STATIC unity/unity.c)
target_include_directories(unity PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/unity")

file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/inputs")
file(GLOB test_inputs "inputs/*")
file(COPY ${test_inputs} DESTINATION "${CMAKE_CURRENT_BINARY_DIR}/inputs")

# These upstream tests exercise the installed cJSON API and public cJSON
# structure layout. The former white-box parser/printer tests have been adapted
# to reach the same behavior through exported parse and print entry points.
set(api_tests
    parse_number
    parse_hex4
    parse_string
    parse_array
    parse_object
    parse_value
    print_string
    print_number
    print_array
    print_object
    print_value
    parse_examples
    parse_with_opts
    compare_tests
    cjson_add
    readme_examples
    minify_tests
    misc_tests)

foreach(api_test ${api_tests})
    add_executable("${api_test}" "${api_test}.c")
    target_compile_definitions("${api_test}" PRIVATE CJSON_TEST_PUBLIC_API_ONLY=1)
    target_link_libraries("${api_test}" PRIVATE cjson unity m)
    add_test(NAME "${api_test}" COMMAND "${api_test}")
endforeach()

add_custom_target(check
    COMMAND "${CMAKE_CTEST_COMMAND}" --output-on-failure
    DEPENDS ${api_tests})
