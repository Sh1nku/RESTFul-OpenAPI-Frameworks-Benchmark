cmake_minimum_required(VERSION 3.16.3)

project (App)

file(GLOB_RECURSE HEADERS "src/*.h")
file(GLOB_RECURSE SOURCES "src/*.cpp")

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup(TARGETS)
set( PROJECT_LINK_LIBS CONAN_PKG::oatpp CONAN_PKG::oatpp-swagger)
get_filename_component(OATPP_RES_BASE "${CONAN_INCLUDE_DIRS_OATPP-SWAGGER}/.." ABSOLUTE)

add_definitions(
        ## define path to swagger-ui static resources folder
        -DOATPP_SWAGGER_RES_PATH="${OATPP_RES_BASE}/bin/oatpp-swagger/res"
)

add_executable(App ${SOURCES} ${HEADERS})
target_link_libraries(App PRIVATE ${PROJECT_LINK_LIBS} )

set_property(TARGET App PROPERTY CXX_STANDARD 17)