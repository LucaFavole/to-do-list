pub fn help(){
    println!("
    add <name> <deadline> <description> : Add a task with a name, a deadline and a description
    modify-deadline <name> <new deadline> : Modify the deadline of a task
    modify-description <name> <new description> : Modify the description of a task
    complete <name> : Mark a task as completed
    display : Display all tasks and other things
    display-long : Display all tasks with their description
    display-long-task <name> : Display a task with its description
    display-by-date <date> : Display all tasks with a deadline equal to the date and other things
    remove <name> : Remove a task
    add-topic <name> <topic> : Add a topic to a task
    make-not-removable <name> : Make a task not removable
    display-topic <topic> : Display all tasks with a topic
    display-all-topic : Display all topics
    display-not-removable : Display all tasks that are not removable
    quit : Quit the program
    help : Display this help
    clear : Clear the screen
    ");

}
pub fn help_add(){
    println!("
    add : Add a task with a name, a deadline and a description
    You can also use the alias 'a'
    If you want to add a task with a name 'name' (one word only), a deadline 'deadline' (dd/mm/yyyy) and a description 'description', you can type:
    'add name deadline description' or 'a name deadline description'
    If you don't provide a description then description = name
    If you don't provide a deadline then deadline = today
    If you don't want to provide the deadline like (dd/mm/yyyy) you can insert a number of days from today
    ");

}
pub fn help_modify_deadline(){
    println!("
    modify-deadline : Modify the deadline of a task
    You can also use the alias 'mdead'
    If you want to modify the deadline of a task with a name 'name' to a new deadline 'new_deadline' (dd/mm/yyyy), you can type:
    'modify-deadline name new_deadline' or 'mdead name new_deadline'
    ");

}
pub fn help_modify_description(){
    println!("
    modify-description : Modify the description of a task
    You can also use the alias 'mdesc'
    If you want to modify the description of a task with a name 'name' to a new description 'new_description', you can type:
    'modify-description name new_description' or 'mdesc name new_description'
    ");

}
pub fn help_complete(){
    println!("
    complete : Mark a task as completed
    You can also use the alias 'c'
    If you want to mark a task with a name 'name' as completed, you can type:
    'complete name' or 'c name'
    ");

}
pub fn help_display_long_task(){
    println!("
    display-long-task : Display a task with its description
    You can also use the alias 'dlt'
    If you want to display a task with a name 'name' with its description, you can type:
    'display-long-task name' or 'dlt name'
    ");

}
pub fn help_remove(){
    println!("
    remove : Remove a task
    You can also use the alias 'r'
    If you want to remove a task with a name 'name', you can type:
    'remove name' or 'r name'
    ");

}
pub fn help_add_topic(){
    println!("
    add-topic : Add a topic to a task
    You can also use the alias 'at'
    If you want to add a topic 'topic' to a task with a name 'name', you can type:
    'add-topic name topic' or 'at name topic'
    ");

}
pub fn help_make_not_removable(){
    println!("
    make-not-removable : Make a task not removable
    You can also use the alias 'mnr'
    If you want to make a task with a name 'name' not removable, you can type:
    'make-not-removable name' or 'mnr name'
    ");

}
pub fn help_display_topic(){
    println!("
    display-topic : Display all tasks with a topic
    You can also use the alias 'dt'
    If you want to display all tasks with a topic 'topic', you can type:
    'display-topic topic' or 'dt topic'
    ");

}
pub fn help_display_by_date(){
    println!("
    display-by-date : Display all tasks with a deadline equal to the date and other things
    You can also use the alias 'dbd'
    If you type only 'display-by-date' or 'dbd' you will see all tasks with a deadline equal to today and not done
    If you want to see all tasks with a deadline equal to a date 'date' and not done, you can use -d 'date':
    'display-by-date -d date' or 'dbd -d date'
    If you want to see all tasks with a deadline equal to a date 'date' and done, you can use -d 'date' -al:
    'dbd -d date -al' or 'dbd -al' (for today tasks)
     If you want to see all tasks with a topic 'topic', you can use -t 'topic':
    'display-by-date -t topic' or 'dbd -t topic'
    If you want to see all tasks with a deadline in an interval of date between today and a date 'date + days_number' , you can use -i 'days_number':
    'display-by-date -i days_number' or 'dbd -i days_number'
    You can combine all the options in the order you want
    ");

}
pub fn display_all_topic(){
    println!("
    display-all-topic : Display all topics
    You can also use the alias 'dat'
    If you want to display all topics, you can type:
    'display-all-topic' or 'dat'
    ")
}
pub fn help_display_not_removable(){
    println!("
    display-not-removable : Display all tasks that are not removable
    You can also use the alias 'dnr'
    If you want to display all tasks that are not removable, you can type:
    'display-not-removable' or 'dnr'
    ");

}
pub fn help_display(){
    println!("
    display : Display all tasks and other things
    You can also use the alias 'd'
    If you want to display all tasks and other things, you can type:
    'display' or 'd'
    you can use -p to display all tasks with a priority greater than 0 in order of priority
    or -p 'priority' to display all tasks with a priority greater or equal to 'priority' in order of priority
    you can use -l to display the long version of the tasks
    ");

}
pub fn help_display_long(){
    println!("
    display-long : Display all tasks with their description
    You can also use the alias 'dl'
    If you want to display all tasks with their description, you can type:
    'display-long' or 'dl'
    ");

}
pub fn help_quit(){
    println!("
    quit : Quit the program
    You can also use the alias 'q'
    If you want to quit the program, you can type:
    'quit' or 'q'
    ");

}
pub fn help_help(){
    println!("
    help : Display this help
    You can also use the alias 'h'
    If you want to display this help, you can type:
    'help' or 'h'
    ");

}
pub fn help_clear(){
    println!("
    clear : Clear the screen
    You can also use the alias 'cl'
    If you want to clear the screen, you can type:
    'clear' or 'cl'
    ");


}
pub fn help_display_all_topic(){
    println!("
    display-all-topic : Display all topics
    You can also use the alias 'dat'
    If you want to display all topics, you can type:
    'display-all-topic' or 'dat'
    ")
}
pub fn help_add_priority(){
    println!("
    add-priority : Add a priority to a task
    You can also use the alias 'ap'
    If you want to add a priority 'priority' to a task with a name 'name', you can type:
    'add-priority name priority' or 'ap name priority'
    ");

}
