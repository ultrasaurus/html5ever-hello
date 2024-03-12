use html5ever::{
    buffer_queue::BufferQueue, tendril::{SliceExt, StrTendril}, tokenizer::{Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts}, LocalName
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

impl TokenSink for StyleExtractor {
    type Handle = ();

    #[allow(clippy::match_same_arms)]
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            Token::TagToken(
                Tag{name,attrs,..}) => {
            // Token::TagToken(tag) => {
            //     let Tag {
            //         kind,
            //         name,
            //         self_closing: _self_closing,
            //         attrs,
            //     } = tag;
            //println!("tag: {}\t attrs: {:?}", name, attrs);
            if name.to_lowercase() == "link" {
                let maybe_style = attrs.iter()
                        .find(|&attr|
                            attr.name.local.to_lowercase() == "rel" &&
                            attr.value == "stylesheet".to_tendril()
                        );
                if let Some(style_attr) = maybe_style {
                    let maybe_href = attrs.iter()
                        .find(|&attr|
                            attr.name.local.to_lowercase() == "href"
                        );
                    if let Some(href_attr) = maybe_href {
                            self.styles.push(href_attr.value.clone())
                    }
                }
            }

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
