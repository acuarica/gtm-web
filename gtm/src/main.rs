//#![windows_subsystem = "windows"]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;

// use gtm::get_projects;
// use gtm::read_projects;
use git2::Error;
use structopt::StructOpt;
use web_view::*;

#[derive(StructOpt)]
struct Args {
    command: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    match args.command {
        Some(command) => println!("{}", command),
        _ => println!("{}", "nada"),
    }

    // let repo = Repository::open("tests/cases/repo")?;

    // let notes = gtm::get_notes(&repo)?;
    // println!("{}", serde_json::to_string(&notes).unwrap());
    // for n in ns {
    //     println!("{:?}", n)
    // }

    run_webview();
    Ok(())
}

fn run_webview() {
    let html = format!(
        r#"
        <!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>gtm Dashboard</title></head>
        <body class="bg-body text-primary">
        {scripts}
        </body></html>
        "#,
        scripts = inline_script(include_str!("../../dist/gtm/main.js"))
    );

    let webview = web_view::builder()
        .title("gtm Dashboard")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(vec![])
        .invoke_handler(|webview, _arg| {
            // use Cmd::*;

            println!("invoke");
            // webview.eval("app.$set({title: 'hola qwer sadfasdf'})");

            // let ps = read_projects("/Users/luigi/.git-time-metric/project.json").unwrap();
            // let ps = get_projects(&ps);
            // webview.eval(&format!(
            //     "app.$set({{projects: {} }})",
            //     serde_json::to_string(&ps).unwrap()
            // ));

            // let tasks_len = {
            //     let tasks = webview.user_data_mut();

            //     match serde_json::from_str(arg).unwrap() {
            //         Init => (),
            //         Log { text } => println!("{}", text),
            //         AddTask { name } => tasks.push(Task { name, done: false }),
            //         MarkTask { index, done } => tasks[index].done = done,
            //         ClearDoneTasks => tasks.retain(|t| !t.done),
            //     }

            //     tasks.len()
            // };

            webview.set_title(&format!("Rust Todo App ({} Tasks)", 9))?;
            render(webview)
        })
        .build()
        .unwrap();

    // webview.set_color((156, 39, 176));
    // cs.$set({commit:{Project: '23990239023'}})

    // webview.navigate("http://localhost:9090/dev/");
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
