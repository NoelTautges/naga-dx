use std::fs;

use dxbc::binary::Parser;
use naga_dx::MatchMacrosConsumer;
use test_generator::test_resources;

#[test_resources("shaders/compiled/**/*.dxbc")]
fn get_macros(shader_path: &str) {
    let bytes = fs::read(shader_path);
    assert!(bytes.is_ok(), "Couldn't read shader!");
    let bytes = bytes.unwrap();

    let mut consumer = MatchMacrosConsumer::new();
    let mut parser = Parser::new(bytes.as_ref(), &mut consumer);
    let result = parser.parse();
    assert!(result.is_ok(), "Couldn't match macros!");
}
