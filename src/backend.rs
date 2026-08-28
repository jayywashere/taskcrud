pub struct Task {
    pub id: usize,
    pub name: String,
    pub priority: i32,
    pub is_complete: bool,
}

pub struct TaskQueue {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, name: String, priority: i32) -> usize {
        let task = Task {
            id: self.next_id,
            name,
            priority,
            is_complete: false,
        };

        let id = task.id;
        self.tasks.push(task);
        self.next_id += 1;

        id
    }

    pub fn upd_task(
        &mut self,
        id: usize,
        new_name: Option<String>,
        new_priority: Option<i32>,
    ) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            if let Some(name) = new_name {
                task.name = name;
            }

            if let Some(priority) = new_priority {
                task.priority = priority;
            }

            return true;
        }

        false
    }

    pub fn remove_task(&mut self, id: usize) -> bool {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            return true;
        }

        false
    }

    pub fn dispatch_next(&mut self) -> Option<Task> {
        if self.tasks.is_empty() {
            return None;
        }

        let mut best_idx = 0;

        for i in 1..self.tasks.len() {
            if self.tasks[i].priority > self.tasks[best_idx].priority {
                best_idx = i;
            }
        }

        Some(self.tasks.remove(best_idx))
    }

    pub fn list_tasks(&self, method: &str, descending: bool) -> Vec<&Task> {
        let mut sorted_tasks: Vec<&Task> = self.tasks.iter().collect();

        match method.to_lowercase().as_str() {
            "priority" | "prio" | "p" => {
                if descending {
                    sorted_tasks.sort_by_key(|t| std::cmp::Reverse(t.priority))
                } else {
                    sorted_tasks.sort_by_key(|t| t.priority)
                }
            }

            "id" => {
                if descending {
                    sorted_tasks.sort_by_key(|t| std::cmp::Reverse(t.id))
                } else {
                    sorted_tasks.sort_by_key(|t| t.id)
                }
            }

            _ => unreachable!(),
        }

        sorted_tasks
    }

    pub fn complete_task(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.is_complete = true;
            return true;
        }

        false
    }
}
