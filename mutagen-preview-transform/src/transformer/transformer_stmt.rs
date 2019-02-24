use quote::quote;
use syn::fold::Fold;
use syn::{parse_quote, Stmt};

use super::default_folds::fold_stmt_default;
use super::MutagenTransformer;
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerStmt {
    pub transform_info: SharedTransformInfo,
}

impl Fold for MutagenTransformerStmt {
    fn fold_stmt(&mut self, e: Stmt) -> Stmt {
        match e {
            Stmt::Semi(e, _) => {
                // WIP: implement check for return-statement
                let mutator_id = self.transform_info.add_mutation(format!("Stmt rem"));
                parse_quote! {
                    if <::mutagen_preview::mutator::MutatorStmt>::new(#mutator_id)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                    {
                        #e;
                    }
                }
            }
            _ => fold_stmt_default(self, e),
        }
    }
}

impl MutagenTransformer for MutagenTransformerStmt {}
