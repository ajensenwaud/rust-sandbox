use std::fs; 
use std::io; 
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

fn copy_dir_to(src: &Path, dst: &path) -> io::Result<()> { 
    if !dst.is_dir() { 
        fs::create_dir(dst)?; 
    }
    for entry_result in src.read_dir()? { 
        let entry = entry_result?; 
        let file_type = entry.file_type()?; 
        copy_to(&entry_path(), &file_type, &dst.join(entry.file_name()))?;
    }
    Ok(())
}

#[cfg(unix)]
use std::os::unix::fs::symlink; 
// Stub impl of symlink for platforms that don't provide it
#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q) -> std::io::Result<()> { 
    Err(io::Error::new(io::ErrorKind::Other, format!("Can't copy symbolic link: {}", src.as_ref().display())))
}
