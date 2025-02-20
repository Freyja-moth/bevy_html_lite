use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, LitStr, Token,
};

enum Attribute {
    Color(LitStr),
    FontSize(LitInt),
    Over(Ident),
    Out(Ident),
    Click(Ident),
}
impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        match name.to_string().as_str() {
            "color" => Ok(Self::Color(input.parse()?)),
            "font_size" => Ok(Self::FontSize(input.parse()?)),
            "over" => Ok(Self::Over(input.parse()?)),
            "out" => Ok(Self::Out(input.parse()?)),
            "click" => Ok(Self::Click(input.parse()?)),
            _ => Err(syn::Error::new(input.span(), "Unrecognised attribute")),
        }
    }
}

enum Tag {
    Start(Ident, Vec<Attribute>),
    End(Ident),
}
impl Parse for Tag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
            Ok(Self::Start(tag, attributes))
        }
    }
}

struct Text(LitStr);
impl Parse for Text {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let b;
        braced!(b in input);

        Ok(Self(b.parse()?))
    }
}

enum Value {
    Text(Text),
    Tag(Tag),
}
impl Parse for Value {
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

#[derive(Clone)]
struct Section {
    text: LitStr,
    bold: bool,
    italic: bool,
    color: Option<LitStr>,
    font_size: Option<LitInt>,
    over: Option<Ident>,
    out: Option<Ident>,
    click: Option<Ident>,
}

struct Sections(Vec<Section>);
impl Parse for Sections {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut values = vec![];
        let mut tags = vec![];

        let mut italic = false;
        let mut bold = false;
        let mut color = None;
        let mut font_size = None;
        let mut over = None;
        let mut out = None;
        let mut click = None;

        while let Ok(value) = input.parse::<Value>() {
            match value {
                Value::Tag(Tag::Start(start_tag, mut attributes)) => {
                    match start_tag.to_string().as_str() {
                        "i" => italic = true,
                        "b" => bold = true,
                        _ => {}
                    }

                    if let Some(attribute) = attributes.pop() {
                        match attribute {
                            Attribute::Color(new_color) => {
                                color.replace(new_color);
                            }
                            Attribute::FontSize(new_size) => {
                                font_size.replace(new_size);
                            }
                            Attribute::Over(new_over) => {
                                over.replace(new_over);
                            }
                            Attribute::Out(new_out) => {
                                out.replace(new_out);
                            }
                            Attribute::Click(new_click) => {
                                click.replace(new_click);
                            }
                        };
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

                    color.take();
                    font_size.take();
                    over.take();
                    out.take();
                    click.take();
                }
                Value::Text(Text(text)) => {
                    values.push(Section {
                        text,
                        italic,
                        bold,
                        color: color.clone(),
                        font_size: font_size.clone(),
                        over: over.clone(),
                        out: out.clone(),
                        click: click.clone(),
                    });
                }
            }
        }

        Ok(Self(values))
    }
}

#[proc_macro]
pub fn sections(input: TokenStream) -> TokenStream {
    let sections = parse_macro_input!(input as Sections);

    let values: Vec<_> = sections
        .0
        .into_iter()
        .map(|value| {
            let string = value.text;
            let bold = value.bold;
            let italic = value.italic;

            let color = value.color.map(|color| quote!{ section.set_color(::bevy::prelude::Color(::bevy::prelude::Srbga::hex(#color))) });
            let font_size = value.font_size.map(|font_size| quote!{ section.set_font_size(#font_size) });
            let over = value.over.map(|over| quote!{ section.set_over(#over) });
            let out = value.out.map(|out| quote!{ section.set_out(#out) });
            let click = value.click.map(|click| quote!{ section.set_click(#click); });

            quote! {
                {
                    let mut section = ::bevy_html_lite::prelude::Section::new(std::format!(#string), #bold, #italic);

                    #color
                    #font_size
                    #over
                    #out
                    #click

                    section
                }
            }
        })
        .collect();

    quote! {
        ::bevy_html_lite::prelude::Sections::from_iter([#(#values),*])
    }
    .into()
}
