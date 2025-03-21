struct Task {
    //40 characters
    description: String,
    priority: u8,
    done: bool,
}

pub(crate) struct ToDoList {
    list: Vec<Task>,
}

impl ToDoList {
    // Constructor
    pub fn new() -> Self {
        ToDoList { list: Vec::new() }
    }

    // Add a task to the list
    pub fn add_task(&mut self, description: &str, priority: u8) -> Result<(), &str> {
        if description.len() > 40 {
            return Err("Description is too long");
        }

        if priority < 1 || priority > 5 {
            return Err("Priority must be between 1 and 5");
        }

        let new_tasl = Task {
            description: description.to_string(),
            priority,
            done: false,
        };

        //insert the new task in the list by ascending order of priority
        let mut index = 0;
        for (i, task) in self.list.iter().enumerate() {
            if task.priority > priority {
                break;
            }
            index = i + 1;
        }
        self.list.insert(index, new_tasl);
        Ok(())
    }

    // Print the list
    pub fn print(&self) {
        for task in &self.list {
            let done = if task.done { "X" } else { " " };
            println!("{} [{}] {}", done, task.priority, task.description);
        }
    }

    // Merge another list into this one
    pub fn merge(&mut self, other: &ToDoList) {
        for task in &other.list {
            self.add_task(&task.description, task.priority).ok();
        }
    }

    // Mark a task as done
    pub fn mark_done(&mut self, description: &str) {
        for task in &mut self.list {
            if task.description == description && !task.done {
                task.done = true;
                break;
            }
        }
    }

    // Remove all done tasks
    pub fn remove_done(&mut self) {
        //self.list.retain(|task| !task.done);
        let mut i = 0;
        while i < self.list.len() {
            if self.list[i].done {
                self.list.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
