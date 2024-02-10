use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
/// Represents the operators in the parse tree.
pub enum Operator {
    /// Represents the die operator.
    Die,
    /// Represents the plus operator.
    Plus,
    /// Represents the minus operator.
    Minus,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Operator::Die => write!(f, "d"),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents the expressions in the parse tree.
pub enum Expr {
    /// Represents the binary expression in the parse tree.
    Binary(Box<Expr>, Box<Expr>, Operator),
    /// Represents the right-associative unary expression in the parse tree.
    Unary(Box<Expr>, Operator),
    /// Represents the numeric literal in the parse tree.
    NumericLiteral(usize),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Expr::NumericLiteral(n) => write!(f, "{}", n),
            Expr::Unary(rhs, op) => {
                write!(f, "({}", op)?;
                write!(f, " {}", rhs)?;
                write!(f, ")")
            }
            Expr::Binary(lhs, rhs, op) => {
                write!(f, "({}", op)?;
                write!(f, " {} {}", lhs, rhs)?;
                write!(f, ")")
            }
        }
    }
}

/// Helper function to create numeric literal expression.
pub fn numeric_literal(n: usize) -> Expr {
    Expr::NumericLiteral(n)
}

/// Helper function to create unary expression.
pub fn unary_expr(rhs: Expr, op: Operator) -> Expr {
    Expr::Unary(Box::new(rhs), op)
}

/// Helper function to create binary expression.
pub fn binary_expr(lhs: Expr, rhs: Expr, op: Operator) -> Expr {
    Expr::Binary(Box::new(lhs), Box::new(rhs), op)
}

/// Helper function to create binary roll expression.
pub fn binary_roll_expr(lhs: usize, rhs: usize) -> Expr {
    binary_expr(numeric_literal(lhs), numeric_literal(rhs), Operator::Die)
}

/// Helper function to create unary roll expression.
pub fn unary_roll_expr(rhs: usize) -> Expr {
    unary_expr(numeric_literal(rhs), Operator::Die)
}
