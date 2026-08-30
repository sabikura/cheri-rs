// TODO: Figure out if this is worth to pursue, seems like the type system suffices
//       for sealing objects.

use quote::quote;
use syn::parse_macro_input;
use syn::ItemImpl;

pub(crate) fn attribute(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let impl_ = parse_macro_input!(input as ItemImpl);

    quote! { #impl_ }.into()
}

pub(crate) fn derive(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
}
