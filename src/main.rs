mod filesystem;

use markdown;
use std::{
    fs::File,
    io,
    io::prelude::*,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};

use filesystem::{collect_path_with_ext, ensure_dir_exists, touch};

fn collect_site_markdown_files() -> io::Result<Vec<PathBuf>> {
    Ok(collect_path_with_ext(
        &Path::new("./site"),
        &vec![".mdx", ".md"],
    )?)
}

fn md_path_to_public_path(path: &PathBuf) -> PathBuf {
    let copy = path.clone();
    let copy = copy.as_os_str().to_str().unwrap();
    let mut path_vec = copy.split(MAIN_SEPARATOR).collect::<Vec<&str>>();
    path_vec[0] = "public";
    let mut path = PathBuf::from(path_vec.join(&MAIN_SEPARATOR.to_string()));
    path.set_extension("html");

    path
}

fn main() -> io::Result<()> {
    let markdown_paths = collect_site_markdown_files()?;

    for path in markdown_paths {
        let output_path = md_path_to_public_path(&path);

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let markup = markdown::to_html(&contents);

        if let Some(path) = output_path.to_str() {
            ensure_dir_exists(&path)?;
            let mut output_file = touch(&path)?;

            output_file.write_all(markup.as_bytes())?;
        }
    }

    Ok(())
}
