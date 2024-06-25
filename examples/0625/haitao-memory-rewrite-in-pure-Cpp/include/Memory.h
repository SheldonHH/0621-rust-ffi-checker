#pragma once
#include <memory>
#include <thread>
#include <chrono>

namespace org {
namespace memory {

class Memory {
public:
    Memory() = default;
    ~Memory() = default;

    void allocate_memory(size_t size);
    void deallocate_memory() const;
    const uint8_t* get_ptr() const;

private:
    mutable std::unique_ptr<uint8_t[]> ptr;
    mutable std::thread deallocate_thread;
};

std::unique_ptr<Memory> new_memory();

}
}
