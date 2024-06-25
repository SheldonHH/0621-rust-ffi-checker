// tests/test.rs

use std::process::Command;

#[cfg(test)]
mod test{
    #[test]
    fn test_ffi_checker_preprocess() {
        // 获取当前可执行文件的路径
        let exe_path = std::env::current_exe().unwrap();

        // 构造运行 cargo-ffi-checker 的命令并传入参数
        let output = Command::new(exe_path)
            .arg("cargo-ffi-checker")
            // .arg("ffi-checker-preprocess")
            .arg("ffi-checker")
            .output()
            .expect("failed to execute process");

        // 将 stdout 和 stderr 转换为字符串
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // 打印 stdout 和 stderr 的内容
        println!("stdout:\n{}", stdout);
        println!("stderr:\n{}", stderr);
    }
}