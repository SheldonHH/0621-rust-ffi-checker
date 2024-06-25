#include <iostream>
#include <cstring>

extern "C" {
    char* create_string();
    void free_string(char*);
}

int main() {
    // Get string from Rust
    char* rust_string = create_string();
    
    // Rust frees the string
    free_string(rust_string);

    // Use after free: this will trigger an error
    std::cout << rust_string << std::endl;

    return 0;
}
