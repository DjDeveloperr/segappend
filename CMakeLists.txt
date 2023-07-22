cmake_minimum_required(VERSION 3.15)

set(NAME segappend)
set(VERSION 0.1.0)

project(segappend)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -pedantic -O3")

include_directories(
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)

add_library(${NAME} STATIC
  src/segappend.cc)

add_executable(${NAME}_util
  src/main.cc
  src/segappend.cc
)
