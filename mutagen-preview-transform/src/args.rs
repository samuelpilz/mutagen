//! parse arguments for the `#[mutate]` attribute and gather all information necessary to transform the source code.
//!
//! Please refer to the customization documentation about the format of arguments.

use proc_macro2::TokenStream;

use crate::transform_info::{SharedTransformInfo, GLOBAL_TRANSFORM_INFO};
use crate::transformer::*;

mod arg_ast;
mod arg_parse;

use arg_parse::{ArgOptions, Conf, Transformers};
use transformer_order::TRANSFORMER_ORDER;

pub struct MutagenArgs {
    pub transformers: Vec<Box<dyn MutagenTransformer>>,
    pub transform_info: SharedTransformInfo,
}

impl MutagenArgs {
    /// parse the arguments of the `#[mutate]` attribute
    pub fn args_from_attr(args: TokenStream) -> MutagenArgs {
        let options = ArgOptions::parse(args).expect("invalid options");

        // WIP: better error messages if format is not valid

        let transform_info: SharedTransformInfo = match options.conf {
            Conf::Global => {
                let transform_info = GLOBAL_TRANSFORM_INFO.clone_shared();
                transform_info.with_default_mutagen_file();
                transform_info
            }
            Conf::Local => Default::default(),
        };

        let transformers = match options.transformers {
            Transformers::All => all_transformers(),
            Transformers::Only(list) => {
                let mut transformers = list.transformers;
                transformers.sort_by_key(|t| TRANSFORMER_ORDER[t]);
                transformers
            }
            Transformers::Not(list) => {
                let mut transformers = all_transformers();
                for l in &list.transformers {
                    transformers.remove_item(l);
                }
                transformers
            }
        };

        let transformers = transformers
            .iter()
            .map(|t| mk_transformer(&t, &vec![], transform_info.clone_shared()))
            .collect();

        MutagenArgs {
            transformers,
            transform_info,
        }
    }
}

// this funciton gives a vec of all transformers, in order they are executed
fn all_transformers() -> Vec<String> {
    vec![
        "lit_int",
        "lit_bool",
        "binop_add",
    ]
        .iter()
        .map(ToString::to_string)
        .collect()
}

mod transformer_order {

    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use super::all_transformers;

    lazy_static! {
        pub static ref TRANSFORMER_ORDER: HashMap<String, usize> = {
            all_transformers().into_iter().enumerate().map(|(i,s)| (s,i)).collect()
        };
    }

}

fn mk_transformer(
    transformer_name: &str,
    _transformer_args: &Vec<String>,
    transform_info: SharedTransformInfo,
) -> Box<dyn MutagenTransformer> {
    match transformer_name {
        "lit_int" => box MutagenTransformerLitInt {
            transform_info: transform_info,
        },
        "lit_bool" => box MutagenTransformerLitBool {
            transform_info: transform_info,
        },
        "binop_add" => box MutagenTransformerBinopAdd {
            transform_info: transform_info,
        },
        _ => panic!("unknown transformer {}", transformer_name),
    }
}
