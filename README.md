# To-Do List Application in Rust

This is a simple command-line **To-Do List** application built using **Rust**. The application allows you to manage tasks with various features such as adding, modifying, displaying, and removing tasks.

## Features

Here are the available commands you can use to interact with the application:

### 1. **Add a Task**
   - **Command**: `add (a) <name> <deadline>`
   - **Description**: Add a new task with a name and deadline.
   - Example: `a "Buy groceries" "2025-03-30"`

### 2. **Modify the Deadline of a Task**
   - **Command**: `modify-deadline (mdead) <name> <new deadline>`
   - **Description**: Modify the deadline of an existing task.
   - Example: `mdead "Buy groceries" "2025-04-01"`

### 3. **Modify the Description of a Task**
   - **Command**: `modify-description (mdesc) <name> <new description>`
   - **Description**: Change the description of an existing task.
   - Example: `mdesc "Buy groceries" "Buy milk, eggs, and bread"`

### 4. **Add a Topic to a Task**
   - **Command**: `add-topic (at) <name> <topic>`
   - **Description**: Assign a topic to a task.
   - Example: `at "Buy groceries" "Shopping"`

### 5. **Display All Topics**
   - **Command**: `display-all-topic (dat)`
   - **Description**: Display all the topics available.
   - Example: `dat`

### 6. **Display Tasks by Topic**
   - **Command**: `display-topic (dt) <topic>`
   - **Description**: Display all tasks associated with a particular topic.
   - Example: `dt "Shopping"`

### 7. **Make a Task Not Removable**
   - **Command**: `make-not-removable (mnr) <name>`
   - **Description**: Mark a task as "not removable".
   - Example: `mnr "Buy groceries"`

### 8. **Display All Tasks That Are Not Removable**
   - **Command**: `display-not-removable (dnr)`
   - **Description**: Display all tasks that are marked as not removable.
   - Example: `dnr`

### 9. **Mark a Task as Done**
   - **Command**: `complete (c) <name>`
   - **Description**: Mark a task as completed.
   - Example: `c "Buy groceries"`

### 10. **Display All Tasks**
   - **Command**: `display (d)`
   - **Description**: Display a list of all tasks.
   - Example: `d`

### 11. **Display All Tasks with Descriptions**
   - **Command**: `display-long (dl)`
   - **Description**: Display all tasks along with their descriptions.
   - Example: `dl`

### 12. **Display a Task with Its Description**
   - **Command**: `display-long-task (dlt) <name>`
   - **Description**: Display a specific task along with its description.
   - Example: `dlt "Buy groceries"`

### 13. **Remove a Task**
   - **Command**: `remove (r) <name>`
   - **Description**: Remove a task from the list.
   - Example: `r "Buy groceries"`

### 14. **Quit the Application**
   - **Command**: `quit (q)`
   - **Description**: Quit the program.
   - Example: `q`

## Requirements

- Rust 1.x or higher
- A terminal or command prompt to run the application

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/LucaFavole/todolist.git
2. Navigate to the project directory:
   ```bash
   cd todolist-rust
3. Build the project:
   ```bash
   cargo build --release

4. Run the application:
   ```bash
   cargo run --release
   ```
5. aggiungi alla variabile d'ambiente (l'executable si trova in to-do-list/target/release)
   ```bash
   export PATH=$PATH:/path/to/your/executable

