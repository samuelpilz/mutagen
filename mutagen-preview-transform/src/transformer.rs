use syn::fold::Fold;
use syn::{Expr, ItemFn};

mod transformer_binop_add;
mod transformer_lit_bool;
mod transformer_lit_int;
mod transformer_unop_not;

use transformer_binop_add::MutagenTransformerBinopAdd;
use transformer_lit_bool::MutagenTransformerLitBool;
use transformer_lit_int::MutagenTransformerLitInt;
use transformer_unop_not::MutagenTransformerUnopNot;

use crate::transform_info::SharedTransformInfo;
use crate::args::arg_options::Transformers;

pub enum MutagenTransformer {
    Expr(Box<dyn MutagenExprTransformer>),
}

pub struct MutagenTransformerBundle {
    pub expr_transformers: Vec<Box<dyn MutagenExprTransformer>>,
}

pub trait MutagenExprTransformer {
    fn map_expr(&mut self, expr: Expr) -> Expr;
}

impl Fold for MutagenTransformerBundle {
    fn fold_expr(&mut self, e: Expr) -> Expr {

        // transform content of the expression first
        let mut result = match e {
            Expr::Box(e0) => Expr::Box(self.fold_expr_box(e0)),
            Expr::InPlace(e0) => Expr::InPlace(self.fold_expr_in_place(e0)),
            Expr::Array(e0) => Expr::Array(self.fold_expr_array(e0)),
            Expr::Call(e0) => Expr::Call(self.fold_expr_call(e0)),
            Expr::MethodCall(e0) => Expr::MethodCall(self.fold_expr_method_call(e0)),
            Expr::Tuple(e0) => Expr::Tuple(self.fold_expr_tuple(e0)),
            Expr::Binary(e0) => Expr::Binary(self.fold_expr_binary(e0)),
            Expr::Unary(e0) => Expr::Unary(self.fold_expr_unary(e0)),
            Expr::Lit(e0) => Expr::Lit(self.fold_expr_lit(e0)),
            Expr::Cast(e0) => Expr::Cast(self.fold_expr_cast(e0)),
            Expr::Type(e0) => Expr::Type(self.fold_expr_type(e0)),
            Expr::Let(e0) => Expr::Let(self.fold_expr_let(e0)),
            Expr::If(e0) => Expr::If(self.fold_expr_if(e0)),
            Expr::While(e0) => Expr::While(self.fold_expr_while(e0)),
            Expr::ForLoop(e0) => Expr::ForLoop(self.fold_expr_for_loop(e0)),
            Expr::Loop(e0) => Expr::Loop(self.fold_expr_loop(e0)),
            Expr::Match(e0) => Expr::Match(self.fold_expr_match(e0)),
            Expr::Closure(e0) => Expr::Closure(self.fold_expr_closure(e0)),
            Expr::Unsafe(e0) => Expr::Unsafe(self.fold_expr_unsafe(e0)),
            Expr::Block(e0) => Expr::Block(self.fold_expr_block(e0)),
            Expr::Assign(e0) => Expr::Assign(self.fold_expr_assign(e0)),
            Expr::AssignOp(e0) => Expr::AssignOp(self.fold_expr_assign_op(e0)),
            Expr::Field(e0) => Expr::Field(self.fold_expr_field(e0)),
            Expr::Index(e0) => Expr::Index(self.fold_expr_index(e0)),
            Expr::Range(e0) => Expr::Range(self.fold_expr_range(e0)),
            Expr::Path(e0) => Expr::Path(self.fold_expr_path(e0)),
            Expr::Reference(e0) => Expr::Reference(self.fold_expr_reference(e0)),
            Expr::Break(e0) => Expr::Break(self.fold_expr_break(e0)),
            Expr::Continue(e0) => Expr::Continue(self.fold_expr_continue(e0)),
            Expr::Return(e0) => Expr::Return(self.fold_expr_return(e0)),
            Expr::Macro(e0) => Expr::Macro(self.fold_expr_macro(e0)),
            Expr::Struct(e0) => Expr::Struct(self.fold_expr_struct(e0)),
            Expr::Repeat(e0) => Expr::Repeat(self.fold_expr_repeat(e0)),
            Expr::Paren(e0) => Expr::Paren(self.fold_expr_paren(e0)),
            Expr::Group(e0) => Expr::Group(self.fold_expr_group(e0)),
            Expr::Try(e0) => Expr::Try(self.fold_expr_try(e0)),
            Expr::Async(e0) => Expr::Async(self.fold_expr_async(e0)),
            Expr::TryBlock(e0) => Expr::TryBlock(self.fold_expr_try_block(e0)),
            Expr::Yield(e0) => Expr::Yield(self.fold_expr_yield(e0)),
            Expr::Verbatim(e0) => Expr::Verbatim(self.fold_expr_verbatim(e0)),
        };

        // call all transformers on this expression
        for t in &mut self.expr_transformers {
            result = t.map_expr(result);
        }
        result
        // WIP: preserve span
    }
}

impl MutagenTransformerBundle {
    pub fn mutagen_transform_item_fn(&mut self, target: ItemFn) -> ItemFn {
        self.fold_item_fn(target)
    }

    pub fn new(transformers: Transformers, transform_info: &SharedTransformInfo) -> Self {
        let transformers = match transformers {
            Transformers::All => all_transformers(),
            Transformers::Only(list) => {
                let mut transformers = list.transformers;
                transformers.sort_by_key(|t| TRANSFORMER_ORDER[t]);
                transformers
            },
            Transformers::Not(list) => {
                let mut transformers = all_transformers();
                for l in &list.transformers {
                    transformers.remove_item(l);
                }
                transformers
            }
        };

        let mut expr_transformers = Vec::new();
        for t in &transformers {
            let t = mk_transformer(t, &[], transform_info.clone_shared());
            match t {
                MutagenTransformer::Expr(t) => expr_transformers.push(t)
            }
        }

        Self {
            expr_transformers
        }
    }
}

fn mk_transformer(
    transformer_name: &str,
    _transformer_args: &[String],
    transform_info: SharedTransformInfo,
) -> MutagenTransformer {
    match transformer_name {
        "lit_int" => MutagenTransformer::Expr(box MutagenTransformerLitInt {
            transform_info: transform_info,
        }),
        "lit_bool" => MutagenTransformer::Expr(box MutagenTransformerLitBool {
            transform_info: transform_info,
        }),
        "unop_not" => MutagenTransformer::Expr(box MutagenTransformerUnopNot {
            transform_info: transform_info,
        }),
        "binop_add" => MutagenTransformer::Expr(box MutagenTransformerBinopAdd {
            transform_info: transform_info,
        }),
        _ => panic!("unknown transformer {}", transformer_name),
    }
}

// this funciton gives a vec of all transformers, in order they are executed
fn all_transformers() -> Vec<String> {
    ["lit_int", "lit_bool", "unop_not", "binop_add"]
        .iter()
        .map(ToString::to_string)
        .collect()
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSFORMER_ORDER: HashMap<String, usize> = {
        all_transformers()
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect()
    };
}
