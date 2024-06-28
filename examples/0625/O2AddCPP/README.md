# 添加 C++ 代码
## CXX 与 Cargo 的集成中
所有的 #include 路径默认以 crate 名称开头（如果 crate 没有明确选择其他路径；参见第 5 章的 CFG.include_prefix）

`include!("cxx-demo/include/blobstore.h")`
将把 C++ 头文件放在 Rust crate 内的相对路径 `include/blobstore.h` 中。

如果你的 crate 在 `Cargo.toml` 中的 name 字段中命名为其他名称，你需要在本教程中的所有地方用该名称替换 `cxx-demo`