use core::str::FromStr;

use stack_core::prelude::*;

pub fn module() -> Module {
  let mut module = Module::new(Symbol::from_ref("scope"));

  module
    .add_func(Symbol::from_ref("where"), |engine, mut context, expr| {
      let symbol = context.stack_pop(&expr)?;

      match symbol.kind {
        ExprKind::Symbol(ref x) => {
          if Intrinsic::from_str(x.as_str()).is_ok() {
            context.stack_push(ExprKind::String("intrinsic".into()).into())
          } else if engine
            .module(&Symbol::new(
              x.as_str().split(':').next().unwrap_or_default().into(),
            ))
            .is_some()
          {
            context.stack_push(ExprKind::String("module".into()).into())
          } else if context.let_get(*x).is_some() {
            context.stack_push(ExprKind::String("let".into()).into())
          } else if context.scope_item(*x).is_some() {
            context.stack_push(ExprKind::String("scope".into()).into())
          } else {
            context.stack_push(ExprKind::Nil.into())
          }
        }
        _ => context.stack_push(ExprKind::Nil.into()),
      }
      .map(|_| context)
    })
    .add_func(Symbol::from_ref("dump"), |_, mut context, _| {
      let items: Vec<Expr> = context
        .scope_items()
        .map(|(name, content)| {
          let list: Vec<Expr> = vec![
            ExprKind::Symbol(*name).into(),
            content
              .borrow()
              .val()
              .map(|e| e.kind)
              .unwrap_or(ExprKind::Nil)
              .into(),
          ];

          ExprKind::List(list).into()
        })
        .collect();

      context
        .stack_push(ExprKind::List(items).into())
        .map(|_| context)
    });

  module
}
