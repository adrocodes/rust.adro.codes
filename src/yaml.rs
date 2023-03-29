use liquid::{
    model::{
        ScalarCow,
        Value::{self, Array, Nil, Scalar},
    },
    Object,
};
use yaml_rust::{yaml::Hash, Yaml};

pub trait YamlIntoObject {
    fn to_liquid_object(&self) -> Object;
}

fn yaml_to_liquid_value(value: &Yaml) -> Value {
    match value {
        Yaml::Real(r) => {
            if let Ok(r) = r.parse::<f64>() {
                return Scalar(ScalarCow::new(r));
            }

            Nil
        }
        Yaml::Integer(i) => Scalar(ScalarCow::new(*i)),
        Yaml::String(s) => Scalar(ScalarCow::new(s.clone())),
        Yaml::Boolean(b) => Scalar(ScalarCow::new(*b)),
        Yaml::Array(arr) => {
            let mapped = arr.iter().map(yaml_to_liquid_value).collect::<Vec<_>>();
            Array(mapped)
        }
        _ => Nil,
    }
}

fn decode_hash(hash: &Hash) -> Object {
    let mut obj = liquid::object!({});

    for (k, v) in hash {
        let key: Option<String> = match k {
            Yaml::String(s) => Some(s.clone()),
            _ => None,
        };

        if key.is_none() {
            continue;
        }

        obj.insert(key.unwrap().into(), yaml_to_liquid_value(v));
    }

    obj
}

impl YamlIntoObject for Vec<Yaml> {
    fn to_liquid_object(&self) -> Object {
        let inner_hash = self.get(0);

        if let Some(inner_hash) = inner_hash {
            match inner_hash {
                Yaml::Hash(hash) => return decode_hash(&hash),
                _ => println!("First item isn't a hash"),
            }
        }

        liquid::object!({})
    }
}
