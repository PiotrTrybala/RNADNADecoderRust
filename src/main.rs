
pub mod engine;

use engine::DecoderEngine;

use std::path::Path;
use std::env;

fn main() {
    


    let decoderEngine = DecoderEngine::new("./schema/schema.json".to_string());

    


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
        let engine = DecoderEngine::new("./schema/schemajson.json".to_string());
    }

    #[test]
    fn test_base_has_correct_compounds() {
        let engine = DecoderEngine::new("./schema/schema.json".to_string());

        assert!(engine.base_size() == 26);
    }

}

