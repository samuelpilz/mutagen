use syn::{parse_quote, Expr, ExprLit, Lit, LitBool};

use super::MutagenExprTransformer;
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerLitBool {
    pub transform_info: SharedTransformInfo,
}

// transforms bool literals to mutator expressions
impl MutagenExprTransformer for MutagenTransformerLitBool {
    fn map_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, span: _ }),
                attrs: _,
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation(format!("LitBool {} -> {}", value, !value));
                parse_quote! {
                    ::mutagen_preview::mutator::MutatorLitBool::new(#mutator_id, #value)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                }
            }
            _ => e,
        }
    }
}
