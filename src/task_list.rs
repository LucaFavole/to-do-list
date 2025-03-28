use crate::{Task, Style, Term, Local, Write, NaiveDate, Duration, Result, Read, File, HashMap};

pub struct TaskList {
    pub tasks: HashMap<String, Task>,
}

impl TaskList{
    pub fn load()-> Result<TaskList>{
        let home_dir = dirs::home_dir();
        match  home_dir {
            None => {println!("errore"); Ok(TaskList{tasks: HashMap::new()})},
            Some(_) => {
                let file_path = home_dir.unwrap().to_str().unwrap().to_string() +"\\to-do-list\\tasks.json";
                let mut file = File::open(file_path)?;
                let mut data = String::new();
                file.read_to_string(&mut data)?;
                let tasks: HashMap<String, Task> = serde_json::from_str(&data)?;
                Ok(TaskList{tasks})

            }
        }

    }
    pub fn save(&self) -> Result<()>{
        let home_dir = dirs::home_dir();
        match home_dir{
            None => {println!("errore"); Ok(())},
            Some(_) => {
                println!("{:?}", home_dir);
                let mut file_path = home_dir.unwrap().to_str().unwrap().to_string() +"\\to-do-list";
                println!("{}", file_path);
                std::fs::create_dir_all(&file_path)?;
                file_path+= "\\tasks.json";
                let mut file = File::create(file_path)?;
                let data = serde_json::to_string(&self.tasks)?;
                file.write_all(data.as_bytes())?;
                Ok(())}
        }
    }
    pub fn add(&mut self, task: Task){
        self.tasks.insert(task.name.clone(), task);
    }
    pub fn modify_description(&mut self, name: &str, description: &str) {
        let task = self.tasks.get_mut(name);
        match task {
            Some(task) => {
                task.modify_description(description.to_string());
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }

    pub fn modify_deadline(&mut self, name: &str, deadline: &str) {
        let task = self.tasks.get_mut(name);
        match task {
            Some(task) => {
                task.modify_deadline(deadline.to_string());
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }
    pub fn complete(&mut self, name: &str){
        let task = self.tasks.get_mut(name);
        match task {
            Some(task) => {
                task.complete();
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }
    pub fn display(&self){
        for task in &self.tasks{
            task.1.display_short();
        }
    }
    pub fn display_priority(&self, priority: i32){
        let mut tasks = self.tasks.values().collect::<Vec<&Task>>();
        tasks.sort_by(|b, a| a.priority.cmp(&b.priority));
        for task in tasks{
            if task.priority >= priority{
                task.display_short();
            }
        }
    }
    pub fn display_long_priority(&self, priority: i32){
        let mut tasks = self.tasks.values().collect::<Vec<&Task>>();
        tasks.sort_by(|b, a| a.priority.cmp(&b.priority));
        for task in tasks{
            if task.priority >= priority{
                task.display_long();
            }
        }
    }
    pub fn display_long_task(&mut self, name: &str) {
        let task = self.tasks.get_mut(name);
        match task {
            Some(task) => {
                task.display_long();
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }

    pub fn remove(&mut self, name: &str) {
        let task = self.tasks.get_mut(name);
        match task {
            Some(_task) => {
                self.tasks.remove(name);
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }

    pub fn add_from_cmd(&mut self, name : &str, description: String, deadline: String){
        let task = self.tasks.get(name);
        if task.is_some(){
            let red_underlined = Style::new().red().underlined();
            Term::stdout().write_line(&red_underlined.apply_to("Task already exists").to_string()).unwrap();
            return;}
        let task = Task::new(name.parse().unwrap(), description, deadline.parse().unwrap());
        self.add(task);
    }
    pub fn display_long(&self){
        for task in &self.tasks{
            task.1.display_long()
        }
    }

    pub fn display_task_not_done_with_today_deadline(&self){
        let mut flag = false;
        Term::stdout().write_line("Task with today deadline not done:").unwrap();
        for task in &self.tasks{
            if task.1.deadline == Local::now().format("%d/%m/%Y").to_string() && !task.1.done{
                task.1.display_short();
                flag = true;
            }
        }
        if !flag {
            Term::stdout().write_line("No task with today deadline not done").unwrap();
        }
    }
    pub fn display_task_done_with_today_deadline(&self){
        let mut flag = false;
        Term::stdout().write_line("Task with today deadline done:").unwrap();
        for task in &self.tasks{
            if task.1.deadline == Local::now().format("%d/%m/%Y").to_string() && task.1.done{
                task.1.display_short();
                flag = true;
            }
        }
        if !flag {
            Term::stdout().write_line("No task with today deadline done").unwrap();
        }
    }
    pub fn display_task_not_done_with_past_deadline(&mut self){
        let mut flag = false;
        Term::stdout().write_line("Task with past deadline not done (you have to modify these):").unwrap();
        for task in &self.tasks{
            if compare_date(&task.1.deadline , Local::now().format("%d/%m/%Y").to_string())<0 && !task.1.done{
                task.1.display_short();
                flag = true;
    }}
        if !flag {
            Term::stdout().write_line("No task with past deadline not done").unwrap();
        }
    }
    /*
    pub fn display_task_not_done_with_past_deadline(&mut self){
        Term::stdout().write_line("Task with past deadline not done (you have to modify these):").unwrap();
        for task in &self.tasks{
            if compare_date(&task.deadline , Local::now().format("%d/%m/%Y").to_string())<0 && !task.done{
                task.display_long();
                io::stdout().flush().unwrap();
                Term::stdout().write_line("type remove (r) or modify-deadline(mdead)").unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                match input{
                    "remove" | "r" => {
                        self.remove(&task.name);
                        self.save().unwrap();
                    }
                    "modify-deadline" | "mdead" => {
                        Term::stdout().write_line("Insert new deadline:").unwrap();
                        let mut new_deadline = String::new();
                        io::stdin().read_line(&mut new_deadline).unwrap();
                        new_deadline = new_deadline.trim().to_string();
                        self.modify_deadline(&task.name, &new_deadline);
                        self.save().unwrap();
                    }
                    _ => {
                        Term::stdout().write_line("Default remove").unwrap();
                        self.remove(&task.name);
                        self.save().unwrap();
                    }
                }
            }
        }
    }*/
    pub fn remove_task_done_with_past_deadline(&mut self) {
        Term::stdout().write_line("Task removed:").unwrap();
        let mut index : Vec<String>= Vec::new();
        for task in &self.tasks {
            if compare_date(&task.1.deadline, Local::now().format("%d/%m/%Y").to_string()) < 0 && task.1.done && task.1.removable {
                task.1.display_short();
                index.insert(0, task.1.name.clone());
            }
        }
        if index.len() == 0{
            Term::stdout().write_line("No task removed").unwrap();
        }else{
            for i in index{
                self.remove(&i);
            }
        }
}
    pub fn add_topic(&mut self, name: &str, topic: &str) {
        let task = self.tasks.get_mut(name);
        match task {
            Some(task) => {
                task.add_topic(topic.to_string());
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }
    pub fn make_not_removable(&mut self, name: &str) {
        let task =self.tasks.get_mut(name);
        match task {
            Some(task) => {
                task.make_not_removable();
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }
    pub fn display_not_removable(&self) {
        for task in &self.tasks {
            if !task.1.removable {
                task.1.display_short();
            }
        }
    }
    pub fn display_all_topic(&self) {
        let mut topics: Vec<String> = Vec::new();
        for task in &self.tasks {
            if task.1.topic.is_some() {
                topics.push(task.1.topic.as_ref().unwrap().to_string());
            }
        }
        topics.sort();
        topics.dedup();
        for topic in topics {
            Term::stdout().write_line(&topic).unwrap();
        }
    }
    pub fn display_topic(&self, name: &str) {
        let mut flag = false;
        for task in &self.tasks {
            if task.1.topic.is_some() && task.1.topic.as_ref().unwrap() == name {
                task.1.display_short();
                flag = true;
            }
        }
        if !flag {
            Term::stdout().write_line("No task with this topic").unwrap();
        }

    }
    pub fn display_by_dates(&self, num: i32, date: String, done: bool, not_done: bool, long: bool, topic: &str) {
        let mut start_date =Local::now().date_naive();
        if date.len() == 10 {
            start_date = NaiveDate::parse_from_str(&*date, "%d/%m/%Y").unwrap();
        }
        if num>=0{
        for i in 0..num {
            let new_date = start_date + Duration::days(i as i64);
            self.display_by_date(new_date.format("%d/%m/%Y").to_string().as_ref(), done, not_done, long, topic);
        }}
        else{
            for i in 0..-num {
                let new_date = start_date - Duration::days(i as i64);
                self.display_by_date(new_date.format("%d/%m/%Y").to_string().as_ref(), done, not_done, long, topic);
            }
        }
    }
    pub(crate)  fn display_by_date(&self, date: &str, done: bool, not_done: bool, long: bool, topic: &str) {
        for task in &self.tasks {
            if topic.is_empty() || (task.1.is_some_topic() && topic == task.1.topic.as_ref().unwrap()) {
            if task.1.deadline.eq(date) {
                if done && task.1.done {
                    if long {
                        task.1.display_long();
                    } else {
                        task.1.display_short();
                    }
                }
                if not_done && !task.1.done {
                    if long {
                        task.1.display_long();
                    } else {
                        task.1.display_short();
                    }
                }

            }}
        }
    }
    pub(crate) fn add_priority(&mut self, name: &str, priority: &str) {
        let task = self.tasks.get_mut(name);
        match task {
            Some(task) => {
                if priority.parse::<i32>().is_ok(){
                    task.add_priority(priority.parse().unwrap());
                }else{
                    let red_underlined = Style::new().red().underlined();
                    Term::stdout().write_line(&red_underlined.apply_to("Invalid priority").to_string()).unwrap();
                }
            }
            None => {
                let red_underlined = Style::new().red().underlined();
                Term::stdout().write_line(&red_underlined.apply_to("Task not found").to_string()).unwrap();
            }
        }
    }
}

pub fn compare_date(dat1: &String, date2: String) -> i32 {
    let date1 = dat1.split("/").collect::<Vec<&str>>();
    let date2 = date2.split("/").collect::<Vec<&str>>();
    let day1 = date1[0].parse::<i32>().unwrap();
    let month1 = date1[1].parse::<i32>().unwrap();
    let year1 = date1[2].parse::<i32>().unwrap();
    let day2 = date2[0].parse::<i32>().unwrap();
    let month2 = date2[1].parse::<i32>().unwrap();
    let year2 = date2[2].parse::<i32>().unwrap();
    if year1 < year2 {
        -1
    } else if year1 > year2 {
        1
    } else {
        if month1 < month2 {
            -1
        } else if month1 > month2 {
            1
        } else {
            if day1 < day2 {
                -1
            } else if day1 > day2 {
                1
            } else {
                0
            }
        }
    }

}

