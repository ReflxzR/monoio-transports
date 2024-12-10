fn main() {
    let monoio_default = cfg!(feature = "monoio-default");
    let monoio_legacy = cfg!(feature = "monoio-legacy");

    if monoio_legacy && monoio_default {
        panic!("The feature 'monoio-legacy' cannot work with 'monoio-default' flag, consider disabling default features");
    }

    if !monoio_legacy && !monoio_default {
        panic!("Atleast one of 'monoio-default' or 'monoio-legacy' feature is required to be enabled");
    }
}
