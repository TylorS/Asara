use crate::lexing::token::{span, Span, Token};
use crate::parsing::ast::{Declaration, Expression, Program, Spanned, Statement};
use chumsky::prelude::*;
use logos::Logos;

pub fn parse<'a>(source: &str) -> Result<Program, Vec<Simple<Token<'a>>>> {
    let mut lexer = Token::lexer(source).spanned();
    let span = span(0, source.len());
    let parser = program(span);

    parser.parse(lexer)
}

fn program<'a>(span: Span) -> impl Parser<Token<'a>, Program<'a>, Error = Simple<Token<'a>>> {
    statement().repeated().map(move |statements| Program {
        statements,
        span: span.clone(),
    })
}

fn statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    expression_statement()
        .or(return_statement())
        .or(break_statement())
        .or(continue_statement())
        .or(block_statement())
        .or(if_statement())
        .or(while_statement())
        .or(for_statement())
        .or(for_of_statement())
        .or(for_in_statement())
        .or(import_declaration())
        .or(declaration_statement())
}

fn expression_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    expression().map(|expression| {
        let span = expression.span();
        Statement::Expression {
            expr: Box::new(expression),
            span,
        }
    })
}

fn expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    string_expression()
        .or(integer_expression())
        .or(decimal_expression())
        .or(big_integer_expression())
        .or(big_decimal_expression())
        .or(boolean_expression())
        .or(array_expression())
        .or(tuple_expression())
        .or(record_expression())
        .or(symbol_expression())
        .or(operator_expression())
        .or(function_call_expression())
        .or(resume_expression())
        .or(yield_expression())
        .or(perform_expression())
        .or(handle_expression())
}

fn string_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn integer_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn decimal_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn big_integer_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>>
{
    todo!("TODO: IMPLEMENT")
}

fn big_decimal_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>>
{
    todo!("TODO: IMPLEMENT")
}

fn boolean_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn array_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn tuple_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn record_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn symbol_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn operator_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    unary_operator_expression().or(binary_operator_expression())
}

fn unary_operator_expression<'a>(
) -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn binary_operator_expression<'a>(
) -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn function_call_expression<'a>(
) -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn resume_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn yield_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn perform_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn handle_expression<'a>() -> impl Parser<Token<'a>, Expression<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn return_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn break_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn continue_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn block_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn if_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn while_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn for_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn for_of_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn for_in_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn import_declaration<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn declaration_statement<'a>() -> impl Parser<Token<'a>, Statement<'a>, Error = Simple<Token<'a>>> {
    declaration().map(|declaration| Statement::Declaration(declaration))
}

fn declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>> {
    function_declaration()
        .or(brand_declaration())
        .or(type_alias_declaration())
        .or(data_declaration())
        .or(effect_declaration())
        .or(let_declaration())
}

fn function_declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>>
{
    todo!("TODO: IMPLEMENT")
}

fn brand_declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn type_alias_declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>>
{
    todo!("TODO: IMPLEMENT")
}

fn data_declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn effect_declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}

fn let_declaration<'a>() -> impl Parser<Token<'a>, Declaration<'a>, Error = Simple<Token<'a>>> {
    todo!("TODO: IMPLEMENT")
}
