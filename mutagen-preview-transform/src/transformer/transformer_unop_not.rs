use syn::{parse_quote, Expr, ExprUnary, UnOp};

use super::MutagenExprTransformer;
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerUnopNot {
    pub transform_info: SharedTransformInfo,
}

impl MutagenExprTransformer for MutagenTransformerUnopNot {
    fn map_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Unary(ExprUnary {
                attrs: _,
                expr,
                op: UnOp::Not(_),
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation("LitUnopNot".to_string());
                parse_quote! {
                    <::mutagen_preview::mutator::MutatorUnopNot<_>>
                        ::new(#mutator_id, #expr)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                }
            }
            _ => e
        }
    }
}
