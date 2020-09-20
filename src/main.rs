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

fn create_override(directory: &std::path::Path) -> ignore::overrides::Override {
    let mut override_builder = ignore::overrides::OverrideBuilder::new(directory);
    let glob_override = override_builder.build().unwrap();
    glob_override
}

fn print_dir() {
    let test_dir = "test_dir";
    let test_path = std::path::PathBuf::from(&test_dir);
    let orrde = create_override(&test_path);
    let files: Vec<_> = ignore::WalkBuilder::new(test_dir)
        .git_ignore(false)
        .hidden(false)
        .build()
        .map(|entry| entry.unwrap().path().to_owned())
        .collect();

    for i in files {
        println!("File {:?}", &i)
    }

}

fn main() {
    print_dir();
}