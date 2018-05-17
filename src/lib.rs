#![recursion_limit = "1024"]

#[macro_use]
extern crate stdweb;
use stdweb::web::HtmlElement;

use std::cell::RefCell;
use std::rc::Rc;
#[macro_use]
mod macros;

pub trait IComponent: Default {
    fn connected_callback(&mut self, _element: HtmlElement) {}
    fn constructor(&mut self, _element: HtmlElement) {}
    fn observed_attributes() -> &'static [&'static str] {
        &[]
    }
    fn attribute_changed_callback(
        &mut self,
        _element: HtmlElement,
        name: String,
        old_value: String,
        new_value: String,
    ) {
    }
}

pub fn register_component<T: 'static>(name: &str)
where
    T: IComponent,
{
    let component = Rc::new(RefCell::new(T::default()));

    js! {
        window.customElements.define(@{name}, class extends HTMLElement {
            constructor() {
                super();
                @{clone!(component => move |element| component.borrow_mut().constructor(element))}(this);
            }
            connectedCallback() {
                @{clone!(component => move |element| component.borrow_mut().connected_callback(element))}(this);
            }
            static get observedAttributes() {
                return @{T::observed_attributes}();
            }
            attributeChangedCallback(name, oldValue, newValue) {
                @{clone!(component => move |element, name, old_value, new_value| component.borrow_mut().attribute_changed_callback(element, name, old_value, new_value))}(this, name, newValue || "", oldValue || "");
            }
        });
    }
}
