use openapi_schema::v2;

mod tests {
    use super::*;
    use serde_json::from_str;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_transtion() {
        let mut file = match File::open("./tests/swagger.json") {
            Ok(file) => file,
            Err(e) => panic!("无法打开文件: {}", e),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => println!("文件读取完成"),
            Err(e) => panic!("无法读取文件: {}", e),
        }

        let swagger = from_str::<v2::Swagger>(&contents);
        assert_eq!("2.0", swagger.unwrap().swagger);
    }
}
