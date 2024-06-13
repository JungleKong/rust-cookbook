
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// 读取文件内容作为字符串
fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let mat_path = "examples/cache.rs";
    let content = read_file(mat_path);
    let file = format!("{}", std::env::current_dir().unwrap().display());
    let content = format!("{}{}", file, content);
    let hash = hash_string(&content);
    println!("{}: {}", mat_path, hash);


    // 判断缓存目录是否存在，不存在则创建
    let cache_dir = std::env::current_dir().unwrap().join("../.cache/matc");
    if !std::path::Path::new(&cache_dir).exists() {
        match std::fs::create_dir(&cache_dir) {
            Ok(_) => println!("create dir: {}", cache_dir.display()),
            Err(e) => println!("create dir error: {}", e),
        }
    }

    let hash_path = cache_dir.join(hash.to_string());
    if !std::path::Path::new(&hash_path).exists() {
        // 如果缓存文件不存在，则复制文件到缓存目录
        match std::fs::copy(&mat_path, &hash_path) {
            Ok(_) => println!("copy file: {}", hash_path.display()),
            Err(e) => println!("copy file error: {}", e),
        }
    } else {
        // 如果缓存文件存在，则拷贝缓存文件到指定目录
    }
}




