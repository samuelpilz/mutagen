//! helper function to fold an `Expr` as the default folder would do,
//! but continue using the original visitor in the sub-AST
//!
//! copied from the source from syn::gen::fold and syn::gen_helper::fold
//! these modules are not exported, so the code has been copied

use proc_macro2::Span;
use syn::fold::Fold;
use syn::{Expr, Stmt, Token};

// fold an expression
pub fn fold_expr_default<V: Fold + ?Sized>(visitor: &mut V, i: Expr) -> Expr {
    match i {
        Expr::Box(_binding_0) => Expr::Box(visitor.fold_expr_box(_binding_0)),
        Expr::InPlace(_binding_0) => Expr::InPlace(visitor.fold_expr_in_place(_binding_0)),
        Expr::Array(_binding_0) => Expr::Array(visitor.fold_expr_array(_binding_0)),
        Expr::Call(_binding_0) => Expr::Call(visitor.fold_expr_call(_binding_0)),
        Expr::MethodCall(_binding_0) => Expr::MethodCall(visitor.fold_expr_method_call(_binding_0)),
        Expr::Tuple(_binding_0) => Expr::Tuple(visitor.fold_expr_tuple(_binding_0)),
        Expr::Binary(_binding_0) => Expr::Binary(visitor.fold_expr_binary(_binding_0)),
        Expr::Unary(_binding_0) => Expr::Unary(visitor.fold_expr_unary(_binding_0)),
        Expr::Lit(_binding_0) => Expr::Lit(visitor.fold_expr_lit(_binding_0)),
        Expr::Cast(_binding_0) => Expr::Cast(visitor.fold_expr_cast(_binding_0)),
        Expr::Type(_binding_0) => Expr::Type(visitor.fold_expr_type(_binding_0)),
        Expr::Let(_binding_0) => Expr::Let(visitor.fold_expr_let(_binding_0)),
        Expr::If(_binding_0) => Expr::If(visitor.fold_expr_if(_binding_0)),
        Expr::While(_binding_0) => Expr::While(visitor.fold_expr_while(_binding_0)),
        Expr::ForLoop(_binding_0) => Expr::ForLoop(visitor.fold_expr_for_loop(_binding_0)),
        Expr::Loop(_binding_0) => Expr::Loop(visitor.fold_expr_loop(_binding_0)),
        Expr::Match(_binding_0) => Expr::Match(visitor.fold_expr_match(_binding_0)),
        Expr::Closure(_binding_0) => Expr::Closure(visitor.fold_expr_closure(_binding_0)),
        Expr::Unsafe(_binding_0) => Expr::Unsafe(visitor.fold_expr_unsafe(_binding_0)),
        Expr::Block(_binding_0) => Expr::Block(visitor.fold_expr_block(_binding_0)),
        Expr::Assign(_binding_0) => Expr::Assign(visitor.fold_expr_assign(_binding_0)),
        Expr::AssignOp(_binding_0) => Expr::AssignOp(visitor.fold_expr_assign_op(_binding_0)),
        Expr::Field(_binding_0) => Expr::Field(visitor.fold_expr_field(_binding_0)),
        Expr::Index(_binding_0) => Expr::Index(visitor.fold_expr_index(_binding_0)),
        Expr::Range(_binding_0) => Expr::Range(visitor.fold_expr_range(_binding_0)),
        Expr::Path(_binding_0) => Expr::Path(visitor.fold_expr_path(_binding_0)),
        Expr::Reference(_binding_0) => Expr::Reference(visitor.fold_expr_reference(_binding_0)),
        Expr::Break(_binding_0) => Expr::Break(visitor.fold_expr_break(_binding_0)),
        Expr::Continue(_binding_0) => Expr::Continue(visitor.fold_expr_continue(_binding_0)),
        Expr::Return(_binding_0) => Expr::Return(visitor.fold_expr_return(_binding_0)),
        Expr::Macro(_binding_0) => Expr::Macro(visitor.fold_expr_macro(_binding_0)),
        Expr::Struct(_binding_0) => Expr::Struct(visitor.fold_expr_struct(_binding_0)),
        Expr::Repeat(_binding_0) => Expr::Repeat(visitor.fold_expr_repeat(_binding_0)),
        Expr::Paren(_binding_0) => Expr::Paren(visitor.fold_expr_paren(_binding_0)),
        Expr::Group(_binding_0) => Expr::Group(visitor.fold_expr_group(_binding_0)),
        Expr::Try(_binding_0) => Expr::Try(visitor.fold_expr_try(_binding_0)),
        Expr::Async(_binding_0) => Expr::Async(visitor.fold_expr_async(_binding_0)),
        Expr::TryBlock(_binding_0) => Expr::TryBlock(visitor.fold_expr_try_block(_binding_0)),
        Expr::Yield(_binding_0) => Expr::Yield(visitor.fold_expr_yield(_binding_0)),
        Expr::Verbatim(_binding_0) => Expr::Verbatim(visitor.fold_expr_verbatim(_binding_0)),
    }
}

pub fn fold_stmt_default<V: Fold + ?Sized>(visitor: &mut V, i: Stmt) -> Stmt {
    match i {
        Stmt::Local(_binding_0) => Stmt::Local(visitor.fold_local(_binding_0)),
        Stmt::Item(_binding_0) => Stmt::Item(visitor.fold_item(_binding_0)),
        Stmt::Expr(_binding_0) => Stmt::Expr(visitor.fold_expr(_binding_0)),
        Stmt::Semi(_binding_0, _binding_1) => Stmt::Semi(
            visitor.fold_expr(_binding_0),
            Token ! [ ; ](tokens_helper(visitor, &_binding_1.spans)),
        ),
    }
}

fn tokens_helper<F: Fold + ?Sized, S: Spans>(folder: &mut F, spans: &S) -> S {
    spans.fold(folder)
}

trait Spans {
    fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self;
}

impl Spans for Span {
    fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
        folder.fold_span(*self)
    }
}

impl Spans for [Span; 1] {
    fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
        [folder.fold_span(self[0])]
    }
}

impl Spans for [Span; 2] {
    fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
        [folder.fold_span(self[0]), folder.fold_span(self[1])]
    }
}

impl Spans for [Span; 3] {
    fn fold<F: Fold + ?Sized>(&self, folder: &mut F) -> Self {
        [
            folder.fold_span(self[0]),
            folder.fold_span(self[1]),
            folder.fold_span(self[2]),
        ]
    }
}
