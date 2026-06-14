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
                      "type": "tableCell",
                      "content": [
                        {
                          "type": "paragraph"
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

#[test]
fn test_empty_table_cell_is_paragraph() {
    assert_output_json_eq(
        "<table><tr><td></td></tr></table>",
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
                          "type": "paragraph"
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

#[test]
fn test_table_cell_span_attributes() {
    assert_output_json_eq(
        r#"<table>
            <tr><th colspan="2">Header</th></tr>
            <tr><td rowspan="2">A</td><td>B</td></tr>
            <tr><td>C</td></tr>
        </table>"#,
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
                      "type": "tableHeader",
                      "attrs": {
                        "colspan": 2
                      },
                      "content": [
                        {
                          "type": "paragraph",
                          "content": [
                            {
                              "type": "text",
                              "text": "Header"
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
                      "attrs": {
                        "rowspan": 2
                      },
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
                              "text": "C"
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

#[test]
fn test_empty_table_cell_preserves_span_attributes() {
    assert_output_json_eq(
        r#"<table><tr><td colspan="2" rowspan="3"></td></tr></table>"#,
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
                      "attrs": {
                        "colspan": 2,
                        "rowspan": 3
                      },
                      "content": [
                        {
                          "type": "paragraph"
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
