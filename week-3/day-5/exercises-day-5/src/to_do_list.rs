#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    done: bool,
}

pub struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    /// Creates a new, empty ToDoList.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Adds a new task to the list.
    ///
    /// Returns an error if the description is longer than 40 characters or if the priority
    /// is not between 1 and 5 (inclusive). The new task is inserted so that the list remains
    /// sorted by ascending priority (for equal priority, insertion order is preserved).
    pub fn add_task(&mut self, description: &str, priority: u8) -> Result<(), &'static str> {
        if description.len() > 40 {
            return Err("Description is too long");
        }
        if priority < 1 || priority > 5 {
            return Err("Priority must be between 1 and 5");
        }
        let new_task = Task {
            description: description.to_string(),
            priority,
            done: false,
        };
        // Find the first task with a higher priority value (i.e., lower urgency)
        // and insert before it. If none is found, insert at the end.
        let pos = self
            .tasks
            .iter()
            .position(|task| task.priority > priority)
            .unwrap_or(self.tasks.len());
        self.tasks.insert(pos, new_task);
        Ok(())
    }

    /// Prints all tasks in the list.
    ///
    /// Each task is printed on a new line in the format:
    /// "[ ] 1 - Task description" for tasks not done, and
    /// "[X] 1 - Task description" for tasks that are done.
    pub fn print(&self) {
        for task in &self.tasks {
            let marker = if task.done { "[X]" } else { "[ ]" };
            println!("{} {} - {}", marker, task.priority, task.description);
        }
    }

    /// Merges another ToDoList into this one.
    ///
    /// Tasks from the other list are added with their original description and priority,
    /// but are marked as not done regardless of their state in the other list.
    pub fn merge(&mut self, other: &ToDoList) {
        for task in &other.tasks {
            // Ignore any error here; we assume tasks in the other list are valid.
            let _ = self.add_task(&task.description, task.priority);
        }
    }

    /// Marks the first not-done task with the given description as done.
    pub fn mark_done(&mut self, description: &str) {
        if let Some(task) = self
            .tasks
            .iter_mut()
            .find(|t| t.description == description && !t.done)
        {
            task.done = true;
        }
    }

    /// Removes all tasks that are marked as done.
    pub fn remove_done(&mut self) {
        self.tasks.retain(|task| !task.done);
    }
}
