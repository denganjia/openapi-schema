# OpenApi-Schema

Rust structure mapped to openapi 

You can use this package will openapi document into rust structure
# Use
```bash
cargo add openapi-schema
```

# Notice

Because some words are reserved words in rust, so will the json structure mapped to rust after structure, rust in the field name is different from some of json
  |Rust|Json|
  | --- | --- |
  | r#type | type |
  | r#enum | enum |
  | r#in | in |
  | reference | $ref |

>For using camelCase in json representation of the field, use snake_case in rust