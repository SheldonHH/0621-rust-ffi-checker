#include "cxx_memory_relation/include/Memory.h"
#include <cstdio>
#include <memory>
#include <thread>
#include <chrono>

namespace org {
namespace memory {

// 定义Memory类的成员函数，用于分配内存
void Memory::allocate_memory(size_t size) const {
    // 使用智能指针分配指定大小的内存
    ptr = std::make_unique<uint8_t>(size);
    // 创建一个新线程，在1秒后释放内存
    deallocate_thread = std::thread([this]() {
        std::this_thread::sleep_for(std::chrono::seconds(1));
        deallocate_memory();
    });
    // 分离线程，使其在后台运行
    deallocate_thread.detach();
}

// 定义Memory类的成员函数，用于释放内存
void Memory::deallocate_memory() const {
    // 重置智能指针，释放内存
    ptr.reset();
}

// 定义Memory类的成员函数，获取内存指针
const uint8_t* Memory::get_ptr() const {
    // 返回智能指针的原始指针
    return ptr.get();
}

// 定义一个全局函数，创建Memory对象的智能指针
std::unique_ptr<Memory> new_memory() {
    // 返回一个新的Memory对象的智能指针
    return std::make_unique<Memory>();
}

}
}
