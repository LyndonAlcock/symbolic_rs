extern crate proc_macro;
use proc_macro::TokenStream;
pub enum Symbolic<'a>{
    Identifier(&'a str),
    Function(&'a str),
    Separator(char),
    Rational(isize, usize),
    Operator(char)
}

pub enum Node<'a>{
    Symbolic(Symbolic<'a>),
    Expression{
        op: Symbolic<'a>,
        buf: Vec<&'a Self>
    }
}


macro_rules! symbolic{
    (fn $($expression:tt)*) => {
        symbolic!(@munch ( $($expression)* ) -> [])
    };
    ( @munch ($num:literal $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Rational($num,1), ])
    };
    ( @munch ($param:ident $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Identifier(stringify!($param)), ])
    };
    ( @munch ($func:ident ($($expression:tt)*) $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ 
            $($accum)* 
            Function(stringify!(func)),
            symbolic!(@munch ( $($expression)* ) -> []),
            Seperator(")"),
        ])
    };
    ( @munch (($($expression:tt)*) $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ 
            $($accum)* 
            Symbolic::Operator('('),
            symbolic!(@munch ( $($expression)* ) -> []),
            Symbolic::Separator(')'),
        ])
    };
    ( @munch (+ $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Operator('+'), ])
    };
    ( @munch (- $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Operator('-'), ])
    };
    ( @munch (/ $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Operator('/'), ])
    };
    ( @munch (* $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Operator('*'), ])
    };
    ( @munch (^ $($rest:tt)*) -> [$($accum:tt)*] ) => {
        symbolic!(@munch ( $($rest)* ) -> [ $($accum)* Symbolic::Operator('^'), ])
    };
    ( @munch () -> [$($accum:tt)*] ) => {
        vec![ $($accum)* ]
    };
}

fn main() {
    let func = symbolic!{fn a * sin(a * x) + x^2};
}