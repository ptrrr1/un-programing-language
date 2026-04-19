use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    expr::Expr,
    interpreter::Interpreter,
    stmt::{Stmt, signal::Signal},
    tokens::{Token, TokenType},
    types::{un_callable::UnCallable, value::Value},
};

pub trait StmtVisitor<R> {
    fn visit_expr(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> R;
    fn visit_print(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> R;
    fn visit_var(&mut self, env: Rc<RefCell<Enviroment>>, target: &Token, expr: &Expr) -> R;
    fn visit_block(&mut self, env: Rc<RefCell<Enviroment>>, stmts: &[Stmt]) -> R;
    fn visit_conditional(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        condition: &Expr,
        true_branch: &[Stmt],
        false_branch: &Option<Vec<Stmt>>,
    ) -> R;
    fn visit_while(&mut self, env: Rc<RefCell<Enviroment>>, condition: &Expr, stmts: &[Stmt]) -> R;
    #[allow(clippy::too_many_arguments)] // TODO: WHAT?!
    fn visit_for(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        identifier: &Token,
        start: &Expr,
        end: &Expr,
        step: &Expr,
        condition: &Token,
        stmts: &[Stmt],
    ) -> R;
    fn visit_function(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        identifier: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> R;
    fn visit_return(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> R;
    fn visit_break(&mut self) -> R;
    // fn visit_continue(&self) -> R;
}

impl Stmt {
    pub fn eval<R>(&self, env: Rc<RefCell<Enviroment>>, visitor: &mut impl StmtVisitor<R>) -> R {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(env, expr),
            Stmt::Print(expr) => visitor.visit_print(env, expr),
            Stmt::Var { target, expr } => visitor.visit_var(env, target, expr),
            Stmt::Block(stmts) => visitor.visit_block(env, stmts),
            Stmt::Conditional {
                condition,
                true_branch,
                false_branch,
            } => visitor.visit_conditional(env, condition, true_branch, false_branch),
            Stmt::While { condition, stmts } => visitor.visit_while(env, condition, stmts),
            Stmt::For {
                identifier,
                start,
                end,
                step,
                condition,
                stmts,
            } => visitor.visit_for(env, identifier, start, end, step, condition, stmts),
            Stmt::Function {
                identifier,
                params,
                body,
            } => visitor.visit_function(env, identifier, params, body),
            Stmt::Return(expr) => visitor.visit_return(env, expr),
            Stmt::Break => visitor.visit_break(),
        }
    }
}

impl StmtVisitor<Signal> for Interpreter {
    fn visit_expr(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> Signal {
        expr.eval(env.clone(), self);

        Signal::Normal
    }

    fn visit_print(&mut self, env: Rc<RefCell<Enviroment>>, expr: &Expr) -> Signal {
        println!("{}", expr.eval(env.clone(), self));

        Signal::Normal
    }

    fn visit_var(&mut self, env: Rc<RefCell<Enviroment>>, target: &Token, expr: &Expr) -> Signal {
        if let TokenType::Identifier(s) = &target.token_type {
            let val = expr.eval(env.clone(), self);
            env.borrow_mut().define_var(s, val);
        }

        Signal::Normal
    }

    fn visit_block(&mut self, env: Rc<RefCell<Enviroment>>, stmts: &[Stmt]) -> Signal {
        let new_env = Rc::new(RefCell::new(Enviroment::default()));
        new_env.borrow_mut().set_outer(env.clone());

        for stmt in stmts {
            let r = stmt.eval(new_env.clone(), self);

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
        let c = condition.eval(env.clone(), self);
        if c.get_truthyness() {
            for stmt in true_branch {
                let r = stmt.eval(env.clone(), self);

                if !matches!(r, Signal::Normal) {
                    return r;
                }
            }
        } else if let Some(f) = false_branch {
            for stmt in f {
                let r = stmt.eval(env.clone(), self);

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
        'outer: while condition.eval(env.clone(), self).get_truthyness() {
            for stmt in stmts {
                let r = stmt.eval(env.clone(), self);

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

    fn visit_for(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        identifier: &Token,
        start: &Expr,
        end: &Expr,
        step: &Expr,
        condition: &Token,
        stmts: &[Stmt],
    ) -> Signal {
        let var_decl = Stmt::var(identifier.clone(), start.clone());

        let st = Stmt::expr(Expr::assignment(
            Expr::variable(identifier.clone()),
            Expr::binary(
                Expr::variable(identifier.clone()),
                Token::new(TokenType::Plus, (0, 0)), // TODO: Handle this pos
                step.clone(),
            ),
        ));

        let mut stmts_cl = stmts.to_vec();
        stmts_cl.push(st);

        let condition = Expr::binary(
            Expr::variable(identifier.clone()),
            condition.clone(),
            end.clone(),
        );

        let while_stmt = Stmt::while_stmt(condition, stmts_cl);

        Stmt::block(vec![var_decl, while_stmt]).eval(env.clone(), self)
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
        let v = expr.eval(env.clone(), self);

        Signal::Return(v)
    }

    fn visit_break(&mut self) -> Signal {
        Signal::Break
    }
}
