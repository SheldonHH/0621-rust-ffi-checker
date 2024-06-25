# 克隆 Infer 仓库并编译


# 更新包列表并安装基础依赖
sudo apt-get update -y
sudo apt-get install -y opam pkg-config gcc autoconf automake cmake git

# 安装其他依赖项
sudo apt-get install -y m4 unzip bubblewrap

# 初始化 opam
opam init
opam update
eval $(opam env)



# 使用 opam 安装 Infer 所需的 OCaml 依赖项
opam install . --deps-only

# 编译 Infer (用于 C 和 Objective-C 分析器，使用 `./build-infer.sh clang`)
./build-infer.sh clang

# 将 Infer 安装到系统范围内
sudo make install

# 或者将 Infer 添加到 PATH 环境变量中
export PATH=`pwd`/infer/bin:$PATH



# ，即使你主要用于分析 C++ 代码，安装 Infer 的过程仍然需要 opam，因为 Infer 的一些核心组件和依赖项是用 OCaml 编写的，而 opam 是 OCaml 的包管理工具。通过 opam，你可以轻松管理和安装 Infer 所需的 OCaml 库和工具。


# 安装 Java (仅在需要 Java 分析时)
# sudo apt-get install -y default-jdk

# 安装其他依赖项
sudo apt-get install -y m4 unzip bubblewrap



git clone https://github.com/facebook/infer.git
cd infer
make install

# 使用 Infer 进行静态分析
infer run -- g++ -o main main.cpp -Ltarget/release -lrust_lib -lpthread
