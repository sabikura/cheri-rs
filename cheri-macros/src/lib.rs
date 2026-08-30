#![deny(warnings)]

mod sealed;

/// This proc-macro attribute applies to structs that derive the `Sealed` trait
#[proc_macro_attribute]
pub fn sealed(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    sealed::attribute(args, input)
}

#[proc_macro_derive(Sealed)]
pub fn derive_sealed(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sealed::derive(item)
}
