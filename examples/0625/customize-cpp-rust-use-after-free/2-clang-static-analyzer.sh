# 使用 scan-build 进行静态分析
scan-build g++ -o main main.cpp -Ltarget/release -lrust_lib -lpthread
