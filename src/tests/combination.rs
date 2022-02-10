#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn top_level() {
    assert_output_json_eq(
        r"
            <html>
                <body>
                    <p>A Paragraph</p>
                    <ul>
                        <li>An unordered list</li>
                        <li>
                            <p>Some Content</p>
                            <ol>
                                <li>With an ordered list inside it</li>
                            </ol>
                        </li>
                        <li>
                            <div style='color: #0F0'>
                                <span>With some blue text inside</span>
                            </div>
                        </li>
                        <li>
                            <p>And an image!</p>
                            <img src='http://example.com/example.jpg/>
                        </li>
                    </ul>
                </body>
            </html>
        ",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
            {
              "type": "paragraph",
              "content": [
                {
                  "type": "text",
                  "text": "A Paragraph"
                }
              ]
            },
            {
              "type": "bulletList",
              "content": [
                {
                  "type": "listItem",
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "type": "text",
                          "text": "An unordered list"
                        }
                      ]
                    }
                  ]
                },
                {
                  "type": "listItem",
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "type": "text",
                          "text": "Some Content"
                        }
                      ]
                    },
                    {
                      "type": "orderedList",
                      "content": [
                        {
                          "type": "listItem",
                          "content": [
                            {
                              "type": "paragraph",
                              "content": [
                                {
                                  "type": "text",
                                  "text": "With an ordered list inside it"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "type": "listItem",
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "type": "text",
                          "text": "With some blue text inside",
                          "marks": [
                            {
                              "type": "textColor",
                              "attrs": {
                                "color": "#00FF00"
                              }
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "type": "listItem",
                  "content": [
                    {
                      "type": "paragraph",
                      "content": [
                        {
                          "type": "text",
                          "text": "And an image!"
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        }),
    );
}
