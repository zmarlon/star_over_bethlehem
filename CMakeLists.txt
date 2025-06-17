cmake_minimum_required(VERSION 3.31)
project(star_over_bethlehem)

option(SOB_RUNTIME_SHADER_COMPILATION "Enable runtime shader compilation support" ON)

set(CMAKE_CXX_STANDARD 26)

file(GLOB_RECURSE CORE_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/base/*.cpp)

if(WIN32)
    add_definitions(-DRHI_D3D12_SUPPORTED)
    file(GLOB_RECURSE D3D12_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/d3d12/*.cpp)
    list(APPEND CORE_SOURCE_FILES ${D3D12_SOURCE_FILES})

    file(GLOB_RECURSE VULKAN_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/vulkan/*.cpp)
    list(APPEND CORE_SOURCE_FILES ${VULKAN_SOURCE_FILES})

elseif(APPLE)
    file(GLOB_RECURSE METAL_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/metal/*.cpp)
    list(APPEND CORE_SOURCE_FILES ${METAL_SOURCE_FILES})

    file(GLOB_RECURSE VULKAN_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/vulkan/*.cpp)
    list(APPEND CORE_SOURCE_FILES ${VULKAN_SOURCE_FILES})

elseif(UNIX)
    file(GLOB_RECURSE VULKAN_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/vulkan/*.cpp)
    list(APPEND CORE_SOURCE_FILES ${VULKAN_SOURCE_FILES})
endif()

list(APPEND CORE_SOURCE_FILES ${CMAKE_SOURCE_DIR}/src/star_over_bethlehem.cpp)

add_library(star_over_bethlehem STATIC ${CORE_SOURCE_FILES})

include_directories(${CMAKE_SOURCE_DIR}/include)

include_directories(${CMAKE_SOURCE_DIR}/dependencies/include/base)

if(APPLE)
    include_directories(${CMAKE_SOURCE_DIR}/dependencies/include/apple)
endif()

if(SOB_RUNTIME_SHADER_COMPILATION)
    target_compile_definitions(star_over_bethlehem PRIVATE SOB_RUNTIME_SHADER_COMPILATION)
endif()

if(WIN32)
    target_compile_definitions(star_over_bethlehem PRIVATE SOB_D3D12_SUPPORTED SOB_VULKAN_SUPPORTED)
elseif(APPLE)
    target_compile_definitions(star_over_bethlehem PRIVATE SOB_METAL_SUPPORTED SOB_VULKAN_SUPPORTED)
    target_link_libraries(star_over_bethlehem "-framework Foundation" "-framework Metal" "-framework QuartzCore" "/usr/local/lib/libdxcompiler.dylib")
elseif(UNIX)
    target_compile_definitions(star_over_bethlehem PRIVATE SOB_VULKAN_SUPPORTED)
endif()

add_executable(star_over_bethlehem_example ${CMAKE_SOURCE_DIR}/example/main.cpp)
target_link_libraries(star_over_bethlehem_example star_over_bethlehem)