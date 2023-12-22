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

/**
 *  Empty paragraphs
 */
 #[test]
 fn empty_paragraphs() {
    assert_output_json_eq(
        "
        <h1>
        <span style=\"color: #f1c40f;\">qweq</span>wewq
        </h1>
        <p>&nbsp;</p>
        <p>&nbsp;</p>
        <p>&nbsp;</p>
        <p>qweqwe</p>
        ",
        json!({
            "version": 1,
            "type": "doc",
            "content":
            [
            {
                "type": "heading",
                "attrs":
                {
                    "level": 1
                },
                "content":
                [
                {
                    "type": "text",
                    "text": "qweq",
                    "marks":
                    [
                    {
                        "type": "textColor",
                        "attrs":
                        {
                            "color": "#f1c40f"
                        }
                    }
                    ]
                },
                {
                    "type": "text",
                    "text": "wewq "
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": " "
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": " "
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": " "
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": "qweqwe"
                }
                ]
            }
            ]
        }),
        );
}


/**
 *  Empty hard breaks
 */
 #[test]
 fn hard_breaks() {
    assert_output_json_eq(
        "
        Naked <br/> break
        <p>Paragraph <br/> break</p>
        <p>Double</br> Paragraph <br/> breaks</p>
        <p>Sibling <br/><br/> breaks</p>
        ",
        json!({
            "version": 1,
            "type": "doc",
            "content":
            [
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": " Naked "
                },
                {
                    "type": "hardBreak"
                },
                {
                    "type": "text",
                    "text": " break "
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": "Paragraph "
                },
                {
                    "type": "hardBreak"
                },
                {
                    "type": "text",
                    "text": " break"
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": "Double"
                },
                {
                    "type": "hardBreak"
                },
                {
                    "type": "text",
                    "text": " Paragraph "
                },
                {
                    "type": "hardBreak"
                },
                {
                    "type": "text",
                    "text": " breaks"
                }
                ]
            },
            {
                "type": "paragraph",
                "content":
                [
                {
                    "type": "text",
                    "text": "Sibling "
                },
                {
                    "type": "hardBreak"
                },
                {
                    "type": "hardBreak"
                },
                {
                    "type": "text",
                    "text": " breaks"
                }
                ]
            }
            ]
        }),
    );
}
