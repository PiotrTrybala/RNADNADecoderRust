
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

        Vec::<Compound>::new()

    }

}


