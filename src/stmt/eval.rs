use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    expr::Expr,
    interpreter::Interpreter,
    stmt::{Stmt, signal::Signal},
    tokens::{Token, TokenType},
    types::{un_callable::UnCallable, value::Value},
};

pub trait StmtVisitor<R, E> {
    fn visit_expr(&mut self, env: Rc<RefCell<E>>, expr: &Expr) -> R;
    fn visit_print(&mut self, env: Rc<RefCell<E>>, expr: &Expr) -> R;
    fn visit_var(&mut self, env: Rc<RefCell<E>>, target: &Token, expr: &Expr) -> R;
    fn visit_block(&mut self, env: Rc<RefCell<E>>, stmts: &[Stmt]) -> R;
    fn visit_conditional(
        &mut self,
        env: Rc<RefCell<E>>,
        condition: &Expr,
        true_branch: &[Stmt],
        false_branch: &Option<Vec<Stmt>>,
    ) -> R;
    fn visit_while(&mut self, env: Rc<RefCell<E>>, condition: &Expr, stmts: &[Stmt]) -> R;
    fn visit_function(
        &mut self,
        env: Rc<RefCell<E>>,
        identifier: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> R;
    fn visit_return(&mut self, env: Rc<RefCell<E>>, expr: &Expr) -> R;
    fn visit_break(&mut self) -> R;
    // fn visit_continue(&self) -> R;
}

impl StmtVisitor<Signal, Enviroment> for Interpreter {
    fn visit_expr(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> Signal {
        expr.accept(env.clone(), self);

        Signal::Normal
    }

    fn visit_print(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> Signal {
        println!("{}", expr.accept(env.clone(), self));

        Signal::Normal
    }

    fn visit_var(&mut self, env: Rc<RefCell<Enviroment>>, target: &Token, expr: &Expr) -> Signal {
        if let TokenType::Identifier(s) = &target.token_type {
            let val = expr.accept(env.clone(), self);
            env.borrow_mut().define_var(s, val);
        }

        Signal::Normal
    }

    fn visit_block(&mut self, env: Rc<RefCell<Enviroment>>, stmts: &[Stmt]) -> Signal {
        let new_env = Rc::new(RefCell::new(Enviroment::default()));
        new_env.borrow_mut().set_outer(env.clone());

        for stmt in stmts {
            let r = stmt.accept(new_env.clone(), self);

            if !matches!(r, Signal::Normal) {
                return r;
            }
        }

        Signal::Normal
    }

    fn visit_conditional(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        condition: &Expr,
        true_branch: &[Stmt],
        false_branch: &Option<Vec<Stmt>>,
    ) -> Signal {
        let c = condition.accept(env.clone(), self);
        if c.get_truthyness() {
            for stmt in true_branch {
                let r = stmt.accept(env.clone(), self);

                if !matches!(r, Signal::Normal) {
                    return r;
                }
            }
        } else if let Some(f) = false_branch {
            for stmt in f {
                let r = stmt.accept(env.clone(), self);

                if !matches!(r, Signal::Normal) {
                    return r;
                }
            }
        }

        Signal::Normal
    }

    fn visit_while(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        condition: &Expr,
        stmts: &[Stmt],
    ) -> Signal {
        'outer: while condition.accept(env.clone(), self).get_truthyness() {
            for stmt in stmts {
                let r = stmt.accept(env.clone(), self);

                match r {
                    Signal::Return(_) => return r,
                    Signal::Break => break 'outer,
                    // Signal::Continue => continue, // TODO: Continue; doesn't work
                    Signal::Normal => (),
                }
            }
        }

        Signal::Normal
    }

    fn visit_function(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        identifier: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Signal {
        match &identifier.token_type {
            TokenType::Identifier(s) => {
                let un_callable =
                    UnCallable::new(s.clone(), params.to_vec(), body.to_vec(), env.clone());
                let val = Value::Callee(Rc::new(un_callable));

                env.borrow_mut().define_var(s, val);
            }
            _ => panic!("Not an identifier for a function"),
        }

        Signal::Normal
    }

    fn visit_return(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> Signal {
        let v = expr.accept(env.clone(), self);

        Signal::Return(v)
    }

    fn visit_break(&mut self) -> Signal {
        Signal::Break
    }
}
