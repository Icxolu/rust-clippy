use clippy_utils::{diagnostics::span_lint_and_sugg, fn_def_id, get_parent_node};
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind, Node};
use rustc_lint::LateContext;
use rustc_middle::ty::{Clause, PredicateKind};
use rustc_span::{sym, Span};

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, recv: &Expr<'_>, call_span: Span) {
    let Some(Node::Expr(parent)) = get_parent_node(cx.tcx, expr.hir_id) else {return};
    let Some(id) = fn_def_id(cx, parent) else {return};

    let args = match parent.kind {
        ExprKind::Call(_, args) | ExprKind::MethodCall(_, _, args, _) => args,
        _ => &[],
    };

    // find the argument index of the `expr` in the function / method call
    if let Some(arg_idx) = args.iter().position(|e| e.hir_id == expr.hir_id).map(|i| {
        if matches!(parent.kind, ExprKind::MethodCall(_, _, _, _)) {
            i + 1
        } else {
            i
        }
    }) {
        // extract the input types of the function/method call that contains
        // `expr`
        let inputs = cx
            .tcx
            .liberate_late_bound_regions(id, cx.tcx.fn_sig(id).subst_identity())
            .inputs();

        // map IntoIterator generic bounds to their signature
        // types and check whether the argument type is an
        // `IntoIterator`
        if cx
            .tcx
            .param_env(id)
            .caller_bounds()
            .into_iter()
            .filter_map(|p| {
                if let PredicateKind::Clause(Clause::Trait(t)) = p.kind().skip_binder()
                            && cx.tcx.is_diagnostic_item(sym::IntoIterator,t.trait_ref.def_id) {
                                Some(t.self_ty())
                            } else {
                                None
                            }
            })
            .any(|ty| ty == inputs[arg_idx])
        {
            span_lint_and_sugg(
                cx,
                super::NEEDLESS_INTO_ITER,
                call_span.with_lo(recv.span.hi()),
                "unneccessary `.into_iter()` call in generic argument",
                "remove this call",
                String::new(),
                Applicability::MachineApplicable,
            );
        }
    }
}
