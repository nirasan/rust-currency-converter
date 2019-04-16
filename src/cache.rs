use std::env;
use std::path::Path;
use std::fs::create_dir;
use std::io;

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
        create_dir(dir)
    }
}

#[test]
fn test_create_cache_root_dir() {
    let result = create_cache_root_dir();
    assert_eq!(result.is_ok(), true);
    let result = create_cache_root_dir();
    assert_eq!(result.is_ok(), true);
}