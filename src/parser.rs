use pulldown_cmark::{Event, Parser, Tag};
use scraper::{Html, Selector};

pub fn extract_links(content: &str) -> Vec<String> {
    let parser = Parser::new(content);

    parser
        .filter_map(|event| match event {
            Event::Start(Tag::Link { dest_url, .. }) => Some(dest_url.to_string()),
            _ => None,
        })
        .collect()
}

pub fn extract_title(html_contents: &str) -> Option<String> {
    let html_parts = Html::parse_document(html_contents);
    let selector = Selector::parse("title").ok()?;

    html_parts
        .select(&selector)
        .next()
        .map(|content| content.inner_html())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_links_from_markdown_returns_collection_of_links() {
        let md = r#"
# Test Document
Here is a [Google](https://google.com) link and a [broken link](https://esto-no-existe.com.ar).
        "#;

        let links = extract_links(md);

        assert_eq!(links.len(), 2, "Parser should have found 2 links");
        assert_eq!(links[0], "https://google.com");
        assert_eq!(links[1], "https://esto-no-existe.com.ar");
    }

    #[test]
    fn extract_no_links_returns_empty_collection() {
        let md = "# No links here\nJust some good old plain text.";
        let links = extract_links(md);
        assert!(links.is_empty());
    }

    #[test]
    fn extract_title_from_html() {
        let html = r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <title>Lambda Class Residency</title>
                    </head>
                    <body>
                        <h1>Hello world</h1>
                    </body>
                </html>
            "#;
        let title = extract_title(html).unwrap();

        assert_eq!(title, String::from("Lambda Class Residency"));
    }

    #[test]
    fn extract_title_when_its_missing() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="utf-8">
                </head>
                <body>
                    <h1>Página sin título</h1>
                </body>
            </html>
        "#;

        let title = extract_title(html);

        assert!(title.is_none(), "Should be None");
    }
}
