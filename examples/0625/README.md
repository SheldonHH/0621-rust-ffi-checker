# O1 -> O4都是调用`opaque`类型，没有使用`shared` structure

将使用共享结构体 `BlobMetadata` 在我们的 Rust 应用程序和 C++ `blobstore` 客户端之间传递 `blob` 的元数据。



更新 Rust 版本
更新 rustup

首先，确保你的 rustup 是最新的。rustup 是 Rust 的工具链管理器。你可以通过以下命令更新 rustup：

bash
Copy code
rustup self update
更新 Rust

然后，使用 rustup 更新 Rust 到最新的稳定版本：

bash
Copy code
rustup update stable
检查 Rust 版本

确认 Rust 已成功更新到最新版本：

bash
Copy code
rustc --version
你应该看到类似 rustc 1.63.0 或更高版本的输出。

使用最新版本的 Rust 运行项目
现在你可以尝试再次运行你的项目：

bash
Copy code
cargo run
使用特定的 Rust 工具链
如果你需要使用特定版本的 Rust 工具链（例如 nightly 版本），你可以使用 rustup 安装和使用该版本：

bash
Copy code
rustup install nightly
rustup default nightly
完整步骤
总结一下，以下是完整的步骤：

bash
Copy code
rustup self update
rustup update stable
rustc --version
cargo run
如果需要使用 nightly 版本：

bash
Copy code
rustup install nightly
rustup default nightly
rustc --version
cargo run
这样，你的 Rust 版本将更新到最新版本，应该能够解决由于 Rust 版本过旧导致的依赖项构建问题。