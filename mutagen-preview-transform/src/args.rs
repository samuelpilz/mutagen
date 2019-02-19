//! parse arguments for the `#[mutate]` attribute

// use proc_macro2::TokenTree;
use crate::transformer::*;

pub struct MutagenArgs {
    pub transformers: Vec<Box<dyn MutagenTransformer>>
}

impl MutagenArgs {

    /// parse the arguments of the `#[mutate]` attribute
    pub fn args_from_attr(_args: proc_macro2::TokenStream) -> MutagenArgs {
        // WIP: parse args and construct config
        let transformers : Vec<Box<dyn MutagenTransformer>> =
        vec![
            box MutagenTransformerLitInt(),
            box MutagenTransformerLitBool(),
            box MutagenTransformerStmt(),
        ];
        MutagenArgs { transformers }
    }
}
