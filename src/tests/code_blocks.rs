#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn pre_code_renders_code_block() {
    assert_output_json_eq(
        "<pre><code>CODE</code></pre>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "codeBlock",
                    "content": [
                        {
                            "type": "text",
                            "text": "CODE"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn inline_code_still_renders_code_mark() {
    assert_output_json_eq(
        "<p>Use <code>inline</code> code</p>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "Use "
                        },
                        {
                            "type": "text",
                            "text": "inline",
                            "marks": [
                                {
                                    "type": "code"
                                }
                            ]
                        },
                        {
                            "type": "text",
                            "text": " code"
                        }
                    ]
                }
            ]
        }),
    );
}
