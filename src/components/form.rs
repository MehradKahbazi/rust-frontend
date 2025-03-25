use yew::prelude::*;

use crate::components::input::Input;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let username_changed = Callback::from(|e: Event| {

    });

    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let password_changed = Callback::from(|e: Event| {

    });


    let onsubmit = Callback::from(|e: SubmitEvent| {
        e.prevent_default();
        
    });
    html! {
        <form onsubmit={onsubmit}>
                
                    <div class="mb-3">
                        <Input 
                        input_type="text" 
                        name="username" 
                        label="User Name"
                        value={username}
                        onchange={username_changed}
                         />
                    </div>

                    <div class="mb-3">
                        <Input 
                        input_type="password" 
                        name="password" 
                        label="Password"
                        value={password}
                        onchange={password_changed}
                         />
                    </div>
                
                    <button class="btn btn-primary" type="submit">{"Submit"}</button>

                </form>
    }
}