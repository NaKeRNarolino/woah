use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

#[proc_macro]
pub fn template_encoder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(tokens as TemplateEncoder);

    quote! {
        #parsed
    }.into()
}

struct TemplateEncoder {
    path: String,
    read_templates: HashMap<String, String>
}

impl Parse for TemplateEncoder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let str = input.parse::<syn::LitStr>()?;

        let path = str.value();

        let read = fs_extra::dir::get_dir_content(&path).unwrap();

        let mut hm: HashMap<String, String> = HashMap::new();

        for r in &read.files {
            hm.insert(
                r.clone(),
                fs::read_to_string(r).unwrap()
            );
        }

        Ok(TemplateEncoder { path, read_templates: hm })
    }
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