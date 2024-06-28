通常在使用 `cxx` 时不需要查看这些生成的代码，但出于教学目的，可以了解这些代码的生成过程

## Rust生成的代码
1. 安装 `cargo-expand` 工具，以便查看过程宏（procedural macro）的展开结果：
```bash
cargo install cargo-expand
```


2. 使用 `cargo expand` 命令展开 ffi 模块的宏：
```bash
cargo expand ::ffi
```


## C++ 生成的代码
`cxx_build`  会将所有生成的 `C++` 代码链接到 Cargo 的目标目录下，即 `target/cxxbridge/`

1. 查看生成的 C++ 代码文件结构：
```bash
exa -T target/cxxbridge/
```
2. 输出的目录结构可能如下：
```bash
target/cxxbridge
├── cxx-demo
│  └── src
│     ├── main.rs.cc -> ../../../debug/build/cxx-demo-11c6f678ce5c3437/out/cxxbridge/sources/cxx-demo/src/main.rs.cc
│     └── main.rs.h -> ../../../debug/build/cxx-demo-11c6f678ce5c3437/out/cxxbridge/include/cxx-demo/src/main.rs.h
└── rust
   └── cxx.h -> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cxx-1.0.0/include/cxx.h
```

这些文件中会包含在你的语言边界中任何 CXX Rust 类型（例如 `rust::Slice<T>` 对应于 `&[T]`）的声明或模板，以及对应于你定义的 `extern` 函数的 `extern "C"` 签名。





# 独立的 CXX C++ 代码生成器
使用独立的可执行文件形式的 CXX C++ 代码生成器，它会将生成的代码输出到标准输出（stdout）
1. 安装 `cxxbridge-cmd` 工具：
```bash
cargo install cxxbridge-cmd
```

2. 使用 `cxxbridge` 命令生成代码：
```bash
cxxbridge src/main.rs
```