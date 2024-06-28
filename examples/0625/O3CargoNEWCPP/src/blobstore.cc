// src/blobstore.cc

#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <functional>
#include <string>

// 上传一个新的 blob 并返回一个 blobid，作为 blob 的句柄。
uint64_t BlobstoreClient::put(MultiBuf &buf) const {
  // 遍历调用者的块迭代器。
  std::string contents;
  while (true) {
    auto chunk = next_chunk(buf);
    if (chunk.size() == 0) {
      break;
    }
    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
  }

  // 假装我们做了一些有用的事情来持久化数据。
  auto blobid = std::hash<std::string>{}(contents);
  return blobid;
}
