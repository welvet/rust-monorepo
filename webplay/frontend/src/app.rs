use chrono::prelude::*;
use chrono::Duration;
use yew::prelude::*;

use util::simple_add;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub struct App {
    diff: Duration,
    fib: u32,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ButtonClick,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            diff: Duration::zero(),
            fib: 0,
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        let start = Local::now();
        self.fib = simple_add(fibonacci(42));
        self.diff = Local::now() - start;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="grid-container">
                <div class="grid-x grid-padding-x">
                    <div class="large-12 cell">
                        <h1>{"Welcome to Foundation"}</h1>
                    </div>
                </div>

                <div class="grid-x grid-padding-x">
                    <div class="large-12 cell">
                        <p>{ format!("Diff: {}", self.diff) }</p>
                        <p>{ format!("Fib: {}", self.fib) }</p>

                        <a href="#" class="button"
                           onclick=self.link.callback(|_| Msg::ButtonClick)>{"Click Me"}</a>
                    </div>
                </div>
            </div>
        }
    }
}
