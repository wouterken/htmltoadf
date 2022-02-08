#[cfg(test)]
#[allow(unused_imports)]
use super::assert_output_json_eq;

#[allow(unused_imports)]
use serde_json::json;

#[test]
fn absolute_image() {
    assert_output_json_eq(
        "<img src='http://www.google.com/test.jpg'/>",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
            {
              "type": "mediaSingle",
              "content": [
                {
                  "type": "media",
                  "attrs": {
                    "url": "http://www.google.com/test.jpg",
                    "type": "external"
                  }
                }
              ]
            }
          ]
        }),
    );
}

#[test]
fn relative_image() {
    assert_output_json_eq(
        "<img src='relative/abc.jpg'>",
        json!({
          "version": 1,
          "type": "doc",
          "content": [
            {
              "type": "mediaSingle",
              "content": [
                {
                  "type": "media",
                  "attrs": {
                    "url": "relative/abc.jpg",
                    "type": "external"
                  }
                }
              ]
            }
          ]
        }),
    );
}
