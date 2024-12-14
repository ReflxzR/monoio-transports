fn main() {
    let monoio_default = cfg!(feature = "monoio-default");
    let monoio_legacy = cfg!(feature = "monoio-legacy");
    let default_crate = cfg!(feature = "default-crate");

    // Ensure only one of monoio-default or monoio-legacy is enabled
    if monoio_legacy && monoio_default {
        panic!("The feature 'monoio-legacy' cannot work with 'monoio-default'. Consider disabling default features.");
    }

    if default_crate && (monoio_default || monoio_legacy) {
        panic!("The features 'monoio-default' or 'monoio-legacy' cannot be used with 'default-crate'");
    }

    if !monoio_legacy && !monoio_default && !default_crate {
        panic!("At least one of 'monoio-default' or 'monoio-legacy' or 'default-crate' must be enabled.");
    }
}
