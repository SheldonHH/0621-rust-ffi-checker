
#[cxx::bridge]
mod ffi {
    // 从 Rust 暴露的类型和函数
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    // 从 C++ 暴露的类型和函数
    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;
        // Rust中的UniquePtr确保对象在不再使用时自动自动自动释放内存
        // 都提供独占所有权和自动内存管理的功能，区别在于 UniquePtr 是为 Rust 和 C++ 互操作设计的，专门用于跨语言边界管理对象。
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
    }
}

// 一个迭代不连续文件对象的连续块的迭代器。
// 这里使用 Vec<Vec<u8>> 作为示例，但实际上这可能迭代某些更复杂的 Rust 数据结构，
// 比如 rope，或者可能从某处惰性加载块。
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

// 获取下一个块
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    let client = ffi::new_blobstore_client();

    // 上传一个 blob。
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);
}