mod filesystem;

use std::{
    io,
    path::{Path, PathBuf},
};

use filesystem::collect_path_with_ext;

fn collect_site_markdown_files() -> io::Result<Vec<PathBuf>> {
    Ok(collect_path_with_ext(
        &Path::new("./site"),
        &vec![".mdx", ".md"],
    )?)
}

fn main() -> io::Result<()> {
    let _markdown_paths = collect_site_markdown_files()?;

    Ok(())
}
