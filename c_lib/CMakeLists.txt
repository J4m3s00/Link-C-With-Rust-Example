cmake_minimum_required(VERSION 3.15)
project(LinkCWithRust)

set(CMAKE_CXX_STANDARD 14)

add_library(CLib STATIC CLib.cpp)