use crate::Error;

/// An item processed by the library.
///
/// Replace this with your domain types.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub raw: String,
    pub normalized: String,
}

/// Parse raw input into an Item.
///
/// Strips whitespace and normalizes to uppercase.
pub fn parse(input: &str) -> Result<Item, Error> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Item {
            raw: input.into(),
            normalized: String::from("(empty)"),
        });
    }
    Ok(Item {
        raw: input.into(),
        normalized: trimmed.to_uppercase(),
    })
}

/// Transform an Item into an output string.
///
/// This is where your core logic lives. Replace with
/// your actual transformation.
pub fn transform(item: &Item) -> String {
    format!("Processed: {}", item.normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normal_input() {
        let item = parse("hello").unwrap();
        assert_eq!(item.raw, "hello");
        assert_eq!(item.normalized, "HELLO");
    }

    #[test]
    fn parse_empty_input() {
        let item = parse("").unwrap();
        assert_eq!(item.normalized, "(empty)");
    }

    #[test]
    fn parse_whitespace_only() {
        let item = parse("  ").unwrap();
        assert_eq!(item.normalized, "(empty)");
    }

    #[test]
    fn transform_output() {
        let item = Item {
            raw: "test".into(),
            normalized: "TEST".into(),
        };
        assert_eq!(transform(&item), "Processed: TEST");
    }

    #[test]
    fn item_serializes_to_json() {
        let item = parse("hello").unwrap();
        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("\"raw\":\"hello\""));
    }
}
