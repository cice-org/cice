#[macro_export]
macro_rules! Image {
    ($caseName:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resource/image/",
            $caseName
        )
    };
}

#[macro_export]
macro_rules! ImageData {
    ($caseName:expr) => {
        include_bytes!(Image!($caseName))
    };
}
