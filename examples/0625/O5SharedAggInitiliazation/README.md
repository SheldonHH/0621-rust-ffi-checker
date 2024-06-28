# Shared structs translate to a C++ aggregate-initialization compatible struct exactly matching the layout of the Rust one.

C++ 聚合初始化兼容的结构体，指的是可以使用聚合初始化方式初始化的 C++ 结构体。聚合初始化是一种简单的初始化方式，允许直接使用大括号 {} 给结构体的成员赋值。

在 C++ 中，聚合类型通常包括以下几种：

没有用户定义的构造函数的类（struct 或 class）。
没有私有或受保护的非静态数据成员。
没有基类（继承）。
没有虚函数。





在 Rust 和 C++ 之间共享结构体时，需要确保这些结构体在两种语言中的布局是完全一致的，以便可以**按值跨语言边界传递**。这样，Rust 结构体会被翻译为符合 C++ 聚合初始化规则的结构体，确保它们的内存布局匹配，并且可以在 C++ 中使用聚合初始化来创建实例。
例如，Rust 中定义的结构体：
```rust
#[cxx::bridge]
mod ffi {
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }
}
```
会被翻译为一个符合 C++ 聚合初始化规则的结构体：
```cpp
struct BlobMetadata {
    size_t size;
    std::vector<std::string> tags;
};

// 在 C++ 中可以这样初始化
BlobMetadata metadata = {1024, {"tag1", "tag2"}};
```

# Aggregated-Initialized Compatible Struct 聚合初始化兼容的结构体的例子
### 没有私有或受保护的非静态数据成员所有数据成员都是公有的。没有基类（继承） 没有虚函数
```cpp
struct Point {
    int x;
    int y;
};

Point p = {10, 20};  // 聚合初始化
```


非兼容聚合初始化的示例
```cpp
struct NonAggregate {
private:
    int hidden;  // 私有成员
public:
    std::string name;
    int age;
    
    NonAggregate() : hidden(0), name("Unknown"), age(0) {}  // 用户定义的构造函数
};
```