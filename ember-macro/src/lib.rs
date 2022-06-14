use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Path};

fn ember_path() -> Path {
    match crate_name("ember").expect("ember not found in Cargo.toml") {
        FoundCrate::Itself => parse_quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());

            parse_quote!(::#ident)
        }
    }
}

#[proc_macro_derive(PhaseLabel)]
pub fn derive_phase_label(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ember = ember_path();

    let ident = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #ember::label::PhaseLabel for #ident #ty_generics #where_clause {
            fn raw_label(&self) -> #ember::label::RawPhaseLabel {
                #ember::label::RawPhaseLabel::from(#ember::label::RawLabel::new(self))
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
