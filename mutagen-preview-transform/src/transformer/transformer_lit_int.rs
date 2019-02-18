use quote::quote;
use syn::fold::Fold;
use syn::{parse_quote, Expr, ExprLit, Lit};

use super::default_folds::fold_expr_default;
use super::MutagenTransformer;
use crate::transform_info::register_global_mutation;

pub struct MutagenTransformerLitInt();

impl Fold for MutagenTransformerLitInt {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Int(l),
                attrs: _,
            }) => {
                // WIP: avoid compilation errors for non-`u32` types
                let value = l.value() as u32;
                let mutator_id = register_global_mutation(format!("LitInt {}", value));
                parse_quote! {
                    <::mutagen_preview::mutator::MutatorLitInt<u32>>::new(#mutator_id, #value)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                }
            }
            _ => fold_expr_default(self, e),
        }
    }
}

impl MutagenTransformer for MutagenTransformerLitInt {}
