mod error;
pub mod example;

pub use error::Error;

/// Process input through the library's core logic.
///
/// This is the main entry point for library consumers.
/// Replace the example implementation with your own.
pub fn process(input: &str) -> Result<String, Error> {
    let item = example::parse(input)?;
    Ok(example::transform(&item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_valid_input() {
        let result = process("hello").unwrap();
        assert!(result.contains("HELLO"));
    }

    #[test]
    fn process_empty_input() {
        let result = process("").unwrap();
        assert!(result.contains("(empty)"));
    }
}
