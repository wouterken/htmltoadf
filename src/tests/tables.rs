#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
#[test]
fn test_empty_cell() {
    assert_output_json_eq(
        r"<div><table ><tbody>
            <tr><td >A</td><td >B</td><td >C</td></tr>
            <tr><td >value 1</td><td ></td><td >value 2</td></tr>
            </tbody></table>
        </div>",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
            {
              "type": "table",
              "content": [
                {
                  "type": "tableRow",
                  "content": [
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "A"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "B"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "C"
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "type": "tableRow",
                  "content": [
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "value 1"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell"
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "value 2"
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
              "type": "paragraph",
              "content": [
                {
                  "type": "text",
                  "text": " "
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
 fn test_hard_break_in_cell() {
    assert_output_json_eq(
        r"<div><table ><tbody>
            <tr><td >A</td><td >B</td><td >C</td></tr>
            <tr><td >value 1</td><td ><br/></td><td >value 2</td></tr>
            </tbody></table>
        </div>",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
            {
              "type": "table",
              "content": [
                {
                  "type": "tableRow",
                  "content": [
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "A"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "B"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "C"
                            }
                          ]
                        }
                      ]
                    }
                  ]
                },
                {
                  "type": "tableRow",
                  "content": [
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "value 1"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "hardBreak"
                            }
                          ]
                        }
                      ]
                    },
                    {
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "value 2"
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
              "type": "paragraph",
              "content": [
                {
                  "type": "text",
                  "text": " "
                }
              ]
            }
          ]
        }),
    );
}
