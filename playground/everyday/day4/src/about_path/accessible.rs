use std::io;

pub fn public_func_in_accessible() {
    println!("This is public func in accessible.rs")
}

fn private_func_in_accessible() {
    println!("This is public func in accessible.rs")
}

pub fn call_private_func_in_accessible() {
    println!("Calling private_func_in_accessible");
    println!("呼び出し先はpubがついていないが、他の関数から呼び出しはできる");
    private_func_in_accessible();
}

pub fn path_absolute_form(path: &std::path::Path) -> io::Result<std::path::PathBuf> {
    // Rust相対パスを絶対パスにして返す関数

    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let path = path.strip_prefix(".").unwrap_or(path);
    std::env::current_dir().map(|path_buf| path_buf.join(path))
}

pub fn absolute_path(path: &std::path::Path) -> io::Result<std::path::PathBuf> {
    let path_buf = path_absolute_form(path)?;

    #[cfg(windows)]
    let path_buf = Path::new(
        path_buf
            .as_path()
            .to_string_lossy()
            .trim_start_matches(r"\?"),
    )
    .to_path_buf();

    Ok(path_buf)
}
