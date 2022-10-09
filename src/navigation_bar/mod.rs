#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;
use crate::welcome_page;
use crate::editor_page;

pub fn NavigationBar(cx: Scope) -> Element {
    let active_screen = use_state(&cx, || 0);
    // Temporarily storing the editor page's states here.
    // Later on, I want to have some other component handle screen states when a screen isn't being rendered.
    let editor_text = use_ref(&cx, || String::new());
    let editor_num_rows = use_state(&cx, || 1);
    let editor_filename = use_ref(&cx, || String::new());

    let screen = match active_screen.get() {
        1 => { rsx!(welcome_page::WelcomePage {}) },
        2 => { 
            rsx!(editor_page::EditorPage {
                text: editor_text.clone(),
                num_rows: editor_num_rows.clone(),
                filename: editor_filename.clone()
            })
        },
        _ => { rsx!(div {}) }
    };

    cx.render(rsx! (
        div {
            button {
                onclick: move |_| {
                    match active_screen.get() {
                        1 => active_screen.set(0),
                        _ => active_screen.set(1)
                    }
                },
                "Welcome!"
            }
            button {
                onclick: move |_| {
                    match active_screen.get() {
                        2 => active_screen.set(0),
                        _ => active_screen.set(2)
                    }
                },
                "Editor"
            }
        }
        div {
            screen
        }
    ))
}