use quote::quote;
use syn::fold::Fold;
use syn::{parse_quote, Expr, ExprLit, Lit, LitInt, IntSuffix};
use proc_macro2::Literal;

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
                let original_literal = syn_litint_to_proc_macro2_lit(&l);
                let mutator_id = register_global_mutation(format!("LitInt {}", l.value()));
                parse_quote! {
                    <::mutagen_preview::mutator::MutatorLitInt<_>>
                        ::new(#mutator_id, #original_literal)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                }
            }
            _ => fold_expr_default(self, e),
        }
    }
}

// transforms syn::IntLit to proc_macro2::Literal, which is used inside quote!
//
// this reads the suffix of the LitInt and calls the appropriate Literal::X_suffixed function
fn syn_litint_to_proc_macro2_lit(lit: &LitInt) -> Literal {
    let value = lit.value();
    match lit.suffix() {
        IntSuffix::I8 => Literal::i8_suffixed(value as i8),
        IntSuffix::I16 => Literal::i16_suffixed(value as i16),
        IntSuffix::I32 => Literal::i32_suffixed(value as i32),
        IntSuffix::I64 => Literal::i64_suffixed(value as i64),
        IntSuffix::I128 => Literal::i128_suffixed(value as i128),
        IntSuffix::Isize => Literal::isize_suffixed(value as isize),
        IntSuffix::U8 => Literal::u8_suffixed(value as u8),
        IntSuffix::U16 => Literal::u16_suffixed(value as u16),
        IntSuffix::U32 => Literal::u32_suffixed(value as u32),
        IntSuffix::U64 => Literal::u64_suffixed(value as u64),
        IntSuffix::U128 => Literal::u128_suffixed(value as u128),
        IntSuffix::Usize => Literal::usize_suffixed(value as usize),
        IntSuffix::None => Literal::u64_unsuffixed(value),
    }
}

impl MutagenTransformer for MutagenTransformerLitInt {}
