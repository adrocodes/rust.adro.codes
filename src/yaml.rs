use liquid::Object;
use yaml_rust::Yaml;

pub trait YamlIntoObject {
    fn to_liquid_object(&self) -> Object;
}

impl YamlIntoObject for Vec<Yaml> {
    fn to_liquid_object(&self) -> Object {
        // TODO:
        liquid::object!({})
    }
}
