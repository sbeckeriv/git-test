extern crate git2;
use std::path::Path;

fn main() {
    let current_path = Path::new(".");
    match git2::Repository::open(&current_path) {
        Ok(repo) => {
            let mut opts = git2::StatusOptions::new();
            let status = repo.statuses(Some(&mut opts));
            let statuses = status.unwrap();
            opts.include_untracked(true).recurse_untracked_dirs(true);
            opts.include_untracked(true);
            println!("files {}", statuses.len());
            for entry in statuses.iter().filter(|e| e.status() == git2::STATUS_WT_NEW) {
                let file = entry.index_to_workdir().unwrap().old_file().path().unwrap();
                println!("#\t{}", file.display());
            }
        }
        _ => (),
    };
    println!("Hello, world!");
}
