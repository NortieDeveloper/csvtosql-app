use std::collections::HashMap;
use csvtosql_core::sql_builder;
use yew::prelude::*;
use gloo_file::{Blob, File};
use gloo_file::callbacks::FileReader;
use web_sys::{Event, HtmlInputElement};
fn main() {
    yew::start_app::<App>();
}

pub enum Msg{
    ProcessFile,
    FileSelected(File),
    FileProcessed(String, String),
    SetDatabase(String),
    SetTable(String),
    CopySqlToClipboard
}

struct App{
    selected_file: Option<File>,
    readers: HashMap<String, FileReader>,
    sql: Option<String>,
    error: Option<String>,
    database: String,
    table: String
}

impl Component for App{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            selected_file: None,
            readers: HashMap::default(),
            sql: None,
            error: None,
            database: "default_db".to_string(),
            table: "default_table".to_string()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::ProcessFile => {
                if self.selected_file.is_none(){
                    self.error = Some("Please select a file.".to_string());
                    return true
                }

                let link = ctx.link().clone();
                let file_name = self.selected_file.as_ref().unwrap().name();

                let file_type:String = self.selected_file.as_ref().unwrap().raw_mime_type();

                if file_type != "text/csv"{
                    self.error = Some("Selected file is not a csv.".to_string());
                    return true
                }

                let db = self.database.clone();
                let table = self.table.clone();

                let blob:Blob = self.selected_file.clone().unwrap().into();
                let reader = gloo_file::callbacks::read_as_text(&blob, move |res|{

                    let file = res.expect("Failed to read file.");

                    let headers = csvtosql_core::csv_helper::extract_headers(file.as_str()).unwrap();

                    let sql = sql_builder::build_sql_statement(headers, &table, &db);
                    link.send_message(Msg::FileProcessed(
                        file_name,
                        sql
                    ))
                });
                self.readers.insert(self.selected_file.as_ref().unwrap().name(), reader);
                self.error = None;
                true
            }
            Msg::FileSelected(file) => {
                self.selected_file = Some(file);
                true
            }
            Msg::FileProcessed(file_name, sql) => {
                self.readers.remove(&file_name);
                self.sql = Some(sql);
                true
            }
            Msg::SetDatabase(db) => {
                self.database = db;
                false
            }
            Msg::SetTable(table) => {
                self.table = table;
                false
            }
            Msg::CopySqlToClipboard => {
                let window = web_sys::window().expect("no window exists");
                let document = window.document().expect("no document exists");

                let val = document.get_element_by_id("sql");

                let navigator = window.navigator();
                let clipboard = navigator.clipboard().expect("can't access clipboard");
                let _ = clipboard.write_text(val.unwrap().inner_html().as_str());

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html!{
            <div class={classes!("mx-auto","max-w-2xl", "m-8")}>
            <h1 class={classes!("font-bold", "text-3xl", "text-center")}>{"CsvToSql"}</h1>
            <h1 class={classes!( "text-xl", "text-center", "mb-8")}>{"Utility for generating SQL table creation statements from CSV files."}</h1>
            <div class={classes!("mb-8", "bg-gray-200","rounded", "shadow-lg", "p-4", "flex", "flex-col", "space-y-4")}>

            <label class={classes!("font-bold")} for="database">{"Database"}</label>
            <input name="database" type="text" value={self.database.clone()} class={classes!("bg-gray-50", "p-2", "rounded")} onchange={link.callback(move |e: Event|{
                let input: HtmlInputElement = e.target_unchecked_into();
                Msg::SetDatabase(input.value())
            }
            )}/>

            <label class={classes!("font-bold")} for="table">{"Table"}</label>
            <input name="table" type="text" value={self.table.clone()} class={classes!("bg-gray-50", "p-2", "rounded")} onchange={link.callback(move |e: Event|{
                let input: HtmlInputElement = e.target_unchecked_into();
                Msg::SetTable(input.value())
            }
            )}/>

            <input type="file" onchange={link.callback(move |e: Event|{
                let input: HtmlInputElement = e.target_unchecked_into();
                Msg::FileSelected(input.files().unwrap().get(0).unwrap().into())
            }
            )}/>
            <button onclick={link.callback(|_| Msg::ProcessFile)} class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "py-2", "px-4", "rounded", "font-bold")}>{"Process"}</button>
            </div>
            if self.error.is_some(){
                <p class={classes!("text-red-700", "font-bold", "mb-8")}>{format!("Failed to process file. Error: {}", self.error.as_ref().unwrap())}</p>
            }

            if self.sql.is_some() && self.error.is_none(){
                <div class={classes!("bg-gray-200","rounded", "shadow-lg", "p-4", "flex", "flex-col", "space-y-4", "mb-8")}>
                <button onclick={link.callback(|_| Msg::CopySqlToClipboard)} class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "py-2", "px-4", "rounded", "font-bold", "mb-4")}>{"Copy to Clipboard"}</button>

                <code id="sql" class={classes!("block","overflow-x-scroll", "whitespace-pre", "bg-gray-100", "border-2", "border-black")}>{self.sql.as_ref().unwrap()}</code>

                </div>
            }
            <footer class={classes!("flex", "justify-center")}><a class={classes!("block", "h-8", "w-8")} href="https://github.com/NortieDeveloper/csvtosql-app"><img alt="GitHub icon" src="assets/github-icon.png" /></a></footer>
            </div>
        }
    }
}



