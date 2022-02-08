#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn top_level() {
    assert_output_json_eq(
        "<p>Paragraph</p>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "Paragraph"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
/**
 * Nested Paragraphs are flattened
 */
fn nested() {
    assert_output_json_eq(
        "<p>Paragraph<p>Nested</p></p>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "Paragraph"
                        }
                    ]
                },
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "Nested"
                        }
                    ]
                }
            ]
        }),
    );
}
