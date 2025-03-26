use crate::{Term, Style, TaskList, Write};

fn check_input(input: &str, n: usize) -> bool{
    if input.split(" ").count() < n{
        let red_underlined = Style::new().red().underlined();
        let term = Term::stdout();
        term.write_line(&red_underlined.apply_to("Invalid command").to_string()).unwrap();
        return true
    }
    false
}
pub fn handle_add(task_list: &mut TaskList, input: &str) {
    if check_input(input, 3) { return; }
    let name = input.split(" ").nth(1).unwrap();
    let deadline = input.split(" ").nth(2).unwrap();
    let mut description = name.clone();
    if input.split(" ").count() >= 4 { description = input.split(" ").nth(3).unwrap(); }
    task_list.add_from_cmd(name, description.parse().unwrap(), deadline.parse().unwrap());
    task_list.save().unwrap();
}

pub fn handle_modify_deadline(task_list: &mut TaskList, input: &str) {
    if check_input(input, 3) { return; }
    let name = input.split(" ").nth(1).unwrap().trim();
    let date = input.split(" ").nth(2).unwrap().trim();
    task_list.modify_deadline(name, date);
    task_list.save().unwrap();
}

pub fn handle_modify_description(task_list: &mut TaskList, input: &str) {
    if check_input(input, 3) { return; }
    let name = input.split(" ").nth(1).unwrap().trim();
    let description = input.split(" ").nth(2).unwrap().trim();
    task_list.modify_description(name, description);
    task_list.save().unwrap();
}

pub fn handle_complete(task_list: &mut TaskList, input: &str) {
    if check_input(input, 2) { return; }
    let name = input.split(" ").nth(1).unwrap().trim();
    task_list.complete(name);
    task_list.save().unwrap();
}

pub fn handle_display_long_task(task_list: &mut TaskList, input: &str) {
    if check_input(input, 2) { return; }
    let name = input.split(" ").nth(1).unwrap();
    task_list.display_long_task(name);
}

pub fn handle_remove(task_list: &mut TaskList, input: &str) {
    if check_input(input, 2) { return; }
    let name = input.split(" ").nth(1).unwrap();
    task_list.remove(name);
    task_list.save().unwrap();
}

pub fn display_help(term: &Term) {
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
    term.write_line("display-by-date (dbd) -d date -i interval -al (anche le done) -l (le vedi long)- Display all tasks with a specific date").unwrap();
    term.write_line("remove (r) <name> - Remove a task").unwrap();
    term.write_line("quit (q) - Quit the program").unwrap();
}

pub fn display_error(term: &Term) {
    let red_underlined = Style::new().red().underlined();
    term.write_line(&red_underlined.apply_to("Invalid command").to_string()).unwrap();
}

pub fn handle_add_topic(task_list: &mut TaskList, input: &str) {
    if check_input(input, 3) { return; }
    let name = input.split(" ").nth(1).unwrap().trim();
    let topic = input.split(" ").nth(2).unwrap().trim();
    task_list.add_topic(name, topic);
    task_list.save().unwrap();
}

pub fn handle_make_not_removable(task_list: &mut TaskList, input: &str) {
    if check_input(input, 2) { return; }
    let name = input.split(" ").nth(1).unwrap().trim();
    task_list.make_not_removable(name);
    task_list.save().unwrap();
}

pub fn handle_display_topic(task_list: &TaskList, input: &str) {
    if check_input(input, 2) { return; }
    let topic = input.split(" ").nth(1).unwrap();
    task_list.display_topic(topic);
}

pub fn handle_display_by_date(task_list: &TaskList, input: &str) {
    let mut done = false;
    let not_done = true;
    let mut long = false;
    let mut date = "";
    let mut i = 1;
    let mut interval = 1;
    let mut topic = "";
    while i < input.split(" ").count() {
        let arg = input.split(" ").nth(i).unwrap();
        if arg == "-d" {
            date = input.split(" ").nth(i + 1).unwrap();
            i += 1;
        }
        if arg == "-al" {
            done = true;
        }
        if arg == "-l" {
            long = true;
        }
        if arg == "-i" {
            interval = input.split(" ").nth(i + 1).unwrap().parse::<i32>().unwrap();
            i += 1;
        }
        if arg == "-t" {
            topic = input.split(" ").nth(i + 1).unwrap();
            i += 1;
        }
        i += 1;
    }
    task_list.display_by_dates(interval, date.parse().unwrap(), done, not_done, long, topic);
}