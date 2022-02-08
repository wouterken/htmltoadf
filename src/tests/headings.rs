#[cfg(test)]
#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[test]
fn h1() {
    assert_output_json_eq(
        "<h1>H1</h1>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "heading",
                    "attrs": {
                        "level": 1
                    },
                    "content": [
                        {
                            "type": "text",
                            "text": "H1"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn h2() {
    assert_output_json_eq(
        "<h2>H2</h2>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "heading",
                    "attrs": {
                        "level": 2
                    },
                    "content": [
                        {
                            "type": "text",
                            "text": "H2"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn h3() {
    assert_output_json_eq(
        "<h3>H3</h3>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "heading",
                    "attrs": {
                        "level": 3
                    },
                    "content": [
                        {
                            "type": "text",
                            "text": "H3"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn h4() {
    assert_output_json_eq(
        "<h4>H4</h4>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "heading",
                    "attrs": {
                        "level": 4
                    },
                    "content": [
                        {
                            "type": "text",
                            "text": "H4"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn h5() {
    assert_output_json_eq(
        "<h5>H5</h5>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "heading",
                    "attrs": {
                        "level": 5
                    },
                    "content": [
                        {
                            "type": "text",
                            "text": "H5"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
fn h6() {
    assert_output_json_eq(
        "<h6>H6</h6>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "heading",
                    "attrs": {
                        "level": 6
                    },
                    "content": [
                        {
                            "type": "text",
                            "text": "H6"
                        }
                    ]
                }
            ]
        }),
    );
}

#[test]
/**
 * No such thing as h7. Renders as plain old text
 */
fn h7() {
    assert_output_json_eq(
        "<h7>H7</h7>",
        json!({
            "version": 1,
            "type": "doc",
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "H7"
                        }
                    ]
                }
            ]
        }),
    );
}
