// extern crate proc_macro;
// use proc_macro::TokenStream;

// #[proc_macro]
// pub fn make_answer(_item: TokenStream) -> TokenStream {
//     "fn answer() -> u32 { 42 }".parse().unwrap()
// }

// impl Add<Self> for Symbolic {
//     type Output = Node;
//     fn add(self, rhs: Self) -> Self::Output {
//         use Node::*;
//         BinaryExpr { op: '+', lhs: Box::new(SymbExpr(self)), rhs: Box::new(SymbExpr(rhs)) }
//     }
// }

// #[proc_macro]
// pub fn con() {
    
// }

// #[macro_export]
// macro_rules! symbolic{
//     (fn $i:ident $t:tt -> $e:expr) => {
//         {
//             let val: usize = $e; // Force types to be integers
//             println!("{} = {}", stringify!{$e}, val);
//             println!("{}", stringify!{$t});
//         }
//     };
// }