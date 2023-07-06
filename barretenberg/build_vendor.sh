#!/bin/sh

export CC=/usr/local/opt/llvm/bin/clang
export CXX=/usr/local/opt/llvm/bin/clang++
export LDFLAGS="-L/usr/local/opt/llvm/lib"
export CPPFLAGS="-I/usr/local/opt/llvm/include"

git submodule update --remote

cmake ./vendor-barretenberg/cpp
cd ./vendor-barretenberg/cpp && make all