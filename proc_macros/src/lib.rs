mod builder;

use proc_macro::{Ident, Span};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::iter::Peekable;
use std::path::PathBuf;
use proc_macro2::{Group, TokenStream, TokenTree};
use proc_macro2::token_stream::IntoIter;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream, Parser};
use syn::{parse_macro_input, Expr};
use syn::token::At;
use crate::builder::Builder;

#[proc_macro]
pub fn template_encoder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(tokens as TemplateEncoder);

    quote! {
        #parsed
    }.into()
}

/// The Woah macro is used for a nicer syntax when building stuff.
/// ```rust
/// use woah::woah;
///
/// woah! {
///     @Item {
///         id = ("namespace", "value");
///         components = sjson! {
///             //...
///         };
///     }
/// }
#[proc_macro]
pub fn woah(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input2 = TokenStream2::from(input);

    let mut iter = input2.clone().into_iter();
    if let Some(TokenTree::Punct(punct)) = iter.next() {
        if punct.as_char() == '@' {
            input2 = iter.collect();
        }
    }

    fn rewrite_stream(input_stream: ParseStream) -> syn::Result<TokenStream2> {
        let mut output = TokenStream2::new();

        while !input_stream.is_empty() {
            if input_stream.peek(syn::Token![@]) {
                if input_stream.peek2(syn::Ident) {
                    let _at_token: At = input_stream.parse()?;

                    let content_to_wrap = if input_stream.peek(syn::Ident) && input_stream.peek2(syn::token::Brace) {
                        let ident: syn::Ident = input_stream.parse()?;
                        let group: Group = input_stream.parse()?;

                        let rewritten_inner = rewrite_stream.parse2(group.stream())?;
                        let mut rewritten_group = Group::new(group.delimiter(), rewritten_inner);
                        rewritten_group.set_span(group.span());

                        quote! { #ident #rewritten_group }
                    } else {
                        let expr: Expr = input_stream.parse()?;
                        quote! { #expr }
                    };

                    let macro_call = quote! {
                        woah! { #content_to_wrap }
                    };

                    output.extend(macro_call);
                } else {
                    let next_token: TokenTree = input_stream.parse()?;
                    output.extend([next_token]);
                }
            } else if input_stream.peek(syn::token::Brace)
                || input_stream.peek(syn::token::Paren)
                || input_stream.peek(syn::token::Bracket) {

                let group: Group = input_stream.parse()?;

                let rewritten_inner = rewrite_stream.parse2(group.stream())?;

                let mut rewritten_group = Group::new(group.delimiter(), rewritten_inner);
                rewritten_group.set_span(group.span());

                output.extend([TokenTree::Group(rewritten_group)]);
            } else {
                let next_token: TokenTree = input_stream.parse()?;
                output.extend([next_token]);
            }
        }

        Ok(output)
    }

    let rewritten_tokens = match rewrite_stream.parse2(input2) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error().into(),
    };

    let final_input = match syn::parse2::<Builder>(rewritten_tokens) {
        Ok(decl) => decl.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    };

    final_input
}

struct TemplateEncoder {
    path: String,
    read_templates: HashMap<String, String>
}

impl Parse for TemplateEncoder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let str = input.parse::<syn::LitStr>()?;

        let relative_path = str.value();

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| input.error("Failed to get CARGO_MANIFEST_DIR"))?;

        let full_path = PathBuf::from(manifest_dir).join(&relative_path);

        let mut hm = HashMap::new();
        read_dir_recursive(&full_path, &mut hm)
            .map_err(|e| input.error(format!("Failed to read template directory '{relative_path}': {e}")))?;

        Ok(TemplateEncoder {
            path: relative_path,
            read_templates: hm,
        })
    }
}

fn read_dir_recursive(dir: &std::path::Path, hm: &mut HashMap<String, String>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                read_dir_recursive(&path, hm)?;
            } else {
                let content = fs::read_to_string(&path)?;
                hm.insert(path.to_string_lossy().to_string(), content);
            }
        }
    }
    Ok(())
}


impl ToTokens for TemplateEncoder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (name, value) in &self.read_templates {
            let rn = name.strip_prefix(
                &format!("{}/", &self.path)
            ).unwrap();
            tokens.append_all(quote! {
                tera.add_raw_template(#rn, #value).unwrap();
            });
        }
    }
}