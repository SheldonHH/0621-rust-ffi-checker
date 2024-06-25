#include "include/Memory.h"
#include <iostream>
#include <thread>
#include <chrono>

using namespace org::memory;

int main() {
    auto mem = new_memory();
    std::cout << "Allocating memory\n";
    mem->allocate_memory(32);
    auto ptr = mem->get_ptr();
    std::cout << "Memory allocated at: " << static_cast<void*>(const_cast<uint8_t*>(ptr)) << "\n";

    std::this_thread::sleep_for(std::chrono::seconds(2));
    std::cout << "Memory deallocation simulated\n";

    return 0;
}
