use yew::prelude::*;

#[function_component(App)]
fn app() -> Html{
    html!{
        <>
            <h1>{"First Yew App"}</h1>
            <p>{"Hello World :))))"}</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
