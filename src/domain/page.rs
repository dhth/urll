pub struct Page {
    pub details: PageDetails,
    pub page_urls: Vec<String>,
}

#[derive(Clone)]
pub struct PageDetails {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
}
