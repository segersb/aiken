use chumsky::prelude::*;

use crate::{
    ast::TraceKind,
    expr::UntypedExpr,
    parser::{error::ParseError, token::Token},
};

pub fn parser() -> impl Parser<Token, UntypedExpr, Error = ParseError> {
    recursive(|expression| {
        choice((
            just(Token::Trace)
                .ignore_then(super::parser(expression.clone()))
                .then(expression.clone())
                .map_with_span(|(text, then_), span| UntypedExpr::Trace {
                    kind: TraceKind::Trace,
                    location: span,
                    then: Box::new(then_),
                    text: Box::new(super::string::flexible(text)),
                }),
            just(Token::ErrorTerm)
                .ignore_then(super::parser(expression.clone()).or_not())
                .map_with_span(|reason, span| {
                    UntypedExpr::error(span, reason.map(super::string::flexible))
                }),
            just(Token::Todo)
                .ignore_then(super::parser(expression.clone()).or_not())
                .map_with_span(|reason, span| {
                    UntypedExpr::todo(span, reason.map(super::string::flexible))
                }),
            super::parser(expression.clone())
                .then(expression.repeated())
                .foldl(|current, next| current.append_in_sequence(next)),
        ))
    })
}