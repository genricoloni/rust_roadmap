//import for ToDoList
mod to_do_list;

fn main() {
    // --- FIRST PART ---
    println!("--- FIRST PART ---");
    println!("Test constructor and add_task");

    let mut tdl = to_do_list::ToDoList::new();

    // Add tasks
    tdl.add_task("Task3", 1).ok();
    tdl.add_task("Task1", 2).ok();
    tdl.add_task("Task2", 2).ok();
    tdl.add_task("Task5", 2).ok();
    tdl.add_task("Task4", 3).ok();

    // Print
    tdl.print();

    // --- SECOND PART ---
    println!("\n--- SECOND PART ---");
    println!("Test merge");

    let mut another_list = to_do_list::ToDoList::new();
    another_list.add_task("Task1", 1).ok();
    another_list.add_task("Task2", 2).ok();
    another_list.add_task("Task3", 3).ok();
    another_list.add_task("Task4", 4).ok();

    tdl.merge(&another_list);
    tdl.print();

    println!("\nTest mark_done");
    tdl.mark_done("Task1");
    tdl.mark_done("Task2");
    tdl.mark_done("Task2");
    tdl.print();

    println!("\nTest remove_done");
    tdl.remove_done();
    tdl.print();
}
