use quote::quote;
use syn::fold::Fold;
use syn::{parse_quote, Expr, ExprLit, Lit};

use super::default_folds::fold_expr_default;
use super::MutagenTransformer;
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerLitInt {
    pub transform_info: SharedTransformInfo,
}

impl Fold for MutagenTransformerLitInt {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Int(l),
                attrs: _,
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation(format!("LitInt {}", l.value()));
                parse_quote! {
                    <::mutagen_preview::mutator::MutatorLitInt<_>>
                        ::new(#mutator_id, #l)
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
