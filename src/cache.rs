use std::env;
use std::path::Path;
use std::io::{self, Write, BufWriter};
use std::fs;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn get_cache_root() -> Option<String> {
    let home = env::home_dir()?;
    let root = format!("{}/.rcc", home.to_str()?);
    Some(root)
}

pub fn create_cache_root_dir() -> io::Result<()> {
    let root = get_cache_root().ok_or(io::Error::new(io::ErrorKind::Other, "failed to get cache root"))?;
    let dir = Path::new(&root);
    if dir.exists() {
        Ok(())
    } else {
        fs::create_dir(dir)
    }
}

#[test]
fn test_create_cache_root_dir() {
    let result = create_cache_root_dir();
    assert_eq!(result.is_ok(), true);
    let result = create_cache_root_dir();
    assert_eq!(result.is_ok(), true);
}

fn create_hash(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(s.as_bytes());
    hasher.result_str()
}

pub fn cache_set(key: &str, val: &str) -> Option<String> {
    let key = create_hash(key);

    let root = get_cache_root()?;
    let path = Path::new(root.as_str())
        .join(key);
    let mut f = BufWriter::new(
        fs::File::create(path.to_str()?).ok()?);
    let _ = f.write_all(val.as_bytes());
    path.to_str().and_then(|s| Some(s.to_string()))
}

#[test]
fn test_cache_set() {
    let path = cache_set("hello", "world").unwrap();
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "world");
}

pub fn cache_get(key: &str) -> Option<String> {
    let key = create_hash(key);

    let root = get_cache_root()?;
    let path = Path::new(root.as_str())
        .join(key);
    fs::read_to_string(path).ok()
}

#[test]
fn test_cache_get() {
    let path = cache_set("hi", "world").unwrap();
    let content = cache_get("hi").unwrap();
    assert_eq!(content, "world");
}

pub fn cache_exists(key: &str) -> bool {
    let key = create_hash(key);

    let root = get_cache_root();
    if root.is_none() {
        return false;
    }

    let path = Path::new(root.unwrap().as_str())
        .join(key);

    return path.exists();
}

#[test]
fn test_cache_exists() {
    cache_clear("", true);
    assert!(!cache_exists("ola"));
    let path = cache_set("ola", "world").unwrap();
    assert!(cache_exists("ola"));
}

pub fn cache_clear(key: &str, is_all: bool) {
    let root = get_cache_root().expect("failed to get cache root");
    let target_path = if is_all {
        Path::new(root.as_str()).to_path_buf()
    } else {
        Path::new(root.as_str()).join(key)
    };
    match fs::remove_dir_all(target_path.to_str().expect("path to_str() error")) {
        Ok(_) => {},
        Err(e) => panic!("remove_dir_all() error: {}", e),
    }
}
