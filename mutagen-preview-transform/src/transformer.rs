use syn::fold::Fold;
use syn::ItemFn;

mod default_folds;
mod transformer_binop_add;
mod transformer_lit_bool;
mod transformer_lit_int;
mod transformer_stmt;

pub use transformer_binop_add::MutagenTransformerBinopAdd;
pub use transformer_lit_bool::MutagenTransformerLitBool;
pub use transformer_lit_int::MutagenTransformerLitInt;
pub use transformer_stmt::MutagenTransformerStmt;

/// trait for transformers that insert mutators.
///
/// currently, this trait is empty, but we might add required functions in the future
pub trait MutagenTransformer: Fold {
    fn mutagen_transform_item_fn(&mut self, target: ItemFn) -> ItemFn {
        self.fold_item_fn(target)
    }
}
