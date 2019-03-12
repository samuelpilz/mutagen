use syn::{parse_quote, Expr, ExprLit, Lit};

use super::MutagenExprTransformer;
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerLitInt {
    pub transform_info: SharedTransformInfo,
}

impl MutagenExprTransformer for MutagenTransformerLitInt {
    fn map_expr(&mut self, e: Expr) -> Expr {
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
            _ => e,
        }
    }
}
