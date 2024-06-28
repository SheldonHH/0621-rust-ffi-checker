// src/main.rs

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

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
