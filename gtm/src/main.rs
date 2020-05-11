//#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

use web_view::*;

pub fn main() {
    let html = format!(
        r#"
        <!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>gtm Dashboard</title></head>
        <body class="bg-body text-primary">
        {scripts}
        </body></html>
        "#,
        scripts = inline_script(include_str!("../../dist/gtm/main.js"))
    );

    let mut webview = web_view::builder()
        .title("Rust Todo App")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(vec![])
        .invoke_handler(|webview, arg| {
            use Cmd::*;

            println!("invoke");

            let tasks_len = {
                let tasks = webview.user_data_mut();

                match serde_json::from_str(arg).unwrap() {
                    Init => (),
                    Log { text } => println!("{}", text),
                    AddTask { name } => tasks.push(Task { name, done: false }),
                    MarkTask { index, done } => tasks[index].done = done,
                    ClearDoneTasks => tasks.retain(|t| !t.done),
                }

                tasks.len()
            };

            webview.set_title(&format!("Rust Todo App ({} Tasks)", tasks_len))?;
            render(webview)
        })
        .build()
        .unwrap();

    webview.set_color((156, 39, 176));

    let res = webview.run().unwrap();

    println!("final state: {:?}", res);
}

pub fn render(webview: &mut WebView<Vec<Task>>) -> WVResult {
    let render_tasks = {
        let tasks = webview.user_data();
        println!("{:#?}", tasks);
        format!("rpc.render({})", serde_json::to_string(tasks).unwrap())
    };
    webview.eval(&render_tasks)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    done: bool,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
    AddTask { name: String },
    MarkTask { index: usize, done: bool },
    ClearDoneTasks,
}

pub fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

pub fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
