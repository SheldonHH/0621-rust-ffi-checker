#include <iostream>
#include <cstring>

extern "C" {
    char* combine();
    char* create_string();
    void free_string(char*);
}

int main() {
    //成功✅detect
    // Get string from Rust
    char* rust_string = create_string();

    // Rust frees the string
    free_string(rust_string);



    // 失败❌detect，复现：uncomment下面的行，comment上面的两行
    // char* rust_string = combine();
    // Use after free: this will trigger an error
    std::cout << rust_string << std::endl;

    return 0;
}
