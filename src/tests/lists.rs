#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn single_level_ul() {
    assert_output_json_eq(
        r"
          <ul>
            <li>Item One</li>
            <li>Item Two</li>
            <li>Item Three</li>
          </ul>
        ",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
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
                          "text": "Item One"
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
                          "text": "Item Two"
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
                          "text": "Item Three"
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

#[test]
fn single_level_ol() {
    assert_output_json_eq(
        r"
          <ol>
            <li>Item One</li>
            <li>Item Two</li>
            <li>Item Three</li>
          </ol>
        ",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
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
                          "text": "Item One"
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
                          "text": "Item Two"
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
                          "text": "Item Three"
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

#[test]
fn nested_1() {
    assert_output_json_eq(
        r"
          <ul>
            <li>
              Nested List
              <ol>
                <li>Item One</li>
                <li>Item Two</li>
                <li>Item Three</li>
              </ol>
            </li>
          </ul>
        ",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
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
                          "text": " Nested List "
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
                                  "text": "Item One"
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
                                  "text": "Item Two"
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
                                  "text": "Item Three"
                                }
                              ]
                            }
                          ]
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
