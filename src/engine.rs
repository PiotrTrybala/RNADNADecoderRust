
use std::fs;
use std::path::Path;
use serde_json::{Result, Value};
use std::collections::HashMap;

pub struct DecoderEngine {
    compound_base: CompoundBase
}

pub struct CompoundBase {
    schema_file: String,
    compounds: HashMap<String, String>
}

impl DecoderEngine {

    pub fn new(schema_file: String) -> Self {

        let base = CompoundBase::new(schema_file);

        Self {
            compound_base: base
        }
    }

}

impl CompoundBase {

    pub fn new(schema_file: String) -> Self {

        let compounds = CompoundBase::gen_compound_base(&schema_file);

        Self {
            schema_file,
            compounds
        }
    }

    fn gen_compound_base<V, K>(schema_file: &String) -> HashMap<V, K> {

        let compounds = 

        let path = Path::new(&schema_file);

        if !path.exists() {
            panic!("File does not exists!");
        }

        let mut content = fs::read_to_string(path).expect("Cannot read from the file");


        let parsed_json: Value = serde_json::from_str(content.as_str()).unwrap();

        
        let special_symbols = parsed_json["special"].clone();


        let start_symbol = special_symbols["start"].clone();






        HashMap::<V, K>::new()

    }

}


