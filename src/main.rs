use eframe::egui;

#[derive(Default)]
struct TodoApp {
    tasks: Vec<Task>,               // List of tasks
    new_task: String,               // New task to be added
    selected_task: Option<usize>,   // Selected task for editing
}

#[derive(Default)]
struct Task {
    description: String, // Task description
    completed: bool,     // Task completion status
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the style
        ctx.set_style(egui::Style {
            visuals: egui::Visuals::dark(), // Use a dark theme
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");
            ui.separator();

            // Input field for a new task
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.new_task)
                    .hint_text("Enter a new task...") // Use hint_text for a prompt
                    .desired_width(300.0)); // Set width
                if ui.button("Add Task").clicked() {
                    if !self.new_task.is_empty() {
                        // Add a new task to the list
                        self.tasks.push(Task {
                            description: self.new_task.clone(),
                            completed: false,
                        });
                        self.new_task.clear();
                    }
                }
            });

            ui.separator();

            // Task list
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| { // Group for the task list
                    ui.label("Task List:");
                    let mut to_remove = Vec::new(); // Vector to store task indices for removal

                    for (i, task) in self.tasks.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut task.completed, ""); // Checkbox for task completion

                            // Check if the task is being edited
                            if self.selected_task.map_or(false, |selected| selected == i) {
                                // Highlight the task being edited
                                ui.horizontal(|ui| {
                                    ui.label("✏️"); // Edit icon
                                    ui.text_edit_singleline(&mut task.description); // Edit task
                                });
                            } else {
                                ui.label(&task.description); // Display task
                            }

                            // "Edit" button
                            if ui.button("Edit").clicked() {
                                self.selected_task = Some(i);
                            }

                            // "Delete" button
                            if ui.button("🗑").clicked() { // Delete icon
                                to_remove.push(i); // Add the task index to the removal list
                            }
                        });
                    }

                    // Remove tasks after iterating
                    for index in to_remove.iter().rev() {
                        self.tasks.remove(*index);
                    }
                });
            });

            // "Save" button for task changes
            if self.selected_task.is_some() {
                if ui.button("Save Changes").clicked() {
                    self.selected_task = None; // Finish editing
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do List",
        options,
        Box::new(|_cc| Ok(Box::new(TodoApp::default()))),
    )
    .unwrap();
}
