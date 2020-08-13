fn create_override(directory: &std::path::Path) -> ignore::overrides::Override {
    let mut override_builder = ignore::overrides::OverrideBuilder::new(directory);
    override_builder.add(".hidden_dir").unwrap();
    let glob_override = override_builder.build().unwrap();
    glob_override
}

fn print_dir() {
    let test_dir = "test_dir";
    let test_path = std::path::PathBuf::from(&test_dir);
    let orrde = create_override(&test_path);
    let files: Vec<_> = ignore::WalkBuilder::new(test_dir)
        .git_ignore(false)
        .overrides(orrde)
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