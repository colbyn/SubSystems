//! FunctionDecl definitions.
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::convert::AsRef;
use std::collections::{HashMap, LinkedList, HashSet, VecDeque};
use num::{FromPrimitive, ToPrimitive, BigRational, BigInt};

use crate::ast::expr::{Expr, FunCall};
use crate::*;

///////////////////////////////////////////////////////////////////////////////
// CONVERSION HELPERS
///////////////////////////////////////////////////////////////////////////////

pub trait ConvertTo<T> {
    fn convert_to(&self) -> Option<T>;
}

impl ConvertTo<BigRational> for Expr {
    fn convert_to(&self) -> Option<BigRational> {
        match self {
            Expr::Num(x) => Some(x.clone()),
            _ => None,
        }
    }
}
impl ConvertTo<BigInt> for Expr {
    fn convert_to(&self) -> Option<BigInt> {
        match self {
            Expr::Num(x) => Some(x.to_integer()),
            _ => None,
        }
    }
}
impl ConvertTo<Expr> for Expr {
    fn convert_to(&self) -> Option<Expr> {
        Some(self.clone())
    }
}



///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Body(Rc<dyn Fn(FunCall) -> Option<Expr>>);

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Body").finish()
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Num
}

// #[derive(Debug, Clone)]
// pub enum Arg {
//     Keyword(String, )
// }

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub path: Vec<String>,
    pub pos_args: usize,
    pub key_args: Vec<String>,
    pub body: Body,
}

impl FunctionDecl {
    pub fn call(&self, source: Expr) -> Result<Expr, Expr> {
        let source_ref = source.clone();
        let root_fun_call = return_fun_call!(Err(source), source.clone());
        match (&self.path.clone()[..], self.pos_args.clone()) {
            ([name1], _) => {
                let fun_call = *root_fun_call.clone();
                let valid_name = &fun_call.name == &self.path[0];
                let valid_pos_args = fun_call.pos_args.len() == self.pos_args;
                let valid_key_args = self.key_args
                    .iter()
                    .all(|key| {
                        fun_call.key_args.contains_key(key)
                    });
                if valid_name && valid_pos_args && valid_key_args {
                    match (self.body.0)(fun_call.clone()) {
                        Some(x) => Ok(x),
                        None => Err(source),
                    }
                } else {
                    Err(source)
                }
            }
            ([name1, name2], _) => {
                let fun_call = return_fun_call_arg0!(Err(source), source.clone());
                let fun_call = *return_fun_call!(Err(source), fun_call);
                let valid_name = {
                    &root_fun_call.name == name1 &&
                    &fun_call.name == name2
                };
                let valid_pos_args = fun_call.pos_args.len() == self.pos_args;
                let valid_key_args = self.key_args
                    .iter()
                    .all(|key| {
                        fun_call.key_args.contains_key(key)
                    });
                if valid_name && valid_pos_args && valid_key_args {
                    match (self.body.0)(fun_call.clone()) {
                        Some(x) => {
                            Ok(x)
                        }
                        None => Err(source),
                    }
                } else {
                    Err(source)
                }
            }
            _ => Err(source)
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// MACRO DSL
///////////////////////////////////////////////////////////////////////////////

/// Internal helper.
#[macro_export]
macro_rules! init_arg_header {
    ($pos_counter:expr; $keyword_state:expr; argument $name:ident : $type:ty) => {
        $pos_counter = $pos_counter + 1;
    };
    ($pos_counter:expr; $keyword_state:expr; keyword $name:ident : $type:ty) => {
        $keyword_state.push(String::from(stringify!($name)));
    };

    ($pos_counter:expr; $keyword_state:expr; argument $name:ident : $type:ty, $($rest:tt)*) => {
        $pos_counter = $pos_counter + 1;
        init_arg_header!($pos_counter; $keyword_state; $($rest)*)
    };
    ($pos_counter:expr; $keyword_state:expr; keyword $name:ident : $type:ty, $($rest:tt)*) => {
        $keyword_state.push(String::from(stringify!($name)));
        init_arg_header!($pos_counter; $keyword_state; $($rest)*)
    };
}

#[macro_export]
macro_rules! init_arg_scope {
    ($pos_args:expr; $key_args:expr; argument $name:ident : $type:ty) => {
        let $name: $type = $pos_args
            .pop_front()
            .and_then(|expr: Expr| -> Option<$type> {
                expr.convert_to()
            })?;
    };
    ($pos_args:expr; $key_args:expr; keyword $name:ident : $type:ty) => {
        let $name: $type = $key_args
            .get(stringify!($name))
            .and_then(|expr: &Expr| -> Option<$type> {
                expr.convert_to()
            })?;
    };

    ($pos_args:expr; $key_args:expr; argument $name:ident : $type:ty, $($rest:tt)*) => {
        let $name: $type = $pos_args
            .pop_front()
            .and_then(|expr: Expr| -> Option<$type> {
                expr.convert_to()
            })?;
        init_arg_scope!($pos_args; $key_args; $($rest)*)
    };
    ($pos_args:expr; $key_args:expr; keyword $name:ident : $type:ty, $($rest:tt)*) => {
        let $name: $type = $key_args
            .get(stringify!($name))
            .and_then(|expr: &Expr| -> Option<$type> {
                expr.convert_to()
            })?;
        init_arg_scope!($pos_args; $key_args; $($rest)*)
    };
}

#[macro_export]
macro_rules! defintion {
    ($name:ident( $($arg:tt)* ) => $body:expr) => {{
        let name = String::from(stringify!($name));
        let mut pos_counter = 0;
        let mut keyword_state = Vec::<String>::new();
        init_arg_header!(pos_counter; keyword_state; $($arg)*);
        let function_decl = FunctionDecl {
            path: vec![name],
            pos_args: pos_counter,
            key_args: keyword_state,
            body: Body(Rc::new(
                |call: FunCall| -> Option<Expr> {
                    let mut pos_args: LinkedList<Expr> = LinkedList::from_iter(call.pos_args.clone());
                    let mut key_args: HashMap<String, Expr> = call.key_args.clone();
                    init_arg_scope!(pos_args; key_args; $($arg)*);
                    $body
                }
            )),
        };
        function_decl
    }};
    ($name:ident( $($arg:tt)* ) => $body:expr) => {{
        let name = String::from(stringify!($name));
        let mut pos_counter = 0;
        let mut keyword_state = Vec::<String>::new();
        init_arg_header!(pos_counter; keyword_state; $($arg)*);
        let function_decl = FunctionDecl {
            path: vec![name],
            pos_args: pos_counter,
            key_args: keyword_state,
            body: Body(Rc::new(
                |call: FunCall| -> Option<Expr> {
                    let mut pos_args: LinkedList<Expr> = LinkedList::from_iter(call.pos_args.clone());
                    let mut key_args: HashMap<String, Expr> = call.key_args.clone();
                    init_arg_scope!(pos_args; key_args; $($arg)*);
                    $body
                }
            )),
        };
        function_decl
    }};
    ($name1:ident => $name2:ident($($arg:tt)*) => $body:expr) => {{
        let name1 = String::from(stringify!($name1));
        let name2 = String::from(stringify!($name2));
        let mut pos_counter = 0;
        let mut keyword_state = Vec::<String>::new();
        init_arg_header!(pos_counter; keyword_state; $($arg)*);
        let function_decl = FunctionDecl {
            path: vec![name1, name2],
            pos_args: pos_counter,
            key_args: keyword_state,
            body: Body(Rc::new(
                |call: FunCall| -> Option<Expr> {
                    let mut pos_args: LinkedList<Expr> = LinkedList::from_iter(call.pos_args.clone());
                    let mut key_args: HashMap<String, Expr> = call.key_args.clone();
                    init_arg_scope!(pos_args; key_args; $($arg)*);
                    $body
                }
            )),
        };
        function_decl
    }};
}

///////////////////////////////////////////////////////////////////////////////
// FUNCTION DEFINITIONS
///////////////////////////////////////////////////////////////////////////////

fn all_functions() -> Vec<FunctionDecl> {
    let mut definitions = Vec::new();
    definitions.push(defintion!(
        mole(argument value:Expr) => {
            Some(Expr::Product(vec![
                value,
                Expr::avogadro_number(),
            ]))
        }
    ));
    definitions.push(defintion!(
        GHz(argument value:BigRational) => {
            Some(Expr::Product(vec![
                Expr::Num(value),
                Expr::gigahertz()
            ]))
        }
    ));
    definitions.push(defintion!(
        MHz(argument value:BigRational) => {
            Some(Expr::Product(vec![
                Expr::Num(value),
                Expr::megahertz()
            ]))
        }
    ));
    definitions.push(defintion!(
        nm(argument value:BigRational) => {
            Some(Expr::Product(vec![
                Expr::Num(value),
                Expr::con("nm")
            ]))
        }
    ));
    // NOTE:
    // - Formula: `E = h * v` where `h` is planck's constant, and `v` is the photon's frequency.
    // - Speed of light: `c = ??v` where `??` is the photon's wavelength.
    // - Frequency: `v = c/??`
    // - Energy (alt): `E = h * v = (hc)/??`
    // As h and c are both constants, photon energy E changes in inverse relation to wavelength ??.
    definitions.push(defintion!(
        energy => photon(keyword wavelength : Expr) => {{
            let numerator = Expr::Product(vec![
                Expr::speed_of_light(),
                Expr::planck_constant(),
            ]);
            let denominator = wavelength;
            Some(Expr::ratio(
                numerator,
                denominator,
            ))
        }}
    ));
    // Formula: `E = h * v` where `h` is planck's constant, and `v` is frequency (in hertz).
    // NOTE: Since `1??? = 1/s`, the seconds (s) cancels out.
    definitions.push(defintion!(
        energy => photon(keyword frequency : Expr) => {{
            Some(Expr::Product(vec![
                Expr::planck_constant(),
                frequency
            ]))
        }}
    ));
    // NOTE:
    // - Formula: `v = c/??` where
    //  * `??` is the photon's wavelength
    //  * `v` is the photon's frequency.
    // - Speed of light: `c = ??v` where `??` is the photon's wavelength.
    // - Energy (alt): `E = h * v = (hc)/??`
    definitions.push(defintion!(
        frequency(keyword wavelength : Expr) => {{
            let numerator = Expr::speed_of_light();
            let denominator = wavelength;
            Some(Expr::ratio(
                numerator,
                denominator,
            ))
        }}
    ));
    // NOTE:
    // - Formula: `?? = c / ??` where
    //  * `??` is the photon's wavelength
    //  * `v` is the photon's frequency.
    // - Speed of light: `c = ??v` where `??` is the photon's wavelength.
    // - Energy (alt): `E = h * v = (hc)/??`
    definitions.push(defintion!(
        wavelength(keyword frequency : Expr) => {{
            let numerator = Expr::speed_of_light();
            let denominator = frequency;
            Some(Expr::ratio(
                numerator,
                denominator,
            ))
        }}
    ));
    definitions.push(defintion!(
        period(keyword frequency : Expr) => {{
            Some(Expr::unit_fraction(frequency))
        }}
    ));
    // // NOTE:
    // // - Formula (Rydberg formula): `1/?? = R_h * (1/n_2 - 1/n_1))` where
    // // * `R_h` is Rydberg constant
    // definitions.push(defintion!(
    //     energy(keyword from : Expr, keyword to : Expr) => {{
    //         let f = defintion!(
    //             electron(keyword n : BigInt) => {{
    //                 Some(Expr::Num(BigRational::new(
    //                     BigInt::from_i64(1).unwrap(),
    //                     num::pow(n, 2),
    //                 )))
    //             }}
    //         );
    //         let from: BigRational = f.call(from)
    //             .ok()?
    //             .convert_to()?;
    //         let to: BigRational = f.call(to)
    //             .ok()?
    //             .convert_to()?;
    //         Some(Expr::Product(vec![
    //             Expr::rydberg_constant(),
    //             Expr::Num(from - to),
    //         ]))
    //     }}
    // ));
    definitions
}

pub fn apply(expr: Expr) -> Expr {
    all_functions()
        .into_iter()
        .fold(expr, |expr, f| {
            let result = match f.call(expr) {
                Ok(x) => {
                    x
                }
                Err(x) => {
                    x
                }
            };
            result
        })
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

pub fn main() {
    let expr = Expr::from_str("nm(250)").unwrap();
    let nm_decl = defintion!(
        nm(argument value:BigRational) => {
            Some(Expr::Product(vec![
                Expr::Num(value),
                Expr::con("nm")
            ]))
        }
    );
    let result = nm_decl.call(expr);
    println!("{:#?}", result);
    // // ------------------------------------------------------------------------
    let expr = Expr::from_str("energy(photon(wavelength = nm(325)))").unwrap();
    let decl1 = defintion!(
        energy => photon(keyword wavelength : Expr) => {{
            let numerator = Expr::Product(vec![
                Expr::con("c"),
                Expr::con("h"),
            ]);
            let denominator = wavelength;
            Some(Expr::ratio(
                numerator,
                denominator,
            ))
        }}
    );
    // let result = decl1.call(expr);
    // println!("{:#?}", result);
}



