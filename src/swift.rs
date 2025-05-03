use zed_extension_api::{self as zed};

#[derive(Default)]
struct SwiftExtension;

impl zed::Extension for SwiftExtension {
    fn new() -> Self {
        Self::default()
    }
}

zed::register_extension!(SwiftExtension);
