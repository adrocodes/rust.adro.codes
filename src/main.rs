mod filesystem;

use liquid;
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

fn get_base_liquid_template() -> io::Result<String> {
    let mut file = File::open("./templates/base.liquid")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
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
    let template_builder = liquid::ParserBuilder::with_stdlib().build().unwrap();
    let base_template_content = get_base_liquid_template()?;

    for path in markdown_paths {
        let output_path = md_path_to_public_path(&path);

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let markup = markdown::to_html(&contents);

        if let Some(path) = output_path.to_str() {
            ensure_dir_exists(&path)?;
            let mut output_file = touch(&path)?;

            let template_parser = template_builder.parse(&base_template_content).unwrap();

            let globals = liquid::object!({
                "body": markup,
            });

            let output = template_parser.render(&globals).unwrap();

            output_file.write_all(output.as_bytes())?;
        }
    }

    Ok(())
}
