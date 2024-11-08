#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    #[route("/:..segments")]
    Login {},
    #[route("/dashboard")]
    Dashboard {}
}

fn Login() -> Element {
    rsx! {
        section {
            class: "bg-gray-50 dark:bg-gray-900",
            div {
                class: "flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0",
                a {
                    href: "#",
                    class: "flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white",
                    img {
                        class: "h-24 mr-2",
                        src: "https://www.backblaze.com/blog/wp-content/uploads/2021/06/image6-1024x366.png",
                        alt: "logo"
                    },
                }
                div {
                    class: "w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700",
                    div {
                        class: "p-6 space-y-4 md:space-y-6 sm:p-8",
                        h1 {
                            class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white",
                            "Sign in to your account"
                        }
                        form {
                            class: "space-y-4 md:space-y-6",
                            action: "#",
                            div {
                                label {
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Your Email"
                                },
                                input {
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    placeholder: "name@company.com",
                                    required: ""
                                }
                            }
                            div {
                                label {
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Your Password"
                                },
                                input {
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    placeholder: "••••••••",
                                    required: ""
                                }
                            }
                            button {
                                class: "w-full text-white bg-red-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                "Sign In"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn Dashboard() -> Element {
    todo!();
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

// pub fn App() -> Element {
//     let mut name = use_signal(|| String::new());
//     let mut greet_msg = use_signal(|| String::new());
//
//     let greet = move |_: FormEvent| async move {
//         if name.read().is_empty() {
//             return;
//         }
//
//         let name = name.read();
//         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
//         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//         let new_msg = invoke("greet", args).await.as_string().unwrap();
//         greet_msg.set(new_msg);
//     };
//
//     rsx! {
//         link { rel: "stylesheet", href: "styles.css" }
//         main {
//             class: "container",
//             h1 { "Welcome to Tauri + Dioxus" }
//
//             div {
//                 class: "row",
//                 a {
//                     href: "https://tauri.app",
//                     target: "_blank",
//                     img {
//                         src: "/tauri.svg",
//                         class: "logo tauri",
//                          alt: "Tauri logo"
//                     }
//                 }
//                 a {
//                     href: "https://dioxuslabs.com/",
//                     target: "_blank",
//                     img {
//                         src: "/dioxus.png",
//                         class: "logo dioxus",
//                         alt: "Dioxus logo"
//                     }
//                 }
//             }
//             p { "Click on the Tauri and Dioxus logos to learn more." }
//
//             form {
//                 class: "row",
//                 onsubmit: greet,
//                 input {
//                     id: "greet-input",
//                     placeholder: "Enter a name...",
//                     value: "{name}",
//                     oninput: move |event| name.set(event.value())
//                 }
//                 button { r#type: "submit", "Greet" }
//             }
//             p { "{greet_msg}" }
//         }
//     }
// }