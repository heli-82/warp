use dioxus::prelude::*;
use rfd::FileHandle;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

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
                    onclick: move |_| async move {
                        if let Some(p) = rfd::AsyncFileDialog::new().pick_files().await {
                            selected_paths.extend(p);
                        }
                    },
                    if !selected_paths.is_empty(){
                        h2 {"Selected files:"}
                    },
                    ul {
                        for path in selected_paths.iter().map(|p| p.file_name()){
                            li {"{path}"}
                        }
                    },
                    if selected_paths.is_empty(){
                        h2 {"Select files to proceed"}
                    }
                }
                if !selected_paths.is_empty(){
                    div {
                        class: "content-container",
                        h2 {"Select device"}
                        h3 {
                            "Heli",
                            p {"- 127.0.0.1"}
                        }
                    }
                }
            }
        }
    }
}
