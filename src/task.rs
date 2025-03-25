use std::io::Write;
use serde::{Deserialize, Serialize};
use chrono::{Local};
use console::{Term, Style};
use crate::task_list::compare_date;
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) done: bool,
    pub(crate) deadline: String,
    pub(crate) topic : Option<String>,
    pub(crate) removable: bool
}

impl Task {
    pub fn new(name: String, description:String, date: String)-> Task{
        Task{
            name,
            description,
            done: false,
            deadline: if check_date(date.as_ref()) {date} else {Local::now().format("%d/%m/%Y").to_string()},
            topic: None,
            removable: true
        }
    }
    pub fn complete (&mut self){
        self.done=true;
    }
    pub fn display_short(&self){
        let done = if self.done { "Done" } else { "Not done" };
        let mut term = Term::stdout();
        let complete = if self.done {Style::new().green()} else {Style::new().red()};
        let blu = Style::new().blue().bold();
        let underlined = Style::new().underlined();
        term.write((&blu.apply_to(&self.name).to_string()).as_ref()).unwrap();
        term.write(" - ".as_ref()).unwrap();
        term.write(&underlined.apply_to(&self.deadline).to_string().as_ref()).unwrap();
        term.write(" - ".as_ref()).unwrap();
        term.write_line((&complete.apply_to(done).to_string()).as_ref()).unwrap();
    }
    pub fn display_long(&self) {
        let done = if self.done { "Done" } else { "Not done" };
        let mut term = Term::stdout();
        let complete = if self.done {Style::new().green()} else {Style::new().red()};
        let blu = Style::new().blue().bold();
        let underlined = Style::new().underlined();
        term.write((&blu.apply_to(&self.name).to_string()).as_ref()).unwrap();
        term.write(" - ".as_ref()).unwrap();
        term.write(&underlined.apply_to(&self.deadline).to_string().as_ref()).unwrap();
        term.write(" - ".as_ref()).unwrap();
        term.write_line((&complete.apply_to(done).to_string()).as_ref()).unwrap();
        term.write_line(self.description.as_ref()).unwrap()
    }
    // devo impl `Deserialize`
    pub fn from_json(json: &str) -> Result<Task, serde_json::Error> {
         serde_json::from_str(json)
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    pub fn modify_deadline(&mut self, date: String){
        if check_date(date.as_ref()){
            self.deadline = date;
        }
    }
    pub fn add_topic(&mut self, topic: String){
        self.topic = Option::from(topic);
    }
    pub fn make_not_removable(&mut self){
        self.removable = false;
    }
}

fn check_date(date: &str) -> bool {
    let datec = date.split("/").collect::<Vec<&str>>();
    if datec.len() != 3 {
        let mut term = Term::stdout();
        let red_underlined = Style::new().red().underlined();
        term.write_line(&red_underlined.apply_to("Invalid date").to_string()).unwrap();
        return false;
    }
    let day = datec[0].parse::<i32>().unwrap();
    let month = datec[1].parse::<i32>().unwrap();
    let year = datec[2].parse::<i32>().unwrap();
    if day < 1 || day > 31 || month < 1 || month > 12 || year < 2020 ||  compare_date(&date.to_string(), Local::now().format("%d/%m/%Y").to_string())<0 {
        let mut term = Term::stdout();
        let red_underlined = Style::new().red().underlined();
        term.write_line(&red_underlined.apply_to("Invalid date").to_string()).unwrap();
        return false;
    }
    true
}