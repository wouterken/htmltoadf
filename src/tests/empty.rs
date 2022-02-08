#[cfg(test)]
#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[test]
fn empty() {
    assert_output_json_eq(
        "",
        json!({
            "version": 1,
            "type": "doc",
            "content": []
        }),
    );
}

#[test]
fn html_only() {
    assert_output_json_eq(
        "<html></html>",
        json!({
            "version": 1,
            "type": "doc",
            "content": []
        }),
    );
}
