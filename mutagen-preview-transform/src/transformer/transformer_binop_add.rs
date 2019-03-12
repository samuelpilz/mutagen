use syn::{parse_quote, Expr, ExprBinary, BinOp};

use super::MutagenExprTransformer;
use crate::transform_info::SharedTransformInfo;

pub struct MutagenTransformerBinopAdd {
    pub transform_info: SharedTransformInfo,
}

impl MutagenExprTransformer for MutagenTransformerBinopAdd {
    fn map_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Binary(ExprBinary {
                attrs: _,
                left,
                right,
                op: BinOp::Add(_),
            }) => {
                let mutator_id = self
                    .transform_info
                    .add_mutation("LitBinopAdd".to_string());
                parse_quote! {
                    <::mutagen_preview::mutator::MutatorBinopAdd<_, _>>
                        ::new(#mutator_id, #left, #right)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                }
            }
            _ => e
        }
    }
}
