use error_chain::error_chain;

use url::Url;

error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
    }
    errors {
        CannotBeABase
    }
}

pub fn get_url() -> Result<()> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}


// 方法2: 从url标识符截取url片段
fn get_base_url2(url: &Url) -> Result<Url> {
    let base_url = Some(url.as_ref())
        .map_or_else(|| Url::parse(&url[..url::Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

// 方法1: 直接分片截取
pub fn get_base_url() -> Result<Url> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let mut url = Url::parse(full)?;

    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }

    url.set_query(Some("page=2"));
    assert_eq!(url.as_str(), "https://example.com/products?page=2");
    assert_eq!(url.query(), Some("page=2"));

    Ok(url)
}


pub fn get_url_source() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(url::Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    Ok(())
}