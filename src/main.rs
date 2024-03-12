use html5ever::{
    buffer_queue::BufferQueue, tendril::{SliceExt, StrTendril}, tokenizer::{Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts}, Attribute, LocalName
};

struct StyleExtractor {
    styles: Vec<StrTendril>
}
impl StyleExtractor {
    pub fn new() -> Self {
        StyleExtractor {
            styles: Vec::new()
        }
    }
}

fn find_attribute_by_name<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| &attr.name.local == name)
}


impl TokenSink for StyleExtractor {
    type Handle = ();


    #[allow(clippy::match_same_arms)]
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            Token::TagToken(
                Tag{name,attrs,..}) => {
                    if name.to_lowercase() == "link" {
                        let style = attrs.iter()
                                .find(|&attr|
                                    &attr.name.local == "rel" &&
                                    attr.value == "stylesheet".to_tendril()
                                );
                        if style.is_some() {
                            let maybe_href = find_attribute_by_name(&attrs, "href");
                            if let Some(href_attr) = maybe_href {
                                self.styles.push(href_attr.value.clone())
                            }
                        }
                    };
            },
            _ => {

            }
        }
        TokenSinkResult::Continue
    }
}
fn main() {
    println!("Hello, world!");
    const HTML_INPUT: &str = r#"
<html>
    <head>
        <link rel="stylesheet" type="text/css" href="style/main.css">
    </head>
    <body>
        <p>This is a paragraph</p>
        <p><b>bold</b></p>
    </body>
</html>"#;


    let mut input = BufferQueue::new();
    input.push_back(StrTendril::from(HTML_INPUT));

    let mut tokenizer = Tokenizer::new(
    StyleExtractor::new(),
    TokenizerOpts::default(),
    );
    let _handle = tokenizer.feed(&mut input);
    tokenizer.end();

    println!("{:?}",tokenizer.sink.styles);

}
