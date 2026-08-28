use crate::backend::{Task, TaskQueue};
use crate::utils::input;

fn print_task(task: &Task) {
    println!(
        "[{}] {} - Priority: {} - {}",
        task.id,
        task.name,
        task.priority,
        if task.is_complete {
            "Complete"
        } else {
            "Incomplete"
        }
    );
}

fn handle_create(queue: &mut TaskQueue) {
    let task_name = input::next_str("Enter task name: ", "[Error] Invalid string.", true);

    let task_prio = input::next_int(
        "Enter priority integer: ",
        "[Error] Please enter a valid integer from 1 to 10.",
        true,
        1,
        10,
    );

    let id = queue.add_task(task_name, task_prio);

    println!("\n>>> Successfully created task #{}.", id);
}

fn handle_update(queue: &mut TaskQueue) {
    let id = input::next_int(
        "Enter an existing Task's ID: ",
        "[Error] Please enter a valid task ID.",
        true,
        1,
        9999999,
    ) as usize;

    println!("\n1. Change NAME\n2. Change PRIORITY\n>>>");

    let target = input::next_int(
        "Choose option: ",
        "[Error] Invalid choice. Either enter 1 or 2.",
        true,
        1,
        2,
    );

    let updated = match target {
        1 => {
            let new_name = input::next_str("Enter new name: ", "[Error] Invalid string.", true);

            queue.upd_task(id, Some(new_name), None)
        }

        2 => {
            let new_prio = input::next_int(
                "Enter a new priority level: ",
                "[Error] Please enter a number from 1 to 10.",
                true,
                1,
                10,
            );

            queue.upd_task(id, None, Some(new_prio))
        }

        _ => unreachable!(),
    };

    if updated {
        println!("\n>>> Successfully updated the task.");
    } else {
        println!("\n>>> Task ID not found.");
    }
}

fn handle_remove(queue: &mut TaskQueue) {
    let id = input::next_int(
        "Enter the Task's ID: ",
        "[Error] Please enter a valid task ID.",
        true,
        1,
        9999999,
    ) as usize;

    if queue.remove_task(id) {
        println!("\n>>> Successfully removed task #{}.", id);
    } else {
        println!("\n>>> Task ID not found.");
    }
}

fn handle_complete(queue: &mut TaskQueue) {
    let id = input::next_int(
        "Enter the Task's ID: ",
        "[Error] Please enter a valid task ID.",
        true,
        1,
        9999999,
    ) as usize;

    if queue.complete_task(id) {
        println!("\n>>> Successfully marked task #{} as complete.", id);
    } else {
        println!("\n>>> Task ID not found.");
    }
}

fn handle_list(queue: &TaskQueue) {
    println!("\n1. By ID\n2. By Priority\n>>>");

    let method = input::next_int(
        "Choose sorting method: ",
        "[Error] Invalid choice. Either enter 1 or 2.",
        true,
        1,
        2,
    );

    let descending = input::next_int(
        "1. Ascending\n2. Descending\n>>> ",
        "[Error] Invalid choice. Either enter 1 or 2.",
        true,
        1,
        2,
    ) == 2;

    let method = match method {
        1 => "id",
        2 => "priority",
        _ => unreachable!(),
    };

    let tasks = queue.list_tasks(method, descending);
    if tasks.is_empty() {
        println!("\nThe task queue is empty!");
        return;
    }

    println!("\n[ID] NAME - PRIORITY - STATUS");

    for task in tasks {
        print_task(task);
    }
}

pub fn run_loop() {
    let mut queue = TaskQueue::new();

    loop {
        println!(
            "\n|--- TASK MANAGER ---|\n\
            1. Add a Task\n\
            2. Dispatch Next Task\n\
            3. Remove a Task\n\
            4. List All Tasks\n\
            5. Update a Task\n\
            6. Complete a Task\n\
            7. Exit"
        );

        let choice = input::next_int(
            "Choose option: ",
            "[Error] Please select a valid number from 1 to 7.",
            true,
            1,
            7,
        );

        match choice {
            1 => handle_create(&mut queue),
            2 => match queue.dispatch_next() {
                Some(task) => {
                    println!("\n>>> Dispatched task:");
                    print_task(&task);
                }

                None => println!("\n>>> The task queue is empty!"),
            },
            3 => handle_remove(&mut queue),
            4 => handle_list(&queue),
            5 => handle_update(&mut queue),
            6 => handle_complete(&mut queue),
            7 => {
                println!("Exiting...");
                return;
            }
            _ => unreachable!(),
        }
    }
}
