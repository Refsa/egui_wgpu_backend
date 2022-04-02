use naga::front::wgsl::parse_str;

#[test]
pub fn validate_shader() {
    let shader_path = "src/shader/egui.wgsl";
    let content = std::fs::read_to_string(shader_path).unwrap();

    let parsed = parse_str(&content);
    parsed.unwrap();
}