use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();

        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone();
        })
    };

    html! {
        <div class="bg-gradient-to-br from-violet-200 to-purple-300 flex w-screen h-screen justify-center items-center">

            <div class="bg-white shadow-2xl rounded-3xl p-10 w-96 flex flex-col items-center">

                <img
                    src="https://cdn-icons-png.flaticon.com/512/4712/4712027.png"
                    class="w-24 h-24 mb-4"
                    alt="chat-icon"
                />

                <h1 class="text-3xl font-bold text-violet-700 mb-2">
                    {"Renata's WebChat 💬"}
                </h1>

                <p class="text-gray-500 text-center mb-6">
                    {"Connect and chat in real-time"}
                </p>

                <div class="w-full flex flex-col gap-4">

                    <input
                        {oninput}
                        class="rounded-xl p-4 border text-gray-800 border-gray-200 bg-gray-50 focus:outline-none focus:ring-2 focus:ring-violet-400"
                        placeholder="Enter your username..."
                    />

                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="w-full bg-violet-600 hover:bg-violet-700 transition text-white font-bold p-4 rounded-xl shadow-md disabled:bg-gray-400"
                        >
                            {"Go Chatting! ✨"}
                        </button>
                    </Link<Route>>

                </div>

                <div class="text-xs text-gray-400 mt-6 text-center">
                    {"Real-time messaging experience"}
                </div>

            </div>
        </div>
    }
}