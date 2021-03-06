use nom::IResult;

use crate::{instructions::Expression, Decode};

use super::{
    indices::{FuncIdx, TableIdx},
    types::RefType,
};

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum Element {
    ActiveIndex(Expression, Vec<FuncIdx>),
    PassiveIndex(ElementKind, Vec<FuncIdx>),
    ActiveExplicitIndex(TableIdx, Expression, ElementKind, Vec<FuncIdx>),
    DeclarativeIndex(ElementKind, Vec<FuncIdx>),
    ActiveExpression(Expression, Vec<Expression>),
    PassiveExpression(RefType, Vec<Expression>),
    ActiveExplicitExpression(TableIdx, Expression, RefType, Vec<Expression>),
    DeclarativeExpression(RefType, Vec<Expression>),
}

impl Decode for Element {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

/// TODO: Document
#[derive(Debug, PartialEq)]
pub enum ElementKind {
    FuncRef,
}

impl Decode for ElementKind {
    fn decode(_input: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Write tests
}
