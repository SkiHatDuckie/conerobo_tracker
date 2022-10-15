#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;

#[inline_props]
pub fn TextEditor(cx: Scope, text: UseRef<String>, num_rows: UseState<u32>) -> Element {
    let inner_text = &*text.read();

    cx.render(rsx!(
        textarea {
            value: "{inner_text}",
            rows: "{num_rows}",
            onkeydown: move |evt| {
                match evt.key.as_str() {
                    "Enter" => num_rows.modify(|n| n + 1),
                    "Backspace" => {
                        text.with(|txt| {
                            let lines: Vec<&str> = txt.split('\n').collect::<Vec<&str>>();
                            if lines[lines.len() - 1] == "" && lines.len() > 1 {
                                num_rows.modify(|n| n - 1)
                            }
                        })
                    },
                    _ => {}
                }
            },
            oninput: move |evt| text.set(evt.value.clone()),
        }
    ))
}

#[inline_props]
pub fn LineNumbers(cx: Scope, num_rows: UseState<u32>) -> Element {
    let numbered_lines = (1..=*num_rows.get()).map(|_| rsx!(span {}));

    cx.render(rsx! (
        div { class: "line-numbers",
            numbered_lines
        }
    ))
}