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
    cache_clear("ola");
    assert!(!cache_exists("ola"));
    let path = cache_set("ola", "world").unwrap();
    assert!(cache_exists("ola"));
}

pub fn cache_clear_old(key: &str, is_all: bool) {
    let root = get_cache_root().expect("failed to get cache root");
    let target_path = if is_all {
        Path::new(root.as_str()).join("*")
    } else {
        Path::new(root.as_str()).join(key)
    };
    println!("target_path is {:?}", target_path);
    if !target_path.exists() {
        println!("not exists");
        return;
    }

    match fs::remove_dir_all(target_path.to_str().expect("path to_str() error")) {
        Ok(_) => {},
        Err(e) => panic!("remove_dir_all() error: {}", e),
    }
}

pub fn cache_clear(key: &str) {
    let key = create_hash(key);
    let root = get_cache_root().expect("failed to get cache root");
    let target_path = Path::new(root.as_str()).join(key);
    if target_path.exists() {
        fs::remove_file(target_path).expect("failed to remove file");
    }
}

#[test]
fn test_cache_clear() {
    cache_set("a", "b");
    assert_eq!(cache_exists("a"), true);
    cache_clear("a");
    assert_eq!(cache_exists("a"), false);
}

pub fn cache_clear_all() {
    let root = get_cache_root().expect("failed to get cache root");
    let dir = fs::read_dir(root);
    if let Ok(dir) = dir {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            }
        }
    }
}

#[test]
fn test_cache_clear_all() {
    cache_set("a", "1");
    cache_set("b", "2");
    cache_set("c", "3");
    assert_eq!(cache_exists("a"), true);
    assert_eq!(cache_exists("b"), true);
    assert_eq!(cache_exists("c"), true);
    cache_clear_all();
    assert_eq!(cache_exists("a"), false);
    assert_eq!(cache_exists("b"), false);
    assert_eq!(cache_exists("c"), false);
}