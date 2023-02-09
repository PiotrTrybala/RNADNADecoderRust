
pub mod engine;

use engine::DecoderEngine;

use std::path::Path;
use std::env;

fn main() {
    


    let decoderEngine = DecoderEngine::new("/schema/schema.json".to_string());


}
#[cfg(test)]
#[test]
fn path_test() {
    let cwd = env::current_dir().unwrap();

    println!("Current working directory: {}", cwd.display());

    let path = Path::new("./schema/schema.json");
    assert_eq!(path.exists(), true);
}
