use proc_macro::TokenStream;
use quote::quote;

/// Just adds the linkme::distributed_slice macro
#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut dist_slice: TokenStream = quote!(#[linkme::distributed_slice(mylinkmetypes::GET_HANDLERS)]).into();
    dist_slice.extend(item);
    dist_slice
}