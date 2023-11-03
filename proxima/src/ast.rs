use crate::{
    location::{HasLocation, Location},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Literal {
    value: Value,
    location: Location,
}

impl Literal {
    #[inline]
    #[must_use]
    pub const fn new(value: Value, location: Location) -> Self {
        Self { value, location }
    }

    #[inline]
    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}

impl HasLocation for Literal {
    #[inline]
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct ArrayExpression {
    elements: Vec<Expression>,
    location: Location,
}

impl ArrayExpression {
    #[inline]
    #[must_use]
    pub const fn new(elements: Vec<Expression>, location: Location) -> Self {
        Self { elements, location }
    }

    #[inline]
    #[must_use]
    pub fn elements(&self) -> &[Expression] {
        &self.elements
    }
}

impl HasLocation for ArrayExpression {
    #[inline]
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl HasLocation for BinaryExpression {
    fn location(&self) -> Location {
        Location::new(self.left.location().start(), self.right.location().end())
    }
}

#[derive(Debug, Clone)]
pub struct BreakExpression(Location);

impl BreakExpression {
    #[inline]
    #[must_use]
    pub const fn new(location: Location) -> Self {
        Self(location)
    }
}

impl HasLocation for BreakExpression {
    fn location(&self) -> Location {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct WhileExpression {
    condition: Box<Expression>,
    body: Box<Expression>,
    location: Location,
}

impl WhileExpression {
    #[inline]
    #[must_use]
    pub const fn new(
        condition: Box<Expression>,
        body: Box<Expression>,
        location: Location,
    ) -> Self {
        Self {
            condition,
            body,
            location,
        }
    }

    #[inline]
    #[must_use]
    pub const fn condition(&self) -> &Expression {
        &self.condition
    }

    #[inline]
    #[must_use]
    pub const fn body(&self) -> &Expression {
        &self.body
    }
}

impl HasLocation for WhileExpression {
    #[inline]
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Binary(BinaryExpression),
    Break(BreakExpression),
    Block(StatementsBlock),
    While(WhileExpression),
}

impl HasLocation for Expression {
    #[inline]
    fn location(&self) -> Location {
        match self {
            Self::Literal(literal) => literal.location(),
            Self::Binary(binary) => binary.location(),
            Self::Break(break_) => break_.location(),
            Self::Block(block) => block.location(),
            Self::While(while_) => while_.location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    expression: Expression,
    location: Location,
}

impl ExpressionStatement {
    #[inline]
    #[must_use]
    pub const fn new(expression: Expression, location: Location) -> Self {
        Self {
            expression,
            location,
        }
    }

    #[inline]
    #[must_use]
    pub const fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl HasLocation for ExpressionStatement {
    #[inline]
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    expression: Expression,
    location: Location,
}

impl ReturnStatement {
    #[inline]
    #[must_use]
    pub const fn new(expression: Expression, location: Location) -> Self {
        Self {
            expression,
            location,
        }
    }

    #[inline]
    #[must_use]
    pub const fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl HasLocation for ReturnStatement {
    #[inline]
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Return(ReturnStatement),
}

impl HasLocation for Statement {
    fn location(&self) -> Location {
        match self {
            Self::Expression(expression) => expression.location(),
            Self::Return(return_) => return_.location(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatementsBlock {
    statements: Vec<Statement>,
    location: Location,
}

impl StatementsBlock {
    #[inline]
    #[must_use]
    pub const fn new(statements: Vec<Statement>, location: Location) -> Self {
        Self {
            statements,
            location,
        }
    }

    #[inline]
    #[must_use]
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }
}

impl HasLocation for StatementsBlock {
    #[inline]
    fn location(&self) -> Location {
        self.location
    }
}
