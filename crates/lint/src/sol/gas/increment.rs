use super::PostIncrementDecrement;
use crate::{
    linter::{EarlyLintPass, LintContext},
    sol::{Severity, SolLint},
};
use solar::ast::{Expr, ExprKind, UnOpKind};

declare_forge_lint!(
    POST_INCREMENT_DECREMENT,
    Severity::Gas,
    "post-increment-decrement",
    "use pre-increment/decrement instead of post-increment/decrement"
);

impl<'ast> EarlyLintPass<'ast> for PostIncrementDecrement {
    fn check_expr(&mut self, ctx: &LintContext, expr: &'ast Expr<'ast>) {
        if let ExprKind::Unary(op, _) = &expr.kind
            && matches!(op.kind, UnOpKind::PostInc | UnOpKind::PostDec)
        {
            ctx.emit(&POST_INCREMENT_DECREMENT, expr.span);
        }
    }
}