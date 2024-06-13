
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        use std::env;
        let binary_path = env::var("HOME").unwrap() + "/.cargo/bin/rustfmt";
        println!("{binary_path}");
    }
}
