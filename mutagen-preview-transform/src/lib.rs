#![feature(box_syntax)]

extern crate proc_macro;
use syn::{parse_macro_input, ItemFn};

mod args;
mod transform_info;
mod transformer;
use args::args;
use transform_info::GLOBAL_TRANSFORM_INFO;
use transformer::*;

#[proc_macro_attribute]
pub fn mutate(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use quote::ToTokens;

    // read args and initialize transformers
    let x = args(attr.into());
    println!("args: {:#?}", &x);
    let mut transformers: [Box<dyn MutagenTransformer>; 3] = [
        box MutagenTransformerLitInt(),
        box MutagenTransformerLitBool(),
        box MutagenTransformerStmt(),
    ];

    // setup global info
    GLOBAL_TRANSFORM_INFO
        .lock()
        .unwrap()
        .with_default_mutagen_file();

    // run transformers one after the other
    let mut result = parse_macro_input!(item as ItemFn);
    for t in &mut transformers {
        result = t.mutagen_transform_item_fn(result)
    }
    let result = result.into_token_stream().into();

    println!("mutations: {:?}", GLOBAL_TRANSFORM_INFO.lock().unwrap());
    result
}
