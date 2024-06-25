use std::{thread, time::Duration};

// 使用 C++ FFI (外部函数接口)
#[cxx::bridge(namespace = "org::memory")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx_memory_relation/include/Memory.h");
        
        type Memory;  // 定义 C++ 的 Memory 类型

        fn new_memory() -> UniquePtr<Memory>;  // 创建 Memory 对象

        fn allocate_memory(&self, size: usize);  // 分配内存
        unsafe fn deallocate_memory(&self);  // 释放内存
        fn get_ptr(&self) -> *const u8;  // 获取内存指针
    }
}

fn main() {
    let size = 32;  // 定义分配内存的大小

    unsafe {
        let mem = ffi::new_memory();  // 创建 Memory 对象

        println!("========================   C++   ========================");
        println!("- Pointer before memory allocation: {:?}", mem.get_ptr());
        mem.allocate_memory(size);  // 在 C++ 中分配内存
        let ptr = mem.get_ptr();  // 获取分配后的内存指针
        println!("- Allocated memory at: {:?}, value is {:?}", ptr, std::ptr::read(ptr));

        // 在 C++ 侧释放内存前等待 2 秒
        println!("Deallocating memory");
        thread::sleep(Duration::from_secs(2));
        println!("- Checking value ({:?}) of pointer: {:?}", std::ptr::read(ptr), ptr);
        println!("=========================================================");

        println!("\n========================   Rust   ========================");
        let ptr = ptr as *mut u32;  // 将指针类型转换为 mutable u32 指针
        println!("- Data at pointer ({:?}): {:?}", ptr, std::ptr::read(ptr));

        let n = 12;
        println!("Writing '{:?}' to memory", n);  // 向内存写入数据
        std::ptr::write(ptr, n);  // 在 Rust 中操作内存
        println!("- Checking value at pointer ({:?}): {:?}", ptr, std::ptr::read(ptr));
        println!("=========================================================");
    }
}
