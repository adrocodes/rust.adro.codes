mod filesystem;
mod yaml;

use liquid;
use markdown::{
    self,
    mdast::{Node, Yaml},
    Constructs, Options, ParseOptions,
};
use std::{
    fs::File,
    io,
    io::prelude::*,
    path::{Path, PathBuf, MAIN_SEPARATOR},
};
use yaml_rust::YamlLoader;

use filesystem::{collect_path_with_ext, ensure_dir_exists, touch};
use yaml::YamlIntoObject;

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

fn extract_yaml_frontmatter(node: &Node) -> Option<Yaml> {
    if let Some(children) = node.children() {
        let mut possible_yaml_vec = children
            .into_iter()
            .filter_map(|n| match n {
                Node::Yaml(x) => Some(x.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        return possible_yaml_vec.pop();
    }

    None
}

fn main() -> io::Result<()> {
    let markdown_paths = collect_site_markdown_files()?;
    let template_builder = liquid::ParserBuilder::with_stdlib().build().unwrap();
    let base_template_content = get_base_liquid_template()?;
    let default_options = Options::default();

    let to_html_options = Options {
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..default_options.parse.constructs
            },
            ..default_options.parse
        },
        ..default_options
    };

    for path in markdown_paths {
        let output_path = md_path_to_public_path(&path);
        let output_path = output_path.to_str();

        if output_path.is_none() {
            continue;
        }

        let output_path = output_path.unwrap();

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let markup: String = match markdown::to_html_with_options(&contents, &to_html_options) {
            Ok(content) => content,
            Err(_) => String::from(""),
        };
        let mdast = markdown::to_mdast(&contents, &to_html_options.parse).unwrap();

        // Extract `Yaml` node and parse that into the `globals` below.
        let yaml = extract_yaml_frontmatter(&mdast).unwrap_or(Yaml {
            value: String::new(),
            position: None,
        });

        let docs = YamlLoader::load_from_str(&yaml.value).unwrap();
        let mut _globals = docs.to_liquid_object();

        // TODO: Need changes in https://github.com/cobalt-org/liquid-rust/pull/492 to allow for missing variables
        // parse markdown with liquid with `globals` including the YAML frontmatter
        // let markup_liquid_parsed = template_builder
        //     .parse(&markup)
        //     .unwrap()
        //     .render(&globals)
        //     .unwrap();

        ensure_dir_exists(&output_path)?;
        let mut output_file = touch(&output_path)?;

        // Parse the parsed markdown into the template
        let template_parser = template_builder.parse(&base_template_content).unwrap();

        // globals.insert("body".into(), Scalar(ScalarCow::new(markup_liquid_parsed)));
        let globals = liquid::object!({ "body": markup });

        let output = template_parser.render(&globals).unwrap();

        output_file.write_all(output.as_bytes())?;
    }

    Ok(())
}
