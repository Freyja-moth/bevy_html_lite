use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, LitStr, Token,
};

struct Attribute {
    name: Ident,
    value: Expr,
}
impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        input.parse::<Token![=]>()?;

        let value;

        braced!(value in input);

        Ok(Self {
            name,
            value: value.parse()?,
        })
    }
}

#[derive(Debug)]
struct Attributes(HashMap<Ident, Expr>);
impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = HashMap::new();

        loop {
            if input.peek(Token![>]) {
                break;
            }

            let Attribute { name, value } = input.parse()?;

            attributes.insert(name, value);
        }

        Ok(Self(attributes))
    }
}

#[derive(Debug)]
enum Tag {
    Start(Ident, Attributes),
    End(Ident),
}
impl Parse for Tag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;

        let is_end = input.parse::<Token![/]>().is_ok();
        let tag = input.parse()?;

        if is_end {
            input.parse::<Token![>]>()?;

            Ok(Self::End(tag))
        } else {
            let attributes = input.parse::<Attributes>()?;

            input.parse::<Token![>]>()?;

            Ok(Self::Start(tag, attributes))
        }
    }
}

#[derive(Debug)]
struct Text(LitStr);
impl Parse for Text {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let b;

        braced!(b in input);

        Ok(Self(b.parse()?))
    }
}

#[derive(Debug)]
enum HtmlWord {
    Text(Text),
    Tag(Tag),
}
impl Parse for HtmlWord {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(text) = input.parse() {
            Ok(Self::Text(text))
        } else if let Ok(tag) = input.parse() {
            Ok(Self::Tag(tag))
        } else {
            Err(syn::Error::new(input.span(), "Section could not be parsed"))
        }
    }
}

#[derive(Debug)]
struct Section {
    text: LitStr,
    tags: Vec<Ident>,
    attributes: HashMap<Ident, Expr>,
}

#[derive(Debug)]
struct Sections(Vec<Section>);
impl Parse for Sections {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sections = vec![];
        let mut tags = vec![];
        let mut attributes = vec![HashMap::new()];

        while let Ok(word) = input.parse::<HtmlWord>() {
            match word {
                HtmlWord::Tag(Tag::Start(start_tag, Attributes(tag_attributes))) => {
                    tags.push(start_tag);
                    attributes.push(tag_attributes);
                }
                HtmlWord::Tag(Tag::End(end_tag)) => {
                    let start_tag = tags
                        .pop()
                        .ok_or(syn::Error::new(end_tag.span(), "Unstarted tag"))?;

                    if start_tag != end_tag {
                        return Err(syn::Error::new(end_tag.span(), "Mismatched tag"));
                    }
                    attributes.pop();
                }
                HtmlWord::Text(Text(text)) => {
                    sections.push(Section {
                        text,
                        tags: tags.clone(),
                        attributes: attributes
                            .iter()
                            .flatten()
                            .map(|(item, expr)| (item.clone(), expr.clone()))
                            .collect(),
                    });
                }
            }
        }

        if input.is_empty() {
            Ok(Self(sections))
        } else {
            Err(syn::Error::new(input.span(), "Unterminated input."))
        }
    }
}

#[proc_macro]
pub fn sections(input: TokenStream) -> TokenStream {
    let Sections(sections) = parse_macro_input!(input as Sections);

    let sections = sections.into_iter().map(|section| {
        let text = &section.text;
        let tags = section.tags.iter().map(Ident::to_string);
        let (names, values): (Vec<String>, Vec<Expr>) = section
            .attributes
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect();

        quote! {
            ::bevy_html_lite::prelude::Section::new(format!(#text))
                #(.with_tag(#tags))*
                #(.with_attribute(#names, #values))*
        }
    });

    quote! {
        [#(#sections),*]
    }
    .into()
}
