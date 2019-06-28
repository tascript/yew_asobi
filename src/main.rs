use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model { 
    state: State
}

struct State {
    greet: String,
    counter: i32,
}

impl State {
    fn morning(&mut self) {
        self.greet = "おはよう".to_string()
    }
    fn noon(&mut self) {
        self.greet = "こんにちは".to_string()
    }
    fn night(&mut self) {
        self.greet = "こんばんは".to_string()
    }
    fn count_up(&mut self) {
        self.counter = self.counter + 1
    }
}

enum Msg {
    ChangeGreet()
}

impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state: State {
                greet: "...".to_string(),
                counter: 0,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeGreet() => {
                self.state.count_up();
                if self.state.counter % 3 == 0 {
                    self.state.night();
                } else if self.state.counter % 3 == 1 {
                    self.state.morning();
                } else {
                    self.state.noon();
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>{&self.state.greet}</div>
            <p>{&self.state.counter.to_string()}{"回目のあいさつ"}</p>
            <button onclick=|_| Msg::ChangeGreet(), >{ "あいさつするよ" }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}