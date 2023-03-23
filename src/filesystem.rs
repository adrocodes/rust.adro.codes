use std::{
    cell::RefCell,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

fn has_extension(filename: &str, extensions: &Vec<&str>) -> bool {
    extensions.iter().any(|ext| filename.ends_with(ext))
}

fn walk_directory(folder_path: &Path, file_callback: &dyn Fn(&DirEntry)) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let next_path = entry.path();

        if next_path.is_dir() {
            walk_directory(&next_path, file_callback)?;
        } else {
            file_callback(&entry);
        }
    }

    Ok(())
}

pub fn collect_path_with_ext(start_at: &Path, extensions: &Vec<&str>) -> io::Result<Vec<PathBuf>> {
    let paths: RefCell<Vec<PathBuf>> = RefCell::new(Vec::new());

    walk_directory(start_at, &|entry: &DirEntry| {
        if let Some(filename) = entry.file_name().to_str() {
            if has_extension(&filename, extensions) {
                let mut p = paths.borrow_mut();
                p.push(entry.path());
            }
        }
    })?;

    let paths = paths.borrow();

    Ok(paths.to_vec())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_extension() {
        let has_md = has_extension("filename.md", &vec![".md"]);
        assert_eq!(true, has_md);

        let has_txt = has_extension("filename.mdx", &vec![".txt"]);
        assert_eq!(false, has_txt);
    }

    #[test]
    fn test_collecting_md_files() {
        let paths = collect_path_with_ext(Path::new("./tests/filesystem"), &vec![".md"]).unwrap();
        assert_eq!(2, paths.len());
    }
}
