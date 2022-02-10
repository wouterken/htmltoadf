#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn color_1() {
    assert_output_json_eq(
        "<p style='color: #332255'>Paragraph</p>",
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
                                        "color": "#332255"
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn color_2() {
    assert_output_json_eq(
        "<p style='color: #389'>Paragraph</p>",
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
                                        "color": "#338899"
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn color_3() {
    assert_output_json_eq(
        "<p style='color: rgb(100, 200, 214);'>Paragraph</p>",
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
                                        "color": "#64c8d6"
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        }),
    );
}
