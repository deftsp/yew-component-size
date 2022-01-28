use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_component_size::{ComponentSize, ComponentSizeObserver};

struct MyComponent {
    size: ComponentSize,
}

enum Msg {
    OnComponentSize(ComponentSize),
}

impl Component for MyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            size: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnComponentSize(size) => {
                self.size = size;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsize = ctx.link().callback(Msg::OnComponentSize);
        html! {
            <div style="position:relative">
                <span>{format!("width: {}px, height: {}px", self.size.width, self.size.height)}</span>
                <ComponentSizeObserver onsize={onsize} />
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn start_app() {
    yew::start_app::<MyComponent>();
}
