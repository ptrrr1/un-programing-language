use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    expr::Expr,
    interpreter::Interpreter,
    tokens::{Token, TokenType},
    types::{lambda_callable::LambdaCallable, value::Value},
};

pub trait ExprVisitor<R, E> {
    fn visit_assignment(&mut self, env: Rc<RefCell<E>>, target: &Expr, expr: &Expr) -> R;
    fn visit_binary(
        &mut self,
        env: Rc<RefCell<E>>,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> R;
    fn visit_unary(&mut self, env: Rc<RefCell<E>>, operator: &Token, right: &Expr) -> R;
    fn visit_grouping(&mut self, env: Rc<RefCell<E>>, inner: &Expr) -> R;
    fn visit_literal(&mut self, env: Rc<RefCell<E>>, inner: &Token) -> R;
    fn visit_variable(&mut self, env: Rc<RefCell<E>>, inner: &Token) -> R;
    fn visit_exposed_fn(&mut self, env: Rc<RefCell<E>>, inner: &Token) -> R;
    fn visit_conditional(
        &mut self,
        env: Rc<RefCell<E>>,
        condition: &Expr,
        true_branch: &Expr,
        false_branch: &Expr,
    ) -> R;
    fn visit_call(&mut self, env: Rc<RefCell<E>>, callee: &Expr, paren: &Token, args: &[Expr])
    -> R;
    fn visit_lambda(&mut self, env: Rc<RefCell<E>>, params: &[Token], body: &Expr) -> R;
}

impl ExprVisitor<Value, Enviroment> for Interpreter {
    fn visit_assignment(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        target: &Expr,
        expr: &Expr,
    ) -> Value {
        let val = expr.accept(env.clone(), self);

        match target {
            Expr::Variable(token) => match &token.token_type {
                TokenType::Identifier(s) => {
                    match self.locals.get(s) {
                        Some(depth) => {
                            Enviroment::define_at(env, s, val.clone(), *depth);
                        }
                        None => {
                            self.env.borrow_mut().update_var(s, val.clone());
                        }
                    }

                    val
                }
                _ => unreachable!(),
            },
            _ => panic!("Invalid assignment target"),
        }
    }

    fn visit_binary(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Value {
        let l = left.accept(env.clone(), self);

        match operator.token_type {
            TokenType::Or => {
                if l.get_truthyness() {
                    l
                } else {
                    right.accept(env.clone(), self)
                }
            }
            TokenType::And => {
                if l.get_truthyness() {
                    right.accept(env, self)
                } else {
                    l
                }
            }
            TokenType::EqualEqual => {
                let r = right.accept(env, self);

                Value::Bool(l == r)
            }
            TokenType::BangEqual => {
                let r = right.accept(env, self);

                Value::Bool(l != r)
            }
            TokenType::Lesser => {
                let r = right.accept(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l < r)
            }
            TokenType::LesserEqual => {
                let r = right.accept(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l <= r)
            }
            TokenType::Greater => {
                let r = right.accept(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l > r)
            }
            TokenType::GreaterEqual => {
                let r = right.accept(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l >= r)
            }
            TokenType::Plus => {
                let r = right.accept(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                    (Value::String(left), Value::String(right)) => Value::String(left + &right),
                    _ => panic!(
                        "Invalid Type for Binary Operation 'Addition', Expected only Number or String",
                    ),
                }
            }
            TokenType::Minus => {
                let r = right.accept(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
                    _ => {
                        panic!("Invalid Type for Binary Operation 'Subtraction', Expected Number",)
                    }
                }
            }
            TokenType::Star => {
                let r = right.accept(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
                    _ => panic!(
                        "Invalid Type for Binary Operation 'Multiplication', Expected Number",
                    ),
                }
            }
            TokenType::Slash => {
                let r = right.accept(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left / right),
                    _ => {
                        panic!("Invalid Type for Binary Operation 'Division', Expected Number")
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_unary(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        operator: &Token,
        right: &Expr,
    ) -> Value {
        let r = right.accept(env, self);

        match operator.token_type {
            TokenType::Minus => match r {
                Value::Number(v) => Value::Number(-v),
                _ => panic!("Invalid Type for Unary, Expected Number"),
            },
            TokenType::Not => Value::Bool(!r.get_truthyness()),
            _ => unreachable!(),
        }
    }

    fn visit_grouping(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Expr) -> Value {
        inner.accept(env, self)
    }

    fn visit_literal(&mut self, _env: Rc<RefCell<Enviroment>>, inner: &Token) -> Value {
        Value::try_from(inner.token_type.clone()).unwrap()
    }

    fn visit_variable(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Token) -> Value {
        match &inner.token_type {
            TokenType::Identifier(s) => self.look_up_var(env.clone(), s),
            _ => unreachable!(),
        }
    }

    // TODO: Deal with it...
    fn visit_exposed_fn(&mut self, _env: Rc<RefCell<Enviroment>>, _innerr: &Token) -> Value {
        todo!()
    }

    fn visit_conditional(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        condition: &Expr,
        true_branch: &Expr,
        false_branch: &Expr,
    ) -> Value {
        let c = condition.accept(env.clone(), self);
        if c.get_truthyness() {
            true_branch.accept(env.clone(), self)
        } else {
            false_branch.accept(env.clone(), self)
        }
    }

    fn visit_call(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        callee: &Expr,
        _paren: &Token,
        args: &[Expr],
    ) -> Value {
        let eval_callee = callee.accept(env.clone(), self);

        let mut eval_args: Vec<Value> = vec![];
        for arg in args {
            eval_args.push(arg.accept(env.clone(), self));
        }

        match eval_callee {
            Value::Callee(f) => {
                if f.arity() == eval_args.len()
                    || f.arity() >= eval_args.len() && f.is_variable_arity()
                {
                    return f.call(self, eval_args);
                }

                panic!("Wrong number of arguments") // TODO: Expand err
            }
            _ => panic!("Can only call functions"),
        }
    }

    fn visit_lambda(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        params: &[Token],
        body: &Expr,
    ) -> Value {
        let lambda = LambdaCallable::new(params.to_vec(), body.clone(), env.clone());

        Value::Callee(Rc::new(lambda))
    }
}
