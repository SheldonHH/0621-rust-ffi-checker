// src/blobstore.cc

#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <functional>
#include <string>
#include <iostream>

BlobstoreClient::BlobstoreClient() {}

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
    return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}

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

int main() {
    auto client = new_blobstore_client();

    // 上传一个 blob。
    std::vector<std::vector<uint8_t>> chunks = { {'f', 'e', 'a', 'r', 'l', 'e', 's', 's'}, {'c', 'o', 'n', 'c', 'u', 'r', 'r', 'e', 'n', 'c', 'y'} };
    MultiBuf buf = { chunks, 0 };
    uint64_t blobid = client->put(buf);
    std::cout << "blobid = " << blobid << std::endl;

    return 0;
}
