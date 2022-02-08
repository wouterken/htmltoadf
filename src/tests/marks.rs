#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn top_level() {
    assert_output_json_eq(
        "<p style='text-decoration: underline; color: #333;'>Paragraph</p>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "Paragraph",
                            "marks": [
                                {
                                    "type": "textColor",
                                    "attrs": {
                                        "color": "#333333"
                                    }
                                },
                                {
                                    "type": "underline"
                                }
                            ]
                        }
                    ]
                }
            ]
        }),
    );
}
