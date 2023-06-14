# Polynomial Commitment Benchmark

## Testing on iOS

1. Ensure `cbindgen` is installed in the system
2. Ensure that cargo target has `aarch64-apple-ios` and/or `aarch64-apple-ios-sim` (if doing simulation)
3. Clone https://github.com/gswirski/recmo-pc-bench/tree/eth-stark
4. `git submodule init && git submodule update`
5. `cd eth-stark/vendor-eth-stark`
6. `./install_deps_arm64.sh`
7. `./compile_arm64.sh`
8. Open ios/App/App.xcodeproj in Xcode and change development team.
9. Run.
