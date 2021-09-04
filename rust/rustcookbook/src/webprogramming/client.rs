use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

pub async fn try_request() -> Result<()> {
    // 同步
    let res = reqwest::blocking::get("http://httpbin.org/get")?;
    let body = res.text()?;
    // 异步
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    
    let body = res.text().await?;
    println!("Body:\n{}", body);



    Ok(())
}

// pub fn set_params() -> Result<()> {
//     use serde::Deserialize;

//     use std::collections::HashMap;
//     use url::Url;
//     use reqwest::Client;
//     use reqwest::header::{UserAgent, Authorization, Bearer};

//     header! { (XPoweredBy, "X-Powered-By") => [String] }

//     #[derive(Deserialize, Debug)]
//     pub struct HeadersEcho {
//         pub headers: HashMap<String, String>,
//     }
//     let url = Url::parse_with_params("http://httpbin.org/headers",
//     &[("lang", "rust"), ("browser", "servo")])?;

//     let mut response = Client::new()
//         .get(url)
//         .header(UserAgent::new("Rust-test"))
//         .header(Authorization(Bearer { token: "DEadBEEfc001cAFeEDEcafBAd".to_owned() }))
//         .header(XPoweredBy("Guybrush Threepwood".to_owned()))
//         .send()?;

//     let out: HeadersEcho = response.json()?;
//     assert_eq!(out.headers["Authorization"],
//     "Bearer DEadBEEfc001cAFeEDEcafBAd");
//     assert_eq!(out.headers["User-Agent"], "Rust-test");
//     assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
//     assert_eq!(response.url().as_str(),
//     "http://httpbin.org/headers?lang=rust&browser=servo");

//     println!("{:?}", out);
//     Ok(())
// }