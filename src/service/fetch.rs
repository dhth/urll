use crate::domain::{Page, PageDetails};
use reqwest::get;
use scraper::{Html, Selector};
use url::Url;

pub async fn fetch_urls(url: &str) -> anyhow::Result<Page> {
    let base_url = Url::parse(url)?;
    let body = get(url).await?.text().await?;

    let document = Html::parse_document(&body);

    #[allow(clippy::unwrap_used)]
    let link_selector = Selector::parse("a").unwrap();

    #[allow(clippy::unwrap_used)]
    let title_selector = Selector::parse("title").unwrap();

    #[allow(clippy::unwrap_used)]
    let og_title_selector = Selector::parse(r#"meta[property="og:title"]"#).unwrap();

    #[allow(clippy::unwrap_used)]
    let og_description_selector = Selector::parse(r#"meta[property="og:description"]"#).unwrap();

    let og_title = document
        .select(&og_title_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|s| s.trim().to_string());

    let description = document
        .select(&og_description_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|s| s.trim().to_string());

    let title = og_title.or_else(|| {
        document
            .select(&title_selector)
            .next()
            .map(|element| element.inner_html().trim().to_string())
    });

    let mut page_urls = Vec::new();
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            let link = base_url.join(href)?;
            if link.as_str().starts_with("https://") || link.as_str().starts_with("http://") {
                page_urls.push(link.to_string());
            }
        }
    }
    page_urls.sort();
    page_urls.dedup();

    let details = PageDetails {
        url: url.to_string(),
        title,
        description,
    };

    Ok(Page { details, page_urls })
}
