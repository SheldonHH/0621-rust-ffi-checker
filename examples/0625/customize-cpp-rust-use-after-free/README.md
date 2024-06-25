# 编译
```bash
# 运行build.sh
./build.sh
```


# 1. 方法ASAN
复现错误
```bash
./1-ASAN.sh
```   

为了演示 Rust 生成内存资源，C++ 获取该资源后 Rust 立即释放，导致 C++ 出现 use-after-free 的场景，我们可以构造一个简单的示例。这个示例包括 Rust 代码创建内存资源并将其暴露给 C++，然后 C++ 在 Rust 释放资源后尝试使用该资源。



## 1. 错误位置
```bash
READ of size 2 at 0xffffb8400b50 thread T0
#0 0xffffbcfee11c in __interceptor_strlen
#1 0xffffbce64adc in std::operator<<
#2 0xaaaab4180db8 in main
```

这里显示在 `std::operator<<` 中尝试读取内存，这个内存已经在之前被释放了。


## 2. 内存释放位置
```bash
freed by thread T0 here:
#0 0xffffbd059fe8 in __interceptor_free
#1 0xaaaab4180da8 in main
```

## 3. 内存分配位置
```bash
previously allocated by thread T0 here:
#1 0xffffbcf738a0 in alloc::alloc::alloc
```
内存是由 Rust 的 `CString::new` 分配的。


# 2.方法 2. Clang Static Analyzer
```bash
./2-clang-static-analyzer.sh
```

# 3. 方法3: CPP Checker
```bash
./3-cpp-checker.sh
```

# 4. 方法4 infer facebook
```bash
./4-infer-facebook.sh
```