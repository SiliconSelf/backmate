//! The login page the user will use to authenticate with Backblaze

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::views::Route;

/// Arguments used to invoke authentication via tauri
#[derive(Serialize, Deserialize)]
struct LoginArgs<'a> {
    /// The API Key ID
    api_key_id: &'a str,
    /// The API Key
    api_key: &'a str,
}

/// Render the login view
#[allow(non_snake_case)]
pub(crate) fn Login() -> Element {
    rsx! {
        section {
            class: "bg-gray-50 dark:bg-gray-900",
            div {
                class: "flex flex-col items-center justify-center px-6 mx-auto md:h-screen lg:py-0",
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
                                    "Application Key ID"
                                },
                                input {
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    placeholder: "",
                                    required: ""
                                }
                            }
                            div {
                                label {
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Application Key"
                                },
                                input {
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    placeholder: "••••••••",
                                    required: ""
                                }
                            }
                            div {
                                class: "mt-2",
                                Link {
                                to: Route::Dashboard {},
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
    }
}
