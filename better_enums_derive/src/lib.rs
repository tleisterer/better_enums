mod model;
mod parse;
mod settings;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse_macro_input};

use crate::parse::{domain, extract_repr};
use crate::settings::{parse_setting, prepare};

fn better_enums_impl(
    attr: TokenStream,
    mut input: ItemEnum,
) -> Result<TokenStream, syn::Error> {
    if input.generics.lt_token.is_some() || input.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "better_enums: enum cannot be generic",
        ));
    }

    let setting = parse_setting(attr)?;
    let repr = extract_repr(&input.attrs, &input.ident)?;
    let bounds = domain(&repr);
    let variants = prepare(setting.as_ref(), &mut input, bounds)?;

    let code = setting.generate(&input.ident, &repr, &variants);
    Ok(quote! { #input #code })
}

#[proc_macro_attribute]
pub fn better_enums(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match better_enums_impl(attr.into(), parse_macro_input!(input as ItemEnum)) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
