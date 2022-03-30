use naga_dx::parse;
use std::fs;
use test_generator::test_resources;

#[test_resources("shaders/compiled/**/*.dxbc")]
fn parse_shader(shader_path: &str) {
    let bytes = fs::read(shader_path);
    assert!(bytes.is_ok(), "Couldn't read shader!");
    let bytes = bytes.unwrap();
    let dxbc = parse(bytes);
    assert!(dxbc.is_ok(), "Couldn't parse shader!");
}
