pub fn day(html: &str) -> String {
    let doc = scraper::Html::parse_document(html);

    let selector = scraper::Selector::parse("span.day-text").unwrap();
    let elem = doc.select(&selector).next().unwrap();

    return elem.inner_html();
}
