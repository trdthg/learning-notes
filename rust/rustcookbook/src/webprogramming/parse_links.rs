use error_chain::error_chain;
use std::collections::HashSet;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        Url(url::ParseError);
        JoinError(tokio::task::JoinError);
    }
}

pub async fn get_all_links() -> Result<()> {
    use select::document::Document;
    use select::predicate::Name;

    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

use select::document::Document;
use select::predicate::Name;
use url::{Position, Url};

async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

pub async fn check_dead_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_str()).await?;
    Ok(res.status() == reqwest::StatusCode::OK)
}

pub async fn get_all_ok_links() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let document: Document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;
    let base_parser = Url::options().base_url(Some(&base_url));
    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|e| e.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    let mut tasks = vec![];
    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_dead_link(&link).await.unwrap_or(false) {
                println!("{} is OK", link);
            } else {
                println!("{} is Broken", link);
            }
        }));
    }

    for task in tasks {
        task.await?
    }
    Ok(())
}

#[tokio::test]
pub async fn test() {
    // get_all_links().await;
    get_all_ok_links().await;
}
