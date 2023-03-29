use liquid::{
    model::{
        ScalarCow,
        Value::{Array, Nil, Scalar},
    },
    Object,
};
use yaml_rust::{yaml::Hash, Yaml};

pub trait YamlIntoObject {
    fn to_liquid_object(&self) -> Object;
}

fn decode_and_set_for_key(value: &Yaml, key: String, obj: &mut Object) {
    match value {
        Yaml::Real(r) => {
            if let Ok(r) = r.parse::<f64>() {
                obj.insert(key.into(), Scalar(ScalarCow::new(r)));
            }
        }
        Yaml::Integer(i) => {
            obj.insert(key.into(), Scalar(ScalarCow::new(*i)));
        }
        Yaml::String(s) => {
            obj.insert(key.into(), Scalar(ScalarCow::new(s.clone())));
        }
        Yaml::Boolean(b) => {
            obj.insert(key.into(), Scalar(ScalarCow::new(*b)));
        }
        // TODO:
        // Yaml::Array(arr) => {
        //     // obj.insert(key.into(), liquid::model::Value::Array(arr.clone()));
        // }
        _ => {
            obj.insert(key.into(), Nil);
        }
    };
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

        let key = key.unwrap();

        decode_and_set_for_key(v, key, &mut obj);
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
