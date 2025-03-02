use dioxus::prelude::*;
use rfd::FileHandle;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        FileSelection{}
    }
}

#[component]
pub fn FileSelection() -> Element {
    let mut selected_paths = use_signal(|| Vec::<FileHandle>::new());
    rsx! {
        div {
            class: "app-container",
            div {
                class: "content-container",
                div{
                    class: "button-container",
                    button {
                        class: "btn",
                        onclick: move |_| async move {
                            if let Some(p) = rfd::AsyncFileDialog::new().pick_files().await {
                                selected_paths.set(p);
                            }
                        },
                        "Open Files"
                    },
                    button {
                        class: "btn",
                        onclick: move |_| async move {
                            if let Some(p) = rfd::AsyncFileDialog::new().pick_files().await {
                                selected_paths.extend(p);
                            }
                        },
                        "Add to selected"
                    }
                },
                div {
                    class: "content-container",
                    if !selected_paths.is_empty(){
                        p {"Selected files:"}
                    },
                    ul {
                        for path in selected_paths.iter().map(|p| p.file_name()){
                            li {"{path}"}
                        }
                    },
                    if selected_paths.is_empty(){
                        p {"Select files to proceed"}
                    }
                }
                if !selected_paths.is_empty(){
                    div {
                        class: "content-container",
                        p {"Select device"}
                        p {
                            "Heli",
                            p {"127.0.0.1"}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
