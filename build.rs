fn main() {
    let pool = cfg!(feature = "pool");
    let hyper_tls = cfg!(feature = "hyper-tls");
    let default_crate = cfg!(feature = "default-crate");

    if hyper_tls && pool {
        panic!("The feature 'hyper-tls' cannot be enabled with feature 'pool'");
    }

    if default_crate && (pool || hyper_tls) {
        panic!("The features 'pool' or 'hyper-tls' cannot be used with 'default-crate'. Disable any one of them or disable default features");
    }

    if !pool && !hyper_tls && !default_crate {
        panic!("No features enabled! At least one of 'pool' or 'hyper-tls' or 'default-crate' must be enabled");
    }
}
