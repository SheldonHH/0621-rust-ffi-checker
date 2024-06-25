# CXX_Memory_Relation
这段代码展示了如何使用 Rust 和 C++ 通过 FFI（Foreign Function Interface）进行内存管理的互操作性。具体操作如下：

1. **C++ 类型和函数定义**：通过 `cxx` crate 提供的 `bridge` 宏定义了一个 C++ 的命名空间 `org::memory` 和相关的功能。其中包括：
   - `Memory` 类型的定义。
   - `new_memory` 函数用于创建 `Memory` 类型的对象。
   - `allocate_memory` 函数用于分配指定大小的内存。
   - `deallocate_memory` 函数用于释放内存。
   - `get_ptr` 函数返回当前内存块的指针。

2. **Rust 主函数**：
   - 使用 `ffi::new_memory` 创建一个 `Memory` 对象。
   - 在内存分配前后打印指针地址和值，检查内存状态。
   - 通过 `thread::sleep` 模拟等待，等待 C++ 端可能的内存处理。
   - 使用不安全的 Rust 代码块，对内存进行读写操作，展示 Rust 和 C++ 间的内存交互。

这段代码主要用于展示 Rust 与 C++ 之间通过 FFI 进行安全和有效的内存操作，强调了在使用非安全 Rust 代码时必须非常小心，确保所有内存操作都是安全的。
