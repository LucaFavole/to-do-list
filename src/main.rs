mod task;
mod task_list;
use task_list::TaskList;
use std::io::{self, Read, Write};
use console::{Term, Style};
fn main() {
    let mut task_list = TaskList::load().unwrap_or_else(|_| TaskList { tasks: Default::default() });
    let term = Term::stdout();
    task_list.remove_task_done_with_past_deadline();
    task_list.display_task_not_done_with_today_deadline();
    task_list.display_task_not_done_with_past_deadline();
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }
        let first = input.split(" ").next().unwrap().to_ascii_lowercase();
        match parse_command(&*first) {
            Command::Add => {
                task_list.add_from_cmd(input);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
                },
            Command::ModifyDeadline => {
                let name = input.split(" ").nth(1).unwrap();
                let name = name.trim();
                let date = input.split(" ").nth(2).unwrap();
                let date = date.trim();
                task_list.modify_deadline(name, date);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
            }
            Command::ModifyDescription => {
                let name = input.split(" ").nth(1).unwrap();
                let name = name.trim();
                let description = input.split(" ").nth(2).unwrap();
                let description = description.trim();
                task_list.modify_description(name, description);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
            }
            Command::Complete => {
                let name = input.split(" ").nth(1).unwrap();
                let name = name.trim();
                task_list.complete(name);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
            }
            Command::Display => task_list.display(),
            Command::DisplayLongTask => {
                let name = input.split(" ").nth(1).unwrap();
                task_list.display_long_task(name);
            }
            Command::DisplayLong =>{
                task_list.display_long();
        }
            Command::Remove => {
                let name = input.split(" ").nth(1).unwrap();
                task_list.remove(name);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
            }
            Command::Quit => {
                break;
            }
            Command::Help => {
                term.write_line("add (a) <name> <deadline> - Add a task").unwrap();
                term.write_line("modify-deadline (mdead) <name> <new deadline> - Modify the deadline of a task").unwrap();
                term.write_line("modify-description (mdesc) <name> <new description> - Modify the description of a task").unwrap();
                term.write_line("add-topic (at) <name> <topic> - Add a topic to a task").unwrap();
                term.write_line("display-all-topic (dat) - Display all topics").unwrap();
                term.write_line("display-topic (dt) <topic> - Display all tasks with a topic").unwrap();
                term.write_line("make-not-removable (mnr) <name> - Make a task not removable").unwrap();
                term.write_line("display-not-removable (dnr) - Display all tasks that are not removable").unwrap();
                term.write_line("complete (c) <name> - Mark a task as done").unwrap();
                term.write_line("display (d) - Display all tasks").unwrap();
                term.write_line("display-long (dl) - Display all tasks with their description").unwrap();
                term.write_line("display-long-task (dlt) <name> - Display a task with its description").unwrap();
                term.write_line("remove (r) <name> - Remove a task").unwrap();
                term.write_line("quit (q) - Quit the program").unwrap();

            }
            Command::Error => {
                let red_underlined = Style::new().red().underlined();
                term.write_line(&red_underlined.apply_to("Invalid command").to_string()).unwrap()
                    }
            Command::AddTopic => {
                let name = input.split(" ").nth(1).unwrap();
                let name = name.trim();
                let topic = input.split(" ").nth(2).unwrap();
                let topic = topic.trim();
                task_list.add_topic(name, topic);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
            }
            Command::MakeNotRemovable => {
                let name = input.split(" ").nth(1).unwrap();
                let name = name.trim();
                task_list.make_not_removable(name);
                task_list.save().unwrap();
                //term.clear_screen().unwrap();
            }
            Command::DisplayNotRemovable => {
                task_list.display_not_removable();
            }
            Command::DisplayAllTopic => {
                task_list.display_all_topic();
            }
            Command::DisplayTopic => {
                let topic = input.split(" ").nth(1).unwrap();
                task_list.display_topic(topic);
            }
        }


}
}

enum Command {
    Add,
    ModifyDeadline,
    ModifyDescription,
    AddTopic,
    DisplayAllTopic,
    DisplayTopic,
    MakeNotRemovable,
    DisplayNotRemovable,
    Complete,
    Display,
    DisplayLong,
    DisplayLongTask,
    Remove,
    Quit,
    Help,
    Error,
}

fn parse_command(input: &str) -> Command {
    match input {
        "add" | "a" => Command::Add,
        "modify-deadline" | "mdead" => Command::ModifyDeadline,
        "modify-description" | "mdesc" => Command::ModifyDescription,
        "add-topic" | "at" => Command::AddTopic,
        "display-all-topic" | "dat" => Command::DisplayAllTopic,
        "display-topic" | "dt" => Command::DisplayTopic,
        "make-not-removable" | "mnr" => Command::MakeNotRemovable,
        "display-not-removable" | "dnr" => Command::DisplayNotRemovable,
        "complete" | "c" => Command::Complete,
        "display" | "d" => Command::Display,
        "display-long" | "dl" => Command::DisplayLong,
        "display-long-task" | "dlt" => Command::DisplayLongTask,
        "remove" | "r" => Command::Remove,
        "quit" | "q" => Command::Quit,
        "help" | "h" => Command::Help,
        _ => Command::Error,
    }
}