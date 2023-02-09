
use std::fs;
use std::path::Path;
use serde_json::{Result, Value};

pub struct DecoderEngine {
    compound_base: CompoundBase
}

pub struct Compound {
    symbol: String,
    code: String
}

pub struct CompoundBase {
    schema_file: String,
    compounds: Vec<Compound>
}

impl DecoderEngine {

    fn new(schema_file: String) -> Self {

        let base = CompoundBase::new(schema_file);

        Self {
            compound_base: base
        }
    }

}

impl CompoundBase {

    fn new(schema_file: String) -> Self {

        let compounds = CompoundBase::gen_compound_base(&schema_file);

        Self {
            schema_file,
            compounds
        }
    }

    fn gen_compound_base(schema_file: &String) -> Vec<Compound> {


        let path = Path::new("./schema/schema.json");

        if !path.exists() {
            panic!("File does not exists!");
        }

        let mut content = fs::read_to_string(path).expect("Cannot read from the file");


        let parsed_json: Value = serde_json::from_str(content.as_str()).unwrap();

        assert_eq!(parsed_json["version"], 1);



        Vec::<Compound>::new()

    }

}


