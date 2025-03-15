#[macro_export]
macro_rules! TestImage {
    ($caseName:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resource/image/",
            $caseName
        )
    };
}

#[macro_export]
macro_rules! TestImageData {
    ($caseName:expr) => {
        include_bytes!(TestImage!($caseName))
    };
}
