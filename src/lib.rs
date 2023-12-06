/// Write a function get_static_str that returns a &'static str. 
/// Demonstrate how Rust infers the 'static lifetime for string literals.

fn get_static_str() -> &'static str {
    return "This is a static string";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_static_str() {
        let static_str = get_static_str();
        assert_eq!(static_str, "This is a static string");
    }
}