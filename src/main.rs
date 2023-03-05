mod numbers;
mod verbs;

use numbers::numbers_test;
use yew::prelude::*;
use yew::ChangeData;

#[function_component]
fn App() -> Html {
    let query = use_state(|| "");
    let onchange = {
	let query = query.clone();
	Callback::from(move |event: ChangeData| {
	    match event {
		ChangeData::Value(s) => { query.set(input.to_string());}
		_ => {}
	    }
	})
    };
    
    html! {
	<div>
	    <div>{"こんにちは日本語！"}</div>
	    <input type="text" value={query} onchange={onchange} />
	</div>
    }
}

fn main() {
    // run_verb_test();
    // numbers_test();
    yew::Renderer::<App>::new().render();
}
