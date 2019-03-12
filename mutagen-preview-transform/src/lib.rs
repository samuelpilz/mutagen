#![feature(box_syntax)]
#![feature(vec_remove_item)]
#![feature(specialization)]

extern crate proc_macro;
use syn::{parse_macro_input, ItemFn};

mod args;
mod transform_info;
mod transformer;
use args::MutagenArgs;

#[proc_macro_attribute]
pub fn mutate(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use quote::ToTokens;

    // read args and initialize transformers
    let mut args = MutagenArgs::args_from_attr(attr.into());

    // run transformers one after the other
    let input = parse_macro_input!(item as ItemFn);
    let result = args.transformer_bundle.mutagen_transform_item_fn(input);
    let result = result.into_token_stream().into();

    result
}
