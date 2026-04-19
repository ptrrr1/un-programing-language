use std::{cell::RefCell, rc::Rc};

use crate::{
    enviroment::Enviroment,
    expr::Expr,
    interpreter::Interpreter,
    tokens::{Token, TokenType},
    types::{lambda_callable::LambdaCallable, value::Value},
};

pub trait ExprVisitor<R> {
    fn visit_assignment(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        target: &Rc<Expr>,
        expr: &Rc<Expr>,
    ) -> R;
    fn visit_binary(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        left: &Rc<Expr>,
        operator: &Token,
        right: &Rc<Expr>,
    ) -> R;
    fn visit_unary(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        operator: &Token,
        right: &Rc<Expr>,
    ) -> R;
    fn visit_grouping(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Rc<Expr>) -> R;
    fn visit_literal(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Token) -> R;
    fn visit_variable(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Token) -> R;
    fn visit_exposed_fn(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Token) -> R;
    fn visit_conditional(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        condition: &Rc<Expr>,
        true_branch: &Rc<Expr>,
        false_branch: &Rc<Expr>,
    ) -> R;
    fn visit_call(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        callee: &Rc<Expr>,
        paren: &Token,
        args: &[Expr],
    ) -> R;
    fn visit_lambda(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        params: &[Token],
        body: &Rc<Expr>,
    ) -> R;
}

impl Expr {
    pub fn eval<R>(&self, env: Rc<RefCell<Enviroment>>, visitor: &mut impl ExprVisitor<R>) -> R {
        match self {
            Expr::Assignment { target, expr } => visitor.visit_assignment(env, target, expr),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(env, left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(env, operator, right),
            Expr::Grouping(expr) => visitor.visit_grouping(env, expr),
            Expr::Literal(token) => visitor.visit_literal(env, token),
            Expr::Variable(token) => visitor.visit_variable(env, token),
            Expr::ExposedFn(token) => visitor.visit_exposed_fn(env, token),
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => visitor.visit_conditional(env, condition, true_branch, false_branch),
            Expr::Call {
                callee,
                paren,
                args,
            } => visitor.visit_call(env, callee, paren, args),
            Expr::Lambda { params, body } => visitor.visit_lambda(env, params, body),
        }
    }
}

impl ExprVisitor<Value> for Interpreter {
    fn visit_assignment(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        target: &Rc<Expr>,
        expr: &Rc<Expr>,
    ) -> Value {
        let val = expr.eval(env.clone(), self);

        match target.as_ref() {
            Expr::Variable(token) => match &token.token_type {
                TokenType::Identifier(s) => {
                    env.borrow().clone().update_var(s, val.clone());
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
        left: &Rc<Expr>,
        operator: &Token,
        right: &Rc<Expr>,
    ) -> Value {
        let l = left.eval(env.clone(), self);

        match operator.token_type {
            TokenType::Or => {
                if l.get_truthyness() {
                    l
                } else {
                    right.eval(env.clone(), self)
                }
            }
            TokenType::And => {
                if l.get_truthyness() {
                    right.eval(env, self)
                } else {
                    l
                }
            }
            TokenType::EqualEqual => {
                let r = right.eval(env, self);

                Value::Bool(l == r)
            }
            TokenType::BangEqual => {
                let r = right.eval(env, self);

                Value::Bool(l != r)
            }
            TokenType::Lesser => {
                let r = right.eval(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l < r)
            }
            TokenType::LesserEqual => {
                let r = right.eval(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l <= r)
            }
            TokenType::Greater => {
                let r = right.eval(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l > r)
            }
            TokenType::GreaterEqual => {
                let r = right.eval(env, self);

                if l.get_type() != r.get_type() {
                    panic!("PartialOrd for Different Types");
                }

                Value::Bool(l >= r)
            }
            TokenType::Plus => {
                let r = right.eval(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                    (Value::String(left), Value::String(right)) => Value::String(left + &right),
                    _ => panic!(
                        "Invalid Type for Binary Operation 'Addition', Expected only Number or String",
                    ),
                }
            }
            TokenType::Minus => {
                let r = right.eval(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
                    _ => {
                        panic!("Invalid Type for Binary Operation 'Subtraction', Expected Number",)
                    }
                }
            }
            TokenType::Star => {
                let r = right.eval(env, self);

                match (l, r) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
                    _ => panic!(
                        "Invalid Type for Binary Operation 'Multiplication', Expected Number",
                    ),
                }
            }
            TokenType::Slash => {
                let r = right.eval(env, self);

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
        right: &Rc<Expr>,
    ) -> Value {
        let r = right.eval(env, self);

        match operator.token_type {
            TokenType::Minus => match r {
                Value::Number(v) => Value::Number(-v),
                _ => panic!("Invalid Type for Unary, Expected Number"),
            },
            TokenType::Not => Value::Bool(!r.get_truthyness()),
            _ => unreachable!(),
        }
    }

    fn visit_grouping(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Rc<Expr>) -> Value {
        inner.eval(env, self)
    }

    fn visit_literal(&mut self, _env: Rc<RefCell<Enviroment>>, inner: &Token) -> Value {
        Value::try_from(inner.token_type.clone()).unwrap()
    }

    fn visit_variable(&mut self, env: Rc<RefCell<Enviroment>>, inner: &Token) -> Value {
        match inner.get_token_type() {
            TokenType::Identifier(s) => match env.borrow().get_var_val(&s) {
                Some(v) => v,
                None => panic!("Undefined Variable"),
            },
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
        condition: &Rc<Expr>,
        true_branch: &Rc<Expr>,
        false_branch: &Rc<Expr>,
    ) -> Value {
        let c = condition.eval(env.clone(), self);
        if c.get_truthyness() {
            true_branch.eval(env.clone(), self)
        } else {
            false_branch.eval(env.clone(), self)
        }
    }

    fn visit_call(
        &mut self,
        env: Rc<RefCell<Enviroment>>,
        callee: &Rc<Expr>,
        _paren: &Token,
        args: &[Expr],
    ) -> Value {
        let eval_callee = callee.eval(env.clone(), self);

        let mut eval_args: Vec<Value> = vec![];
        for arg in args {
            eval_args.push(arg.eval(env.clone(), self));
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
        body: &Rc<Expr>,
    ) -> Value {
        let lambda = LambdaCallable::new(params.to_vec(), body.clone(), env.clone());

        Value::Callee(Rc::new(lambda))
    }
}
