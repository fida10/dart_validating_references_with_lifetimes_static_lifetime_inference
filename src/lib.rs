#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_static_str() {
        let static_str = get_static_str();
        assert_eq!(static_str, "This is a static string");
    }
}
