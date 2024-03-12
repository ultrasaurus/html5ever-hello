use html5ever::{
    Attribute,
    buffer_queue::BufferQueue, tendril::{SliceExt, StrTendril},
    tokenizer::{Tag, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts},
};
use  string_cache::Atom;
use markup5ever::LocalNameStaticSet;

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

fn find_attr_by_name<'a>(attrs: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr| &attr.name.local == name)
}

fn find_attr_by_namevalue<'a>(attrs: &'a Vec<Attribute>, name: &str, value: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|&attr|
        &attr.name.local == name &&
        attr.value == value.to_tendril()
    )
}

impl TokenSink for StyleExtractor {
    type Handle = ();

    #[allow(clippy::match_same_arms)]
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        let _link:Atom<LocalNameStaticSet> = "link".into();
        match token {
            Token::TagToken(
                Tag{name: _link ,attrs,..}) => {
                    if find_attr_by_namevalue(&attrs,
                        "rel", "stylesheet").is_some() {
                        if let Some(href_attr) = find_attr_by_name(&attrs, "href") {
                            self.styles.push(href_attr.value.clone())
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
