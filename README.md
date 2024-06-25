# 6.24
##  🌟查看所有安装的Rust工具链版本
```bash
rustup toolchain list
```
```bash
rustup toolchain list
# stable-aarch64-unknown-linux-gnu (default)
# nightly-2021-12-05-aarch64-unknown-linux-gnu
# nightly-aarch64-unknown-linux-gnu (override)
# root@e500e0d544ab:~/trading-programming/rust-ffi-new
# root@e500e0d544ab:~/trading-programming/rust-ffi-new-checker# 
rustup toolchain uninstall stable-aarch64-unknown-linux-gnu 
# info: uninstalling toolchain 'stable-aarch64-unknown-linux-gnu'
# info: toolchain 'stable-aarch64-unknown-linux-gnu' uninstalled

rustup toolchain uninstall nightly-aarch64-unknown-linux-gnu
# info: uninstalling toolchain 'nightly-aarch64-unknown-linux-gnu'
```

### 查看每个工具链版本对应的 Cargo 版本：
```bash
rustup toolchain list
rustup run nightly-2021-12-05-aarch64-unknown-linux-gnu cargo --version
# cargo 1.58.0-nightly (294967c53 2021-11-29)
```


### rustup和Cargo的关系
rustup 是 Rust 工具链管理器，用于安装和管理不同版本的 Rust 工具链；cargo 是 Rust 的包管理器和构建工具，对应关系是一对多，一个 Rust 工具链对应一个 Cargo 版本。

### rustup和rustc的关系
查看特定工具链版本的 rustc 版本：
```bash
rustup run nightly-2021-12-05 rustc --version
```
rustup 并❌不支持单独升级 rustc 而不更新工具链版本。rustup 管理整个工具链（包括 rustc、cargo 等工具），只能通过切换工具链版本来更改 rustc 的版本。
### 一般是找Cargo对应的rustup，还是rustup对应的Cargo?
在网上查找时，一般是找 rustup 对应的 Cargo 版本。也就是说，你先确定要使用的 Rust 工具链版本（如 nightly-2021-12-05），然后查看该版本包含的 Cargo 版本。


# 6.21
## 11pm
```bash
## 验证并运行程序
```bash
clang: error: invalid linker name in argument '-fuse-ld=lld'
```


```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly-2021-12-05
source "$HOME/.cargo/env"

# 安装 rustc-dev and llvm-tools-preview:
rustup component add rustc-dev llvm-tools-preview

# 安装LLVM 13
sudo apt-get install llvm-13-dev libclang-common-13-dev -y 


sudo apt-get install llvm-14-dev libclang-common-14-dev -y 



cargo install --path .
cargo build
```



### 蒋老师的建议
Can use this script and update-alternatives to switch clang/llvm versions: https://github.com/ShangjinTang/dotfiles/blob/05ef87daae29475244c276db5d406b58c52be445/linux/ubuntu/22.04/bin/update-alternatives-clang  


cf. https://gist.github.com/junkdog/70231d6953592cd6f27def59fe19e50d

One more note: based on the list of llvm-* tools mentioned there, may also need to apt install lld lld-13


# linking with cc failed: exit status: 1
## LLVM 是编译器基础设施，而 Clang 是一个使用 LLVM 构建的 C/C++/Objective-C 编译器前端。
```bash
sudo apt install build-essential -y 
sudo apt install pkg-config libssl-dev -y
```

## 如何安装LLVM-13对应Clang 
```bash
sudo apt-get update -y
sudo apt-get install llvm-13 clang-13 libclang-common-13-dev -y
```

# 运行报错 shared libraries找不到
```bash
:~/trading-programming/0621-rust-ffi-checker/target/debug# ./cargo-ffi-checker  
./cargo-ffi-checker: error while loading shared libraries: librustc_driver-f92801b4d17b5b5b.so: cannot open shared object file: No such file or directory
```
## 设置 `LD_LIBRARY_PATH` 环境变量
```bash
export LD_LIBRARY_PATH=$HOME/.rustup/toolchains/nightly-2021-12-05-aarch64-unknown-linux-gnu/lib:$HOME/.rustup/toolchains/nightly-2021-12-05-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH
```



# FFIChecker: A Static Analysis Tool For Detecting Memory Management Bugs Between Rust and C/C++

[![build](https://github.com/lizhuohua/rust-ffi-checker/actions/workflows/build.yml/badge.svg)](https://github.com/lizhuohua/rust-ffi-checker/actions/workflows/build.yml)

This tool generates and analyzes LLVM bitcode to detect potential bugs caused by incorrect use of Rust FFI.

Information about bugs detected by this tool are listed in [Trophy Case](trophy-case/README.md).

## Requirements

* Rust nightly, as specified in [rust-toolchain](rust-toolchain).
* `rustc-dev` and `llvm-tools-preview`:

    ```sh
    $ rustup component add rustc-dev llvm-tools-preview
    ```

* `LLVM 13`:

    ```sh
    # Some required libraries are included in 'libclang-common-13-dev'
    $ sudo apt-get install llvm-13-dev libclang-common-13-dev
    ```

## Build

1. Clone the repository

```sh
$ git clone https://github.com/lizhuohua/rust-ffi-checker.git

$ cd rust-ffi-checker
```

2. Build & Install

```sh
# You can build and install the cargo subcommand:
$ cargo install --path .

# Or, you can only build the checker itself:
$ cargo build
```

## Example

The following is a contrived example which contains a use-after-free bug. For more examples, please see [examples](examples) and [trophy-case](trophy-case).

```rust
use libc::{c_void, free};

fn main() {
    let mut n = Box::new(1);
    unsafe {
        free(&mut *n as *const _ as *mut c_void);
    }

    *n = 2;
}
```

It compiles but will crash at runtime. Our checker can detect it at compile time.

## Usage

Before using this tool, make sure your Rust project compiles without any errors or warnings.

```sh
# If you have installed the cargo subcommand:
$ cargo clean; cargo ffi-checker

# Or, you can directly run the checker binary
$ cargo clean; path/to/cargo-ffi-checker ffi-checker
```

You can also set the threshold of warnings to filter out false positives.
```sh
# Only output warnings with at least medium severity
# Available options: "high", "mid", and "low"
$ cargo clean; cargo ffi-checker -- --precision_filter mid
```

## Debug

Set `RUST_LOG` environment variable to enable logging:

```sh
# Enable all logging
$ export RUST_LOG=rust_ffi_checker

# Can also set logging level
$ export RUST_LOG=rust_ffi_checker=debug
```

For more settings, please see the documents of [env_logger](https://crates.io/crates/env_logger).

## Troubleshooting

For macOS, you may encounter `dyld: Library not loaded` error, try setting:

```sh
$ export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
```

## License

See [LICENSE](LICENSE)
