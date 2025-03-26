use crate::{parse_command, Command, TaskList, Style, Term, Write, stdin, stdout, handle_add, handle_modify_deadline, handle_modify_description, handle_complete, handle_display_long_task, handle_remove, handle_add_topic, handle_make_not_removable, handle_display_topic, handle_display_by_date, display_help, display_error};
pub fn handle_input(){
    let mut task_list = TaskList::load().unwrap_or_else(|_| TaskList { tasks: Default::default() });
    let term = Term::stdout();
    task_list.remove_task_done_with_past_deadline();
    task_list.display_task_not_done_with_today_deadline();
    task_list.display_task_not_done_with_past_deadline();
    loop {
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }
        let first = input.split(" ").next().unwrap().to_ascii_lowercase();
        match parse_command(&*first) {
            Command::Add => handle_add(&mut task_list, &*input),
            Command::ModifyDeadline => handle_modify_deadline(&mut task_list, &*input),
            Command::ModifyDescription => handle_modify_description(&mut task_list, &*input),
            Command::Complete => handle_complete(&mut task_list, &*input),
            Command::Display => task_list.display(),
            Command::DisplayLongTask => handle_display_long_task(&mut task_list, &*input),
            Command::DisplayLong => task_list.display_long(),
            Command::Remove => handle_remove(&mut task_list, &*input),
            Command::Quit => break,
            Command::Help => display_help(&term),
            Command::Error => display_error(&term),
            Command::AddTopic => handle_add_topic(&mut task_list, &*input),
            Command::MakeNotRemovable => handle_make_not_removable(&mut task_list, &*input),
            Command::DisplayNotRemovable => task_list.display_not_removable(),
            Command::DisplayAllTopic => task_list.display_all_topic(),
            Command::DisplayTopic => handle_display_topic(&mut task_list, &*input),
            Command::Clear => term.clear_screen().unwrap(),
            Command::DisplayByDate => handle_display_by_date(&mut task_list, &*input),

        }


    }
}

