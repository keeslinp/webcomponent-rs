#![recursion_limit = "1024"]

#[macro_use]
extern crate stdweb;
extern crate nanoid;
use stdweb::web::HtmlElement;

use std::cell::RefCell;
use std::rc::Rc;
#[macro_use]
pub mod macros;

pub trait IComponent: Default {
    fn connected_callback(&mut self, _element: HtmlElement) {}
    fn constructor(&mut self, _element: HtmlElement) {}
    fn observed_attributes() -> &'static [&'static str] {
        &[]
    }
    fn attribute_changed_callback(
        &mut self,
        _element: HtmlElement,
        _name: String,
        _old_value: String,
        _new_value: String,
    ) {
    }
}

fn parse_id(id: String) -> u8 {
    id.parse::<u8>().unwrap_or(0)
}

pub fn register_component<T: 'static>(name: &str)
where
    T: IComponent,
{
    use std::collections::HashMap;
    let component_pool: Rc<RefCell<HashMap<u8, T>>> = Rc::new(RefCell::new(HashMap::new()));
    let id_count = Rc::new(RefCell::new(0));
    js! {
        window.customElements.define(@{name}, class extends HTMLElement {
            constructor() {
                super();
                this.id = @{clone!(component_pool => move || {
                    let id = id_count.borrow().clone();
                    *id_count.borrow_mut() += 1;
                    component_pool.borrow_mut().insert(id, T::default());
                    id
                })}();
                @{{
                        let mut clone = component_pool.clone();
                        move |element, id| clone.borrow_mut().get_mut(&parse_id(id)).unwrap().constructor(element)
                }}(this, this.id);
            }
            connectedCallback() {
                @{{ let mut clone = component_pool.clone();
                    move |element, id| clone.borrow_mut().get_mut(&parse_id(id)).unwrap().connected_callback(element)}}(this, this.id);
            }
            static get observedAttributes() {
                return @{T::observed_attributes}();
            }
            attributeChangedCallback(name, oldValue, newValue) {
                @{{
                    let clone = component_pool.clone();
                    move |element, id, name, old_value, new_value| clone.borrow_mut().get_mut(&parse_id(id)).unwrap().attribute_changed_callback(element, name, old_value, new_value)
                }}(this, this.id, name, newValue || "", oldValue || "");
            }
        });
    }
}
