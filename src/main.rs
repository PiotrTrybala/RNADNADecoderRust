
pub mod engine;

use engine::DecoderEngine;

fn main() {
    


    let _decoder_engine = DecoderEngine::new("./schema/schema.json".to_string());

    


}

#[cfg(test)]
pub mod tests {
    use crate::DecoderEngine;
    #[test]
    fn test_is_base_not_empty() {
        let engine = DecoderEngine::new("./schema/schema.json".to_string());
        assert!(!engine.is_base_empty());
    }

    #[test]
    #[should_panic]
    fn test_if_empty_schema_specified() {
        let _engine = DecoderEngine::new("./schema/schemajson.json".to_string());
    }

    #[test]
    fn test_base_has_correct_compounds() {
        let engine = DecoderEngine::new("./schema/schema.json".to_string());

        println!("{}", engine.base_size());

        assert!(engine.base_size() == 62);
    }

    #[test]

    fn test_correct_compound_selected() {
        let engine = DecoderEngine::new("./schema/schema.json".to_string());
        assert_eq!(engine.get_symbol("AUG".to_string()), "M(start)");
        assert_eq!(engine.get_symbol("GGG".to_string()), "G");
        assert_eq!(engine.get_symbol("GGU".to_string()), "G");
        assert_eq!(engine.get_symbol("GUA".to_string()), "V");
        assert_eq!(engine.get_symbol("GUG".to_string()), "V");
        assert_eq!(engine.get_symbol("UAA".to_string()), "(stop)");
        assert_eq!(engine.get_symbol("UAG".to_string()), "(stop)");
        assert_eq!(engine.get_symbol("CCC".to_string()), "P");
        assert_eq!(engine.get_symbol("CAU".to_string()), "H");
        assert_eq!(engine.get_symbol("AGG".to_string()), "R");
        assert_eq!(engine.get_symbol("UUU".to_string()), "F");
    }

    #[test]
    fn test_correct_convert_of_sequences() {

        let engine = DecoderEngine::new("./schema/schema.json".to_string());

        let results = engine.decode("AAAUGAACGAAAAUCUGUUCGCUUCAUUCAUUGCCCCCACAAUCCUAGGCCUACCC".to_string());

        assert_eq!(results[0], "KWTKICSLHSLPPQS(stop)AY".to_string());
        assert_eq!(results[2], "M(start)NENLFASFIAPTILGLP".to_string());

    }

}

