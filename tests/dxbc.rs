use naga::back::hlsl;
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
    let (module, info) = dxbc.unwrap();

    let hlsl_options = hlsl::Options {
        shader_model: hlsl::ShaderModel::V5_0,
        binding_map: hlsl::BindingMap::default(),
        fake_missing_bindings: false,
        special_constants_binding: None,
    };
    let mut hlsl_code = String::new();
    let mut hlsl_writer = hlsl::Writer::new(&mut hlsl_code, &hlsl_options);
    hlsl_writer.write(&module, &info).unwrap();
    println!("{}", &hlsl_code);
}
