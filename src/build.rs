fn main() {
    // Initialize the hooky library if the current package is the primary package
    if option_env!("CARGO_PRIMARY_PACKAGE").is_some() {
        hooky::init(true)
    }
}
