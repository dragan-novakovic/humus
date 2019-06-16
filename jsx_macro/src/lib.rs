#![recursion_limit = "128"]

use proc_macro_hack::proc_macro_hack;


extern crate proc_macro;

use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;
use snax::{
    SnaxAttribute,
    //  SnaxFragment,
    SnaxItem,
    SnaxSelfClosingTag,
    SnaxTag,
};

#[proc_macro_hack]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    let parsed_content = snax::parse(input).expect("Could not even");

    let output = emit_item(&parsed_content);

    proc_macro::TokenStream::from(output)
}

fn emit_item(item: &SnaxItem) -> TokenStream {
    match item {
        SnaxItem::Tag(tag) => emit_tag(tag),
        SnaxItem::SelfClosingTag(tag) => emit_self_closing_tag(tag),
        SnaxItem::Content(tt) => emit_content(tt),
        SnaxItem::Fragment(fragment) => TokenStream::new(),
    }
}

fn emit_attributes(attributes: &[SnaxAttribute]) -> TokenStream {
    attributes
        .iter()
        .map(|attribute| match attribute {
            SnaxAttribute::Simple { name, value } => {
                let name_literal = Literal::string(&name.to_string());

                quote!(
                    __snax_tag.set_attribute(#name_literal, #value);
                )
            }
        })
        .collect()
}

fn emit_children(children: &[SnaxItem]) -> TokenStream {
    children
        .iter()
        .map(|child| {
            let emitted = emit_item(child);

            quote!(
                __snax_tag.add_child(#emitted);
            )
        })
        .collect()
}

fn emit_self_closing_tag(tag: &SnaxSelfClosingTag) -> TokenStream {
    let attribute_insertions = emit_attributes(&tag.attributes);

    let attributes_len_literal = Literal::usize_unsuffixed(tag.attributes.len());
    let tag_name_literal = Literal::string(&tag.name.to_string());

    quote!({
        let mut __snax_tag = ::humus::HtmlSelfClosingTag {
            name: ::std::borrow::Cow::Borrowed(#tag_name_literal),
            attributes: ::std::collections::HashMap::with_capacity(#attributes_len_literal),
        };

        #attribute_insertions

        ::humus::HtmlContent::SelfClosingTag(__snax_tag)
    })
}

fn emit_tag(tag: &SnaxTag) -> TokenStream {
    let attribute_insertions = emit_attributes(&tag.attributes);
    let child_insertions = emit_children(&tag.children);

    let attributes_len_literal = Literal::usize_unsuffixed(tag.attributes.len());
    let children_len_literal = Literal::usize_unsuffixed(tag.children.len());
    let tag_name_literal = Literal::string(&tag.name.to_string());

    quote!({
        let mut __snax_tag = ::humus::HtmlTag {
            name: ::std::borrow::Cow::Borrowed(#tag_name_literal),
            attributes: ::std::collections::HashMap::with_capacity(#attributes_len_literal),
            children: ::std::vec::Vec::with_capacity(#children_len_literal),
        };

        #attribute_insertions
        #child_insertions

        ::humus::HtmlContent::Tag(__snax_tag)
    })
}


fn emit_content(tt: &TokenTree) -> TokenStream {
    quote!(
        ::humus::render::t(#tt as &str)
    )
}
