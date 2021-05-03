pub mod utils;
pub mod css;

use core::fmt::Debug;
use std::fmt;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Function;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use crate::browser::utils::{is_svg_tag};

pub mod console {
    pub mod internal_reexports {
        pub use ::wasm_bindgen::JsValue;
        pub use ::web_sys;
    }

    #[macro_export]
    macro_rules! console {
        ($($rest:tt)*) => {{
            use $crate::browser::console::internal_reexports::*;

            let value: String = format!($($rest)*);
            let value: JsValue = JsValue::from_str(value.as_str());
            web_sys::console::log_1(&value);
        }};
    }
}

pub mod prelude {
    pub use super::NodeApi;
}

///////////////////////////////////////////////////////////////////////////////
// WINDOW GETTER
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_WINDOW: Window = {
    	let window_instance: web_sys::Window = web_sys::window().expect("window not available");
    	let document_instance: web_sys::Document = window_instance.document().expect("document not available");
    	let body_instance = document_instance.body().expect("document.body not available");
        let local_storage_instance = window_instance
            .local_storage()
            .expect("localStorage failed")
            .expect("localStorage missing");
        let location_instance = window_instance
            .location();
        let history_instance = window_instance
            .history()
            .expect("window.history getter failed");
        let window = Window {
        	document: Document {
        		body: Body {
        			instance: From::from(body_instance),
        		},
			    instance: From::from(document_instance),
        	},
            local_storage: Storage {
                instance: local_storage_instance,
            },
            location: Location {instance: location_instance},
            history: History {instance: history_instance},
            instance: From::from(window_instance),
        };
        window
    };
}


pub fn window() -> Window {
    let win = GLOBAL_WINDOW.with(|win| win.clone());
    win
}

///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Window {
	pub instance: JsValue,
	pub document: Document,
    pub local_storage: Storage,
    pub location: Location,
    pub history: History,
}
#[derive(Clone, Debug)]
pub struct Document {
	pub instance: JsValue,
	pub body: Body,
}
#[derive(Clone, Debug)]
pub struct Storage {
    pub instance: web_sys::Storage
}
#[derive(Clone, Debug)]
pub struct Location {
    pub instance: web_sys::Location
}
#[derive(Clone, Debug)]
pub struct History {
    pub instance: web_sys::History
}


///////////////////////////////////////////////////////////////////////////////
// METHODS
///////////////////////////////////////////////////////////////////////////////

impl Window {
	pub fn instance_as_window(&self) -> web_sys::Window {
		From::from(self.instance.clone())
	}
	pub fn request_animation_frame(&self, callback: impl FnOnce() + 'static) -> Closure<dyn  FnMut()> {
	    let callback: Closure<dyn FnMut()> = Closure::once(callback);
	    let js_function: &js_sys::Function = callback.as_ref().unchecked_ref();
	    self.instance_as_window()
	    	.request_animation_frame(js_function)
	        .expect("request_animation_frame failed");
	    callback
	}
	pub fn set_timeout(&self, timeout: i32, callback: impl FnOnce() + 'static) -> Closure<dyn  FnMut()> {
	    let callback: Closure<dyn  FnMut()> = Closure::once(callback);
	    let js_function: &js_sys::Function = callback.as_ref().unchecked_ref();
	    self.instance_as_window().set_timeout_with_callback_and_timeout_and_arguments_0(
	        js_function,
	        timeout
	    ).expect("set_timeout_with_callback_and_timeout_and_arguments_0 failed");
	    callback
	}
    pub fn add_event_listener(&self, event_name: &str, callback: &dyn  EventListenerApi) {
        self.instance_as_window()
            .add_event_listener_with_callback(event_name, callback.as_js_function())
            .expect("Window.addEventListener failed");
    }
    pub fn remove_event_listener(&self, event_name: &str, callback: &dyn  EventListenerApi) {
        self.instance_as_window()
            .remove_event_listener_with_callback(event_name, callback.as_js_function())
            .expect("Window.removeEventListener failed");
    }
}

impl Document {
	pub fn instance_as_document(&self) -> web_sys::Document {
		From::from(self.instance.clone())
	}
	pub fn create_element(&self, tag: &str) -> Element {
        let instance = if is_svg_tag(tag) {
        	self.instance_as_document()
        		.create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        	    .expect("failed to create svg-element")
        } else {
            self.instance_as_document()
            	.create_element(tag)
                .expect("failed to create element")
        };
        let class_list = ClassList {
        	instance: From::from(instance.class_list())
        };
        Element {instance: From::from(instance), class_list}
	}
	pub fn create_text_node(&self, value: &str) -> Text {
		let instance = self.instance_as_document().create_text_node(value);
	    Text {instance: From::from(instance)}
	}
}

impl Storage {
    pub fn get<Value>(&self, key: &str) -> Option<Value> where Value: DeserializeOwned {
        let value = self.instance
            .get_item(key)
            .expect("getItem method failed");
        match value {
            None => None,
            Some(value) => match serde_json::from_str(value.clone().as_str()) {
                Err(msg) => None,
                Ok(value) => Some(value)
            }
        }
    }
    pub fn set<Value: Serialize>(&self, key: &str, value: &Value) {
        match serde_json::to_string(value) {
            Err(msg) => (),
            Ok(value) => self.instance
                .set_item(key, value.as_str())
                .expect("setItem method failed")
        }
    }
    pub fn remove(&self, key: &str) {
        self.instance
            .remove_item(key)
            .expect("removeItem method failed")
    }
}
impl History {
    pub fn push_state(&self, url_path: &str) {
        self.instance.push_state_with_url(
            &JsValue::null(),
            "",
            Some(url_path)
        )
        .expect("pushState failed");
    }
}
impl Location {
    pub fn pathname(&self) -> String {
        self.instance
            .pathname()
            .expect("pathname failed")
    }
}


///////////////////////////////////////////////////////////////////////////////
// COMMON DOM APIs
///////////////////////////////////////////////////////////////////////////////

pub trait EventListenerApi {
	fn as_js_function(&self) -> &Function;
}
pub trait NodeApi {
	fn box_clone(&self) -> Box<dyn NodeApi>;
	fn dom_ref(&self) -> JsValue;
	fn dom_ref_as_node(&self) -> web_sys::Node {
		let value: web_sys::Node = From::from(self.dom_ref());
		value
	}
	fn add_event_listener(&self, event_name: &str, callback: &dyn  EventListenerApi) {
	    self.dom_ref_as_node()
	    	.add_event_listener_with_callback(event_name, callback.as_js_function())
	        .expect("addEventListener failed");
	}
	fn remove_event_listener(&self, event_name: &str, callback: &dyn  EventListenerApi) {
	    self.dom_ref_as_node()
	    	.remove_event_listener_with_callback(event_name, callback.as_js_function())
	        .expect("removeEventListener failed");
	}
	fn append_child(&self, child: &dyn NodeApi) {
	    self.dom_ref_as_node()
	        .append_child(&child.dom_ref_as_node())
	        .expect("appendChild failed");
	}
	fn remove_child(&self, child: &dyn NodeApi) {
	    self.dom_ref_as_node()
	        .remove_child(&child.dom_ref_as_node())
	        .expect("removeChild failed");
	}
	fn replace_child(&self, new_child: &dyn NodeApi, old_child: &dyn NodeApi) {
	    self.dom_ref_as_node()
	        .replace_child(&new_child.dom_ref_as_node(), &old_child.dom_ref_as_node())
	        .expect("replacedNode failed");
	}
	fn insert_before(&self, new_child: &dyn NodeApi, ref_child: &dyn NodeApi) {
	    self.dom_ref_as_node()
	        .insert_before(
	            &new_child.dom_ref_as_node(),
	            Some(&ref_child.dom_ref_as_node()),
	        )
	        .expect("replacedNode failed");
	}
}

pub trait ElementApi: NodeApi {
	fn dom_ref_as_element(&self) -> web_sys::Element {
		let value: web_sys::Element = From::from(self.dom_ref());
		value
	}
	fn set_attribute(&self, key: &str, value: &str) {
	    self.dom_ref_as_element()
	    	.set_attribute(key, value)
	        .expect("setAttribute failed");
	}
	fn remove_attribute(&self, key: &str) {
	    self.dom_ref_as_element()
	    	.remove_attribute(key)
	        .expect("removeAttribute failed");
	}
	fn insert_adjacent_element(&self, position: AdjacentPosition, element: &dyn ElementApi) {
		self.dom_ref_as_element()
			.insert_adjacent_element(
				position.as_str(),
				&element.dom_ref_as_element(),
			)
			.expect("ElementApi.insert_adjacent_element failed");
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AdjacentPosition {
	/// Before the targetElement
	BeforeBegin,
	/// Inside the targetElement; before its <b>first child</b>.
	AfterBegin,
	/// Inside the targetElement; after its <b>last child</b>.
	BeforeEnd,
	/// After the targetElement.
	AfterEnd,
}

impl AdjacentPosition {
	pub fn as_str(&self) -> &str {
		match self {
			AdjacentPosition::BeforeBegin => "beforebegin",
			AdjacentPosition::AfterBegin => "afterbegin",
			AdjacentPosition::BeforeEnd => "beforeend",
			AdjacentPosition::AfterEnd => "afterend",
		}
	}
}


///////////////////////////////////////////////////////////////////////////////
// DOM NODES
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Body {
	pub instance: JsValue,
}
#[derive(Clone, Debug)]
pub struct Text {
	pub instance: JsValue,
}
#[derive(Clone, Debug)]
pub struct Element {
	pub instance: JsValue,
	pub class_list: ClassList,
}
#[derive(Clone, Debug)]
pub struct ClassList {
	pub instance: JsValue,
}

impl NodeApi for Text {
	fn box_clone(&self) -> Box<dyn NodeApi> {Box::new(self.clone())}
	fn dom_ref(&self) -> JsValue {self.instance.clone()}
}
impl NodeApi for Element {
	fn box_clone(&self) -> Box<dyn NodeApi> {Box::new(self.clone())}
	fn dom_ref(&self) -> JsValue {self.instance.clone()}
}
impl NodeApi for Body {
	fn box_clone(&self) -> Box<dyn NodeApi> {Box::new(self.clone())}
	fn dom_ref(&self) -> JsValue {self.instance.clone()}
}

impl ElementApi for Body {}
impl ElementApi for Element {}

impl Text {
	pub fn dom_ref_as_text(&self) -> web_sys::Text {
		From::from(self.instance.clone())
	}
	pub fn set_text_content(&self, new_value: &str) {
		self.dom_ref_as_text()
			.set_text_content(Some(new_value));
	}
}

impl ClassList {
	pub fn dom_ref_as_dom_token_list(&self) -> web_sys::DomTokenList {
		From::from(self.instance.clone())
	}
	pub fn add(&self, class: &str) {
		let interface = self.dom_ref_as_dom_token_list();
		interface
			.add_1(class)
			.expect("ClassList.add() method failed");
	}
	pub fn remove(&self, class: &str) {
		let interface = self.dom_ref_as_dom_token_list();
		interface
			.remove_1(class)
			.expect("ClassList.remove() method failed");
	}
    pub fn replace(&self, old: &str, new: &str) {
        let interface = self.dom_ref_as_dom_token_list();
        interface
            .replace(old, new)
            .expect("ClassList.replace() method failed");
    }
}

impl Element {
	pub fn dom_ref_as_html_style_element(&self) -> web_sys::HtmlStyleElement {
		From::from(self.instance.clone())
	}
}


///////////////////////////////////////////////////////////////////////////////
// CSSOM
///////////////////////////////////////////////////////////////////////////////

pub struct Stylesheet {
	instance: Element,
}

impl Stylesheet {
	pub fn dom_ref_as_css_style_sheet(&self) -> web_sys::CssStyleSheet {
		let interface = self.instance.dom_ref_as_html_style_element();
		let sheet: web_sys::StyleSheet = interface
			.sheet()
			.expect("sheet getter failed");
		let sheet: JsValue = From::from(sheet);
		let sheet: web_sys::CssStyleSheet = From::from(sheet);
		sheet
	}

	pub fn from_element(element: Element) -> Self {
		Stylesheet {instance: element}
	}
	
	pub fn push_declaration(&self, value: css::Declaration) {
		let interface = self.dom_ref_as_css_style_sheet();
		interface
			.insert_rule(&value.as_str())
			.expect("insertRule() method failed");
	}
	pub fn push_keyframes(&self, value: css::Keyframes) {
		let interface = self.dom_ref_as_css_style_sheet();
		interface
			.insert_rule(&value.as_str())
			.expect("insertRule() method failed");
	}
	pub fn push_media(&self, value: css::Media) {
		let interface = self.dom_ref_as_css_style_sheet();
		interface
			.insert_rule(&value.as_str())
			.expect("insertRule() method failed");
	}
}






///////////////////////////////////////////////////////////////////////////////
// CALLBACKS - MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct CallbackSettings {
    pub stop_propagation: bool,
    pub prevent_default: bool,
}

impl Default for CallbackSettings {
	fn default() -> Self {
		let stop_propagation = false;
		let prevent_default = false;
		CallbackSettings{stop_propagation,prevent_default}
	}
}

fn callback_settings_handler(settings: CallbackSettings, value: &JsValue) {
    let event: web_sys::Event = From::from(value.clone());
    if settings.prevent_default {
        event.prevent_default();
    }
    if settings.stop_propagation {
        event.stop_propagation();
    }
}

impl EventListenerApi for js_sys::Function {
    fn as_js_function(&self) -> &Function {
    	self
    }
}

///////////////////////////////////////////////////////////////////////////////
// QUEUE-CALLBACK
///////////////////////////////////////////////////////////////////////////////

pub struct QueueCallback {
    settings: CallbackSettings,
    bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
    events: Rc<RefCell<VecDeque<JsValue>>>,
}
impl QueueCallback {
    pub fn new(dom_ref: &dyn NodeApi, event_type: &str, settings: CallbackSettings) -> Self {
        let events_queue: Rc<RefCell<VecDeque<JsValue>>> = Rc::new(RefCell::new(VecDeque::new()));
        let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let events_queue = events_queue.clone();
            let settings = settings.clone();
            move |value: JsValue| {
                callback_settings_handler(settings.clone(), &value);
                events_queue.borrow_mut().push_back(value);
            }
        }));
        let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
        dom_ref.add_event_listener(event_type, js_function);
        QueueCallback {settings, bindgen_closure: Rc::new(bindgen_closure),events: events_queue}
    }
    pub fn drain(&self) -> Vec<JsValue> {
        self.events.borrow_mut().drain(..).collect()
    }
}
impl std::fmt::Debug for QueueCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "QueueCallback")
    }
}
impl PartialEq for QueueCallback {
    fn eq(&self, other: &QueueCallback) -> bool {true}
}
impl EventListenerApi for QueueCallback {
    fn as_js_function(&self) -> &Function {
    	use wasm_bindgen::JsCast;
    	let bindgen_closure: &Closure<dyn Fn(JsValue)> = &self.bindgen_closure;
    	bindgen_closure.as_ref().unchecked_ref()
    }
}



///////////////////////////////////////////////////////////////////////////////
// VOID-CALLBACK
///////////////////////////////////////////////////////////////////////////////

pub struct VoidCallback {
    settings: CallbackSettings,
    callback: Option<Rc<dyn Fn(JsValue)>>,
    bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
}
impl VoidCallback {
	pub fn new(dom_ref: &dyn NodeApi, event_type: &str, settings: CallbackSettings) -> Self {
	    let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
	        let settings = settings.clone();
	        move |value: JsValue| {
	            callback_settings_handler(settings.clone(), &value);
	        }
	    }));
	    let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
	    dom_ref.add_event_listener(event_type, js_function);
	    VoidCallback {settings, callback: None, bindgen_closure: Rc::new(bindgen_closure)}
	}
    pub fn new_with_fn(dom_ref: &dyn NodeApi, event_type: &str, settings: CallbackSettings, callback: impl Fn(JsValue) + 'static) -> Self {
        let callback = Rc::new(callback);
        let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let callback = callback.clone();
            let settings = settings.clone();
            move |value: JsValue| {
                callback_settings_handler(settings.clone(), &value);
                callback(value);
            }
        }));
        let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
        dom_ref.add_event_listener(event_type, js_function);
        VoidCallback {settings, callback: Some(callback), bindgen_closure: Rc::new(bindgen_closure)}
    }
    pub fn new_with_fn_unset(settings: CallbackSettings, callback: impl Fn(JsValue) + 'static) -> Self {
        let callback = Rc::new(callback);
        let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let callback = callback.clone();
            let settings = settings.clone();
            move |value: JsValue| {
                callback_settings_handler(settings.clone(), &value);
                callback(value);
            }
        }));
        let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
        VoidCallback {settings, callback: Some(callback), bindgen_closure: Rc::new(bindgen_closure)}
    }
}
impl std::fmt::Debug for VoidCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VoidCallback")
    }
}
impl PartialEq for VoidCallback {
    fn eq(&self, other: &VoidCallback) -> bool {true}
}
impl EventListenerApi for VoidCallback {
    fn as_js_function(&self) -> &Function {
    	let bindgen_closure: &Closure<dyn Fn(JsValue)> = &self.bindgen_closure;
    	bindgen_closure.as_ref().unchecked_ref()
    }
}


