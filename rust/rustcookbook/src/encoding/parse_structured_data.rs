use error_chain::error_chain;

error_chain! {
    foreign_links {
        Json(serde_json::Error);
        Toml(toml::de::Error);
    }
}

pub fn parse_normal_json() -> Result<()> {
    use serde_json::json;
    use serde_json::Value;

    let j = r#"{
        "userid": 100,
        "verified": true,
        "access_privileges": [
        "user",
        "admin"
        ]
    }"#;

    let parsed: Value = serde_json::from_str(j)?;
    let expected = json!({
        "userid": 100,
        "verified": true,
        "access_privileges": [
        "user",
        "admin"
        ]
    });

    assert_eq!(parsed, expected);
    assert_eq!(*expected.get("userid").unwrap(), json!(100));
    println!("{}", expected["userid"]);
    assert_eq!(expected["userid"], 100);
    Ok(())
}

pub fn parse_toml() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;
    use serde::Deserialize;
    use std::collections::HashMap;
    #[derive(Deserialize, Debug)]
    struct Package {
        name: String,
        version: String,
        authors: Vec<String>,
    }
    #[derive(Deserialize, Debug)]
    struct Config {
        package: Package,
        dependencies: HashMap<String, String>,
    }
    let package_info: Config = toml::from_str(toml_content)?;
    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");
    Ok(())
}

// 以小端模式（低位模式）字节顺序读写整数??? 过了先

#[test]
pub fn test() {
    parse_normal_json();
    parse_toml();
}
