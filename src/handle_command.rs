use crate::{Term, Style, TaskList, Write, Command, parse_command, help, help_add, help_modify_deadline, help_modify_description, help_complete, help_display, help_display_long_task, help_display_long, help_remove, help_quit, help_help, help_add_topic, help_make_not_removable, help_display_not_removable, help_display_all_topic, help_display_topic, help_display_by_date, help_clear};

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
    if input.split(" ").count() >= 4 {
        let desc = input.split(" ").skip(3).collect::<Vec<&str>>().join(" ");
        description = desc.as_str().clone();
        task_list.add_from_cmd(name, description.parse().unwrap(), deadline.parse().unwrap());
    }else{
        task_list.add_from_cmd(name, description.parse().unwrap(), deadline.parse().unwrap());
    }

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
    let description =  &*input.split(" ").skip(2).collect::<Vec<&str>>().join(" ");
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

pub fn display_help(input: &str) {
    if input.split(" ").count()==1{
        help();
        return;
    }
    match parse_command(input.split(" ").nth(1).unwrap()){
        Command::Add => help_add(),
        Command::ModifyDeadline => help_modify_deadline(),
        Command::ModifyDescription => help_modify_description(),
        Command::Complete => help_complete(),
        Command::Display => help_display(),
        Command::DisplayLongTask => help_display_long_task(),
        Command::DisplayLong => help_display_long(),
        Command::Remove => help_remove(),
        Command::Quit => help_quit(),
        Command::Help => help_help(),
        Command::AddTopic => help_add_topic(),
        Command::MakeNotRemovable => help_make_not_removable(),
        Command::DisplayNotRemovable => help_display_not_removable(),
        Command::DisplayAllTopic => help_display_all_topic(),
        Command::DisplayTopic => help_display_topic(),
        Command::Clear => help_clear(),
        Command::DisplayByDate => help_display_by_date(),
        Command::Error => {
            let red_underlined = Style::new().red().underlined();
            let term = Term::stdout();
            term.write_line(&red_underlined.apply_to("Invalid command").to_string()).unwrap();
        }
    }

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
    let mut interval = 0;
    let mut topic = "";
    while i < input.split(" ").count() {
        let arg = input.split(" ").nth(i).unwrap();
        if arg == "-d" {
            if i + 1 < input.split(" ").count() {
                let next_arg = input.split(" ").nth(i + 1).unwrap();
                if !next_arg.starts_with('-') {
                    date = next_arg;
                    i += 1;
                }
            }
        }
        if arg == "-al" {
            done = true;
        }
        if arg == "-l" {
            long = true;
        }
        if arg == "-i" {
            if i + 1 < input.split(" ").count() {
                let next_arg = input.split(" ").nth(i + 1).unwrap();
                    if next_arg.parse::<i32>().is_ok(){
                        interval = next_arg.parse().unwrap();

                    }else{
                        interval=0;
                    }
                    i += 1;
            }
        }
        if arg == "-t" {
            if i + 1 < input.split(" ").count() {
                let next_arg = input.split(" ").nth(i + 1).unwrap();
                if !next_arg.starts_with('-') {
                    topic = next_arg;
                    i += 1;
                }
            }
        }
        i += 1;
    }
    if interval>=0 {interval+=1} else { interval-=1 }
    task_list.display_by_dates(interval, date.parse().unwrap(), done, not_done, long, topic);
}