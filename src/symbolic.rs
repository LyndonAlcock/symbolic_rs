use logos::{Logos,Lexer};


#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a>{
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[regex("[a-zA-Zα-ωΑ-Ω]+")]
    Identifier(&'a str),

    #[regex("(sin|cos|tan|asin|acos|atan|sinh|cosh|tanh|)[(]" )]
    Function(&'a str),

    #[regex(r"(-|)[0-9]+", Self.integer)]
    #[regex(r"(-|)[0-9]+/[0-9]", Self.fraction)]
    #[regex(r"(-|)[0-9]+\\.[0-9]", Self.decimal)]
    Rational((isize, usize)),
    
    #[regex(r"[+\-\*/]+")]
    Operator(&'a str)
}

pub enum Node<'b>{
    Symbolic(Token<'b>),
    Expression{
        op: Token<'b>,
        buf: Vec<&'b Self>
    }
}

// impl Token<'_>
// {
//     fn integer(lex: &mut Lexer<Self>) -> Option<(i8,u8)> {
//         let slice = lex.slice();
//         let numerator: i8 = slice.parse().ok()?; // skip 'k'
//         Some((numerator,1))
//     }

//     fn fraction(lex: &mut Lexer<Self>) -> Option<(i8,u8)> {
//         let mut slice = lex.slice().split('/');
//         let numer: u8 = slice.next()?.parse().ok()?;
//         let denom: u8 = slice.next()?.parse().ok()?;
//         Some(simplify(numer,denom))
//     }

//     fn decimal(lex: &mut Lexer<Token>) -> Option<(i8,u8)> {
//         let mut slice = lex.slice().split('.');
//         let str_int = slice.next()?;
//         let str_dec = slice.next()?;
//         let exp = str_dec.len() as u32;
//         let numer = format!("{}{}",str_int, str_dec)
//             .parse().ok()?;
//         let denom = 10_u8.pow(exp);
//         Some(simplify(numer,denom))
//     }

//     pub fn simplify(mut numer: u8, mut denom: u8)-> (i8, u8){
//         for n in (1..denom).rev(){
//             if ((numer % n) == 0) & ((denom % n) == 0) {       
//                 numer = numer/n;
//                 denom = denom/n;
//             }
//         }
//         (numer as i8, denom)
//     }
// }