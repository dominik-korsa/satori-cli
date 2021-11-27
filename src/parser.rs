pub mod results {
    use scraper::{Html, Selector};

    pub fn get_latest_solution_id(html: &str) -> String {
        let document = Html::parse_document(html);
        let selector =
            Selector::parse(r#"#content .results tr:nth-of-type(2) > td:first-of-type > a"#)
                .unwrap();
        let element = document.select(&selector).next().unwrap();
        element.inner_html()
    }
}
