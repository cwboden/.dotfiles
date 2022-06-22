cmake_minimum_required(VERSION 3.9)
project(tic_tac_toe)

set(LINKER_FLAGS "-lSDL2 -lSDL2_image")
set(CMAKE_EXE_LINKER_FLAGS  "${CMAKE_EXE_LINKER_FLAGS} ${LINKER_FLAGS}")

set(CMAKE_CXX_STANDARD 17)

add_executable(tic_tac_toe main.cpp Tile.cpp Tile.h Texture.cpp Texture.h)
