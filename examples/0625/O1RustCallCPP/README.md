
# 从 Rust 调用 C++ 函数
`BlobstoreClient` 视为 CXX 分类中的不透明类型

不透明类型只能通过引用 `&`、Rust 的 `Box` 或 `UniquePtr`（Rust 对 `std::unique_ptr` 的绑定）进行操作。
我们将添加一个函数，通过该函数 C++ 可以将 `std::unique_ptr<BlobstoreClient>` 返回给 Rust


```rust
// src/main.rs
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

fn main() {
    let client = ffi::new_blobstore_client();
}
```