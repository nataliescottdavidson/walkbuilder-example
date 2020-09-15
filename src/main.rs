use std::{fs::self, fs::DirEntry, path::Path, io};

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn printentry(entry: &DirEntry) {
    println!("{:?}", entry)
}

fn main() -> io::Result<()> {
    let test_path = Path::new("test_dir");
    visit_dirs(test_path, &printentry)?;
    Ok(())
}