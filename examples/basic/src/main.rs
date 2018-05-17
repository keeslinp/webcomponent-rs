extern crate stdweb;
extern crate webcomponent_rs;
use stdweb::{ web::{ HtmlElement, INode, Node, IElement, IEventTarget, event::ClickEvent} };
use webcomponent_rs::*;

#[derive(Default)]
struct TestComponent {
}

fn construct_dom(element: HtmlElement) {
    element.append_child(&Node::from_html("<div></div>").unwrap());
}

fn update_dom(element: HtmlElement) {
    let text = format!("Cheese {} {}", element.get_attribute("flavor").unwrap_or("cheese".into()), element.get_attribute("count").unwrap_or("0".into()));
    if let Some(child) = element.first_child() {
        child.set_text_content(&text);
    }
}

impl IComponent for TestComponent {
    fn connected_callback(&mut self, element: HtmlElement) {
        let element_clone = element.clone();
        element.add_event_listener(move |_: ClickEvent| {
            element_clone.set_attribute("count", &format!("{}", element_clone.get_attribute("count").map(|val| val.parse::<u8>().unwrap() + 1).unwrap_or(0))).unwrap();
            update_dom(element_clone.clone());
        });
        construct_dom(element.clone());
        update_dom(element.clone());
    }

    fn constructor(&mut self, element: HtmlElement) {
        if !element.has_attribute("count") {
            element.set_attribute("count", "0").unwrap();
        }
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["flavor"]
    }

    fn attribute_changed_callback(&mut self, element: HtmlElement, _name: String, _old_value: String, _new_value: String) {
        update_dom(element);
    }
}

fn main() {
    stdweb::initialize();

    register_component::<TestComponent>("test-component");

    stdweb::event_loop();
}

