fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
    config
        .compile_protos(&["proto/task/base_task_data.proto"], &["proto"])
        .unwrap();
}
