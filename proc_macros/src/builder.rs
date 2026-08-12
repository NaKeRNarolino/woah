use proc_macro::{Punct, TokenStream};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{braced, bracketed, custom_keyword, parenthesized, parse::{Parse, ParseStream}, punctuated::Punctuated, Expr, Path, Token};
use syn::token::Paren;

mod keywords {
    use syn::custom_keyword;

    custom_keyword!(map);
}

pub struct Declaration {
    pub ident: Path,
}

pub struct Builder {
    pub decl: Declaration,
    pub props: Vec<Property>,
    pub exprs: Vec<Expr>
}

pub enum Property {
    Expr(Ident, Expr, bool),
    // Viola(Ident, Builder),
}

impl Parse for Builder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
        }

        let struct_ident = input.parse::<Path>()?;

        if input.peek(Paren) {
            let content;
            parenthesized!(content in input);

            let props = content.parse_terminated(|stream| stream.parse::<Expr>(), Token![,])?;

            Ok(Builder {
                decl: Declaration { ident: struct_ident },
                props: Vec::new(),
                exprs: props.into_iter().collect()
            })
        } else {
            let content;
            braced!(content in input);

            let mut props = Vec::new();

            while !content.is_empty() {
                match content.parse::<Property>() {
                    Ok(prop) => {
                        props.push(prop);
                    }
                    Err(_) => {
                        while !content.is_empty() && !content.peek(Token![;]) {
                            if let Ok(tt) = content.parse::<proc_macro2::TokenTree>() {
                                if let proc_macro2::TokenTree::Ident(ident) = tt {
                                    props.push(Property::Expr(ident, syn::parse_quote! { Default::default() }, false));
                                }
                            }
                        }
                    }
                }

                if content.peek(Token![;]) {
                    content.parse::<Token![;]>()?;
                }
            }

            Ok(Self {
                decl: Declaration { ident: struct_ident },
                props,
                exprs: Vec::new()
            })
        }
    }
}

impl Parse for Property {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            let mut maps = false;

            if input.peek(keywords::map) {
                maps = true;
                input.parse::<keywords::map>()?;
            }

            let expr = content.parse::<Expr>()?;
            return Ok(Property::Expr(ident, expr, maps));
            // if content.peek(Token![@]) {
            //     content.parse::<Token![@]>()?;
            //     let nested_builder = content.parse::<Builder>()?;
            //     return Ok(Property::Viola(ident, nested_builder));
            // } else {
            // }
        }

        if !input.peek(Token![=]) {
            let dummy_expr = syn::parse_quote! { Default::default() };
            return Ok(Property::Expr(ident, dummy_expr, false));
        }

        input.parse::<Token![=]>()?;

        let mut maps = false;

        if input.peek(keywords::map) {
            maps = true;
            input.parse::<keywords::map>()?;
        }

        let expr = input.parse::<Expr>()?;
        Ok(Property::Expr(ident, expr, maps))
        // if input.peek(Token![@]) {
        //     input.parse::<Token![@]>()?;
        //     let nested_builder = input.parse::<Builder>()?;
        //     Ok(Property::Viola(ident, nested_builder))
        // } else {
        //
        // }
    }
}

impl ToTokens for Builder {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let struct_ident = &self.decl.ident;
        let mut chain = quote! { #struct_ident::builder() };

        if !self.exprs.is_empty() {
            let exprs = &self.exprs;
            tokens.append_all(quote! {
                #struct_ident(#(#exprs.into()),*)
            })
        }

        for prop in &self.props {
            let (field_ident, value_tokens) = get_prop_val(prop);

            let mut resolved_ident = field_ident.clone();
            resolved_ident.set_span(field_ident.span());

            chain.extend(quote! { .#resolved_ident(#value_tokens) });
        }

        chain.extend(quote! { .build().unwrap() });
        tokens.extend(chain);
    }
}

fn get_prop_val(prop: &Property) -> (&Ident, TokenStream2) {
    match prop {
        // Property::Viola(ident, val) => (ident, val.to_token_stream()),
        Property::Expr(ident, val, maps) => (ident, apply_attribs(val, *maps)),
    }
}



fn apply_attribs(val: &Expr, maps: bool) -> TokenStream2 {
    if maps {
        quote! {
            #val.into_iter().map(Into::into).collect()
        }
    } else {
        quote! { #val }
    }
}