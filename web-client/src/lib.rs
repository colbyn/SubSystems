#![allow(unused)]


pub mod browser;

use std::cell::{RefCell};
use std::rc::{Rc};
use wasm_bindgen::prelude::*;

use web_sys::HtmlCanvasElement;

// thread_local! {
//     pub(crate) static ROOT_PROCESS: RefCell<Option<Box<dyn ProgramImpl>>> = {
//         RefCell::new(None)
//     };
// }

// #[wasm_bindgen]
// pub fn on_request_animation_frame() {
//     ROOT_PROCESS.with(|cell| {
//         let inner: Option<Box<dyn ProgramImpl>> = cell.replace(None);
//         if let Some(mut process) = inner {
//             process.tick();
//             assert!(cell.replace(Some(process)).is_none());
//         }
//     });
// }


// pub(crate) trait ProgramImpl {
//     fn tick(&mut self);
// }





#[wasm_bindgen]
pub fn chem_eval(source: &str) -> js_sys::Object {
    // Roughly equivalent to `let obj = new Object; obj.foo = "bar";`
    let expr = compiler::ast::expr::Expr::from_str(source)
        .unwrap()
        .eval();
    let object = js_sys::Object::new();
    js_sys::Reflect::set(
        &object,
        &"ast".into(),
        &format!("{:#?}", expr).into()
    );
    js_sys::Reflect::set(
        &object,
        &"value".into(),
        &expr.to_string().into()
    );
    object
}


// let mut reaction = Reaction::from_str(source).unwrap();
//     println!("{}", reaction.to_string());
//     reaction.balance();

#[wasm_bindgen]
pub fn balance_reaction(source: &str) -> js_sys::Object {
    use compiler::chem::data::Reaction;
    // Roughly equivalent to `let obj = new Object; obj.foo = "bar";`
    let mut reaction = Reaction::from_str(source).unwrap();
    let object = js_sys::Object::new();
    js_sys::Reflect::set(
        &object,
        &"input_ast".into(),
        &format!("{:#?}", reaction).into()
    );
    js_sys::Reflect::set(
        &object,
        &"input_str".into(),
        &reaction.to_string().into()
    );
    reaction.balance();
    js_sys::Reflect::set(
        &object,
        &"output_ast".into(),
        &format!("{:#?}", reaction).into()
    );
    js_sys::Reflect::set(
        &object,
        &"output_str".into(),
        &reaction.to_string().into()
    );
    object
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // dom::run();
    // ROOT_PROCESS.with(move |cell| {
    //     // let old = cell.replace(Some(Box::new(program)));
    //     // assert!(old.is_none());
    // });
    console!("Hello World");
    
    Ok(())
}

