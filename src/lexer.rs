#![allow(clippy::redundant_closure_call)]

use super::ast::*;
use peg::{error::ParseError, str::LineCol};


peg::parser! {
    grammar pfx() for str {
        use std::sync::Arc;
        
        
        rule kw_true()      = ("true")
        rule kw_false()     = ("false")
        rule kw_null()      = ("NULL" / "null")
        rule kw_and()       = ("And")
        rule kw_or()        = ("Or")
        rule kw_not()       = ("Not")
        rule kw_as()          = ("AS" / "as")
        
        rule kw_parent()      = ("Parent")
        rule kw_self()        = ("Self")
        rule kw_this_item()   = ("ThisItem")
        rule kw_this_record() = ("ThisRecord")
        
        rule kw_in()          = ("in")
        rule kw_exact_in()    = ("exactin")
        

        rule _()
            = [' ']

        rule __()
            = [' ' | '\n' | '\t']

        rule alpha()
            = ['a'..='z' | 'A'..='Z']

        rule num()
            = ['0'..='9']

        rule alpha_num()
            = ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']


        // e.g. '42', '-1'
        rule integer() -> i64
            = integer:$("-"?num()+) {? integer.parse().or(Err("invalid integer")) }

        // e.g. '-0.53', '34346.245', '236.0'
        rule real() -> f64
            = real:$("-"? num()+ "." num()+) {? real.parse().or(Err("invalid real"))}

        // e.g. 'TRUE', 'FALSE'
        rule boolean() -> bool
            = kw_true() { true } / kw_false() { false }

        // e.g. 'hello world'
        rule text() -> Arc<str>
            = "'" text:$([^ '\'' | '\n' | '\r']*) "'" { Arc::from(text) }


        rule record() -> Record
            = "{" __* fields:(key:ident() _* ":" _* value:expression() { (key, value) }) ** (_* "," _*) __* "}" { Record::from(fields) }
        
        // e.g. 'TRUE', '42', 'hello world'
        rule literal() -> Literal
            = i:integer() { Literal::Number(i as f64) }
            / r:real() { Literal::Number(r) }
            / b:boolean() { Literal::Boolean(b) }
            / t:text() { Literal::Text(t) }
            / r:record() { Literal::Record(r) }

            
        rule projection_expression() -> Expression
            = z:expression() _* kw_as() _* a:ident() { UnaryExpression::alias(z, a) }
            / expression()

        
            #[cache_left_rec]
        pub rule expression() -> Expression
            = precedence!{
                a:(@) __+ kw_and() __+ b:@ { BinaryExpression::and(a, b) }
                a:(@) __+ kw_or() __+ b:@ { BinaryExpression::or(a, b) }
                a:(@) __* "&&" __* b:@ { BinaryExpression::and(a, b) }
                a:(@) __* "||" __* b:@ { BinaryExpression::or(a, b) }
                --
                kw_not() _+ c:(@) { UnaryExpression::not(c) }
                "!" _* c:(@) { UnaryExpression::not(c) }
                --
                a:(@) __* "="  __* b:@ { BinaryExpression::eq(a, b) }
                a:(@) __* "<>" __* b:@ { BinaryExpression::ne(a, b) }
                a:(@) __* "<"  __* b:@ { BinaryExpression::lt(a, b) }
                a:(@) __* "<=" __* b:@ { BinaryExpression::le(a, b) }
                a:(@) __* ">"  __* b:@ { BinaryExpression::gt(a, b) }
                a:(@) __* ">=" __* b:@ { BinaryExpression::ge(a, b) }
                a:(@) __* kw_in() __* b:@ { BinaryExpression::in_(a, b, false) }
                a:(@) __* kw_exact_in() __* b:@ { BinaryExpression::in_(a, b, true) }
                --
                a:(@) __* "+" __* b:@ { BinaryExpression::add(a, b) }
                a:(@) __* "-" __* b:@ { BinaryExpression::subtract(a, b) }
                --
                a:(@) __* "*" __* b:@ { BinaryExpression::multiply(a, b) }
                a:(@) __* "/" __* b:@ { BinaryExpression::divide(a, b) }
                --
                a:(@) __* "%" __* b:@ { BinaryExpression::modulo(a, b) }
                a:(@) __* "^" __* b:@ { BinaryExpression::exponent(a, b) }
                --
                
                l:literal() { UnaryExpression::literal(l) }
                p:property() { UnaryExpression::property(p.0, p.1) }
                func:ident() _* "(" __* params:expression() ** (_* "," _*) __* ")" { FunctionExpression::function(func, params ) }
                
                i:ident() { UnaryExpression::ident(i) }                
                --
                
                "(" __* c:expression() __* ")" { c }
            }

        rule ident() -> Arc<str>
            = ident:$(alpha()alpha_num()*) { Arc::from(ident) }

        rule context() -> Context
            = kw_parent() { Context::Parent }
            / kw_self() { Context::Self_ }
            / kw_this_item() { Context::ThisItem }
            / kw_this_record() { Context::ThisRecord }


        rule property() -> (Context, Arc<str>)
            = ctx:context() "." key:ident() { (ctx, key) }


        pub rule expressions() -> Vec<Expression>
            = e:expression() ** (__* ";" __*) __* ";"? { e }

    }
}

pub fn parse(input: &str) -> Result<Vec<Expression>, ParseError<LineCol>> {
    pfx::expressions(input)
}

