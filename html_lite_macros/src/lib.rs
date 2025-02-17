use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitStr, Token,
};

#[derive(Debug)]
struct Attribute {
    key: Ident,
    value: LitStr,
}
impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { key, value })
    }
}

#[derive(Debug)]
enum Tag {
    Start(Vec<Attribute>, Ident),
    End(Ident),
}
impl Parse for Tag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;

        let end = input.parse::<Token![/]>().is_ok();
        let tag = input.parse::<Ident>()?;

        let mut attributes = vec![];

        if !end {
            while let Ok(attr) = input.parse::<Attribute>() {
                attributes.push(attr);
            }
        }

        input.parse::<Token![>]>()?;

        if end {
            Ok(Self::End(tag))
        } else {
            Ok(Self::Start(attributes, tag))
        }
    }
}

#[derive(Debug)]
struct Text(LitStr);
impl Parse for Text {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let b;
        braced!(b in input);

        Ok(Self(b.parse()?))
    }
}

#[derive(Debug)]
enum Value {
    Text(Text),
    Tag(Tag),
}
impl Parse for Value {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(text) = input.parse() {
            Ok(Value::Text(text))
        } else if let Ok(tag) = input.parse() {
            Ok(Value::Tag(tag))
        } else {
            Err(syn::Error::new(input.span(), "Section could not be parsed"))
        }
    }
}

struct Section {
    value: LitStr,
    italic: bool,
    bold: bool,
    color: Option<LitStr>,
}

struct Sections(Vec<Section>);
impl Parse for Sections {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut values = vec![];
        let mut tags = vec![];

        let mut italic = false;
        let mut bold = false;
        let mut color = None;
        while let Ok(value) = input.parse::<Value>() {
            match value {
                Value::Tag(Tag::Start(mut attribute, start_tag)) => {
                    match start_tag.to_string().as_str() {
                        "i" => italic = true,
                        "b" => bold = true,
                        _ => {}
                    }

                    if let Some(Attribute { key, value }) = attribute.pop() {
                        if key == "color" {
                            color.replace(value);
                        }
                    } else {
                        color = None;
                    }

                    tags.push(start_tag.to_string());
                }
                Value::Tag(Tag::End(end_tag)) => {
                    let Some(start_tag) = tags.pop() else {
                        return Err(syn::Error::new(end_tag.span(), "Unstarted tag"));
                    };
                    if end_tag != start_tag {
                        return Err(syn::Error::new(end_tag.span(), "Mismatched tag"));
                    }

                    match end_tag.to_string().as_str() {
                        "i" => italic = false,
                        "b" => bold = false,
                        _ => {}
                    }
                }
                Value::Text(Text(value)) => {
                    values.push(Section {
                        value,
                        italic,
                        bold,
                        color: color.clone(),
                    });
                }
            }
        }

        Ok(Self(values))
    }
}

#[proc_macro]
pub fn sections(input: TokenStream) -> TokenStream {
    let values = parse_macro_input!(input as Sections);

    let values: Vec<_> = values
        .0
        .into_iter()
        .map(|value| {
            let string = value.value;
            let bold = value.bold;
            let italic = value.italic;
            let color = value
                .color
                .map(|color| quote! { Some(Color::from(Srgba::hex(#color).unwrap())) })
                .unwrap_or(quote! { None });
            quote! {
                ::bevy_html_lite::prelude::Section::new(std::format!(#string), #bold, #italic, #color)
            }
        })
        .collect();
    quote! {
        ::html_lite_sections::prelude::Sections::from_iter([#(#values),*])
    }
    .into()
}
