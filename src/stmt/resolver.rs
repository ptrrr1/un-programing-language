use super::Stmt;
use crate::{
    expr::{Expr, eval::ExprVisitor},
    stmt::eval::StmtVisitor,
    tokens::{Token, TokenType},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Scope = HashMap<String, bool>;

#[derive(Default)]
pub struct Resolver {
    locals: HashMap<String, usize>,
    scopes: Vec<Rc<RefCell<Scope>>>,
    cur_fun: Option<String>, // TODO: He uses this to evaluate out of place returns, I don't need it because of signals, but it could be useful later
}

impl Resolver {
    fn declare(&mut self, identifier: &str) {
        if let Some(scope) = self.scopes.last() {
            if scope.borrow().contains_key(identifier) {
                panic!("A variable with this identifier already exists in this scope"); // TODO: Return an Err
            }
            scope.borrow_mut().insert(identifier.to_string(), false);
        }
    }

    fn define(&mut self, identifier: &str) {
        if let Some(scope) = self.scopes.last() {
            scope.borrow_mut().insert(identifier.to_string(), true);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        let scope = match self.scopes.last() {
            Some(s) => s.clone(),
            None => Rc::new(RefCell::new(HashMap::new())), // NOTE: IDK don't think this is needed
        };
        expr.accept(scope, self)
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        let scope = match self.scopes.last() {
            Some(s) => s.clone(),
            None => Rc::new(RefCell::new(HashMap::new())),
        };
        stmt.accept(scope, self)
    }

    pub fn resolve_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_local(&mut self, identifier: &str) {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.borrow().contains_key(identifier) {
                self.locals.insert(identifier.to_string(), depth);
                break;
            }
        }
    }

    fn resolve_function(&mut self, params: &[Token], body: &[Stmt], func: Option<String>) {
        let f = self.cur_fun.clone();
        self.cur_fun = func;

        self.begin_scope();

        for param in params {
            if let TokenType::Identifier(s) = &param.token_type {
                self.declare(s);
                self.define(s);
            }
        }

        self.resolve_stmts(body);
        self.end_scope();

        self.cur_fun = f;
    }

    fn resolve_lambda(&mut self, params: &[Token], body: &Expr) {
        self.begin_scope();

        for param in params {
            if let TokenType::Identifier(s) = &param.token_type {
                self.declare(s);
                self.define(s);
            }
        }

        self.resolve_expr(body);
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Rc::new(RefCell::new(Scope::new())));
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn into_locals(self) -> HashMap<String, usize> {
        self.locals
    }
}

impl StmtVisitor<(), HashMap<String, bool>> for Resolver {
    fn visit_expr(&mut self, _env: Rc<RefCell<Scope>>, expr: &Expr) {
        self.resolve_expr(expr);
    }

    fn visit_print(&mut self, _env: Rc<RefCell<Scope>>, expr: &Expr) {
        self.resolve_expr(expr);
    }

    fn visit_var(&mut self, _env: Rc<RefCell<Scope>>, target: &Token, expr: &Expr) {
        if let TokenType::Identifier(s) = &target.token_type {
            self.declare(s);
            self.resolve_expr(expr); // NOTE: He checks if it isn't null, but I don't need that because I don't allow it
            self.define(s);
        }
    }

    fn visit_block(&mut self, _env: Rc<RefCell<Scope>>, stmts: &[Stmt]) {
        self.begin_scope();
        self.resolve_stmts(stmts);
        self.end_scope();
    }

    fn visit_conditional(
        &mut self,
        _env: Rc<RefCell<Scope>>,
        condition: &Expr,
        true_branch: &[Stmt],
        false_branch: &Option<Vec<Stmt>>,
    ) {
        self.resolve_expr(condition);
        self.resolve_stmts(true_branch);
        if let Some(f) = false_branch {
            self.resolve_stmts(f);
        }
    }

    fn visit_while(&mut self, _env: Rc<RefCell<Scope>>, condition: &Expr, stmts: &[Stmt]) {
        self.resolve_expr(condition);
        self.resolve_stmts(stmts);
    }

    fn visit_function(
        &mut self,
        _env: Rc<RefCell<Scope>>,
        identifier: &Token,
        params: &[Token],
        body: &[Stmt],
    ) {
        if let TokenType::Identifier(s) = &identifier.token_type {
            self.declare(s);
            self.define(s);
            self.resolve_function(params, body, Some(s.to_string()));
        }
    }

    fn visit_return(&mut self, _env: Rc<RefCell<Scope>>, expr: &Expr) {
        if self.cur_fun.is_none() {
            panic!("Return outside of function"); // TODO: Again, not cool to panic, but this will be rewritten from scratch
        }
        self.resolve_expr(expr);
    }

    fn visit_break(&mut self) {} // NOTE: Nothign to resolve // TODO: There's actually something to resolve... If it's outside of a loop
}

impl ExprVisitor<(), Scope> for Resolver {
    fn visit_assignment(&mut self, _env: Rc<RefCell<Scope>>, target: &Expr, expr: &Expr) {
        self.resolve_expr(expr);

        // TODO: This feels like bullshit, maybe i don't know how to use rust
        if let Expr::Variable(t) = target
            && let TokenType::Identifier(s) = &t.token_type
        {
            self.resolve_local(s);
        }
    }

    fn visit_binary(
        &mut self,
        _env: Rc<RefCell<Scope>>,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) {
        self.resolve_expr(left);
        self.resolve_expr(right);
    }

    fn visit_unary(&mut self, _env: Rc<RefCell<Scope>>, _operatorr: &Token, right: &Expr) {
        self.resolve_expr(right);
    }

    fn visit_grouping(&mut self, _env: Rc<RefCell<Scope>>, inner: &Expr) {
        self.resolve_expr(inner);
    }

    fn visit_literal(&mut self, _env: Rc<RefCell<Scope>>, _inner: &Token) {}

    fn visit_variable(&mut self, _env: Rc<RefCell<Scope>>, inner: &Token) {
        if let TokenType::Identifier(s) = &inner.token_type {
            if let Some(scope) = self.scopes.last()
                && scope.borrow().get(s).is_some_and(|v| v == &false)
            {
                panic!("Can't read local variable before it's own initializer."); // TODO: Not panic?
            }
            self.resolve_local(s);
        }
    }

    fn visit_exposed_fn(&mut self, _env: Rc<RefCell<Scope>>, _inner: &Token) {
        todo!() // TODO: Figure this out?
    }

    fn visit_conditional(
        &mut self,
        _env: Rc<RefCell<Scope>>,
        condition: &Expr,
        true_branch: &Expr,
        false_branch: &Expr,
    ) {
        self.resolve_expr(condition);
        self.resolve_expr(true_branch);
        self.resolve_expr(false_branch);
    }

    fn visit_call(
        &mut self,
        _env: Rc<RefCell<Scope>>,
        callee: &Expr,
        _paren: &Token,
        args: &[Expr],
    ) {
        self.resolve_expr(callee);
        for arg in args {
            self.resolve_expr(arg);
        }
    }

    fn visit_lambda(&mut self, _env: Rc<RefCell<Scope>>, params: &[Token], body: &Expr) {
        self.resolve_lambda(params, body);
    }
}
