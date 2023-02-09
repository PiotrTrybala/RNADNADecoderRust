
use std::fs;
use std::path::Path;
use serde_json::{Result, Value};
use std::collections::HashMap;

pub struct DecoderEngine {
    compound_base: CompoundBase
}

pub struct CompoundBase {
    compounds: HashMap<String, String>
}

impl DecoderEngine {

    pub fn new(schema_file: String) -> Self {

        let base = CompoundBase::new(schema_file);

        Self {
            compound_base: base
        }
    }

    pub fn get_symbol(&self, name: String) -> String {
        self.compound_base.get_symbol_by_schema(name)
    }

    pub fn is_base_empty(&self) -> bool {
        self.compound_base.compounds.is_empty()
    }

    pub fn base_size(&self) -> usize {
        self.compound_base.compounds.len()
    }

}

impl CompoundBase {

    pub fn new(schema_file: String) -> Self {

        let compounds = CompoundBase::gen_compound_base(&schema_file);

        Self {
            compounds
        }
    }

    pub fn get_symbol_by_schema(&self, schema: String) -> String {
        self.compounds.get(&schema).unwrap().to_string()
    }
    /// generating compounds db from schema file specified using path as argument
    fn gen_compound_base(schema_file: &String) -> HashMap<String, String> 
    {

        let mut compounds = HashMap::<String, String>::new();
        // checking if schema file exist on given path, if not, PANIC!!!
        let path = Path::new(&schema_file);
        if !path.exists() {
            panic!("File does not exists!");
        }

        // reading and parsing schema file
        let content = fs::read_to_string(path).expect("Cannot read from the file");
        let parsed_json: Value = serde_json::from_str(content.as_str().clone()).unwrap();

        println!("Content: {}", content);

        // extracting special symbols
        let special_symbols = parsed_json["special"].as_object().unwrap().clone();

        // extracting and inserting starting symbol to resulting db
        let start_symbol = special_symbols["start"].as_object().unwrap().clone();

        compounds.insert(start_symbol["schema"].as_str().unwrap().to_string(), start_symbol["symbol"].as_str().unwrap().to_string());

        // extracting and inserting stopping symbols to resulting db
        let stop_symbols = special_symbols["stop"].clone();
        for stop_sym in stop_symbols.as_array().unwrap() {
            compounds.insert(stop_sym["schema"].as_str().unwrap().to_string(), stop_sym["symbol"].as_str().unwrap().to_string());
        }
        // extracting and inserting other compounds to resulting db
        let other_compounds = parsed_json["compounds"].as_array().unwrap().clone();

        for comp in other_compounds {
            /*
                This part is resposible for parsing an range of characters

                If schema is specified like this:

                    XX[YZ]

                then generated resulting schemas are:

                - XXXY
                - XXXZ
            */
            let schema = comp["schema"].as_str().unwrap().to_string();
            let symbol = comp["symbol"].as_str().unwrap().to_string();
            let range_start = schema.find("[").unwrap() + 1;
            let range_stop = schema.len() - 1;
            let base_schema = &schema[..range_start-1];
            for i in range_start..range_stop {
                let mut complete_base = String::from("");
                complete_base += base_schema.clone();
                complete_base.push(schema.chars().nth(i).unwrap());
                compounds.insert(complete_base, symbol.clone());
            }
        }
        compounds
    }

}


