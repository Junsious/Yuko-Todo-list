#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

// File where tasks will be saved
const SAVE_FILE: &str = "tasks.json";

#[derive(Default, Serialize, Deserialize)]
struct TodoApp {
    tasks: Vec<Task>,               // List of tasks
    new_task: String,               // New task
    selected_task: Option<usize>,   // Selected task for editing
}

#[derive(Default, Serialize, Deserialize)]
struct Task {
    description: String, // Task description
    completed: bool,     // Task completion status
}

impl TodoApp {
    // Method for loading tasks from a file
    fn load_tasks() -> Self {
        if let Ok(data) = fs::read_to_string(SAVE_FILE) {
            if let Ok(app) = serde_json::from_str::<TodoApp>(&data) {
                return app;
            }
        }
        TodoApp::default() // If reading fails, return an empty task list
    }

    // Method for saving tasks to a file
    fn save_tasks(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(SAVE_FILE, data); // Ignore the error when writing
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Setting the style
        ctx.set_style(egui::Style {
            visuals: egui::Visuals::dark(),
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");
            ui.separator();

            // Field for entering a new task (multiline input)
            ui.vertical(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.new_task)
                    .hint_text("Enter a new task...")
                    .desired_rows(3) // Set the desired number of rows
                    .desired_width(300.0));
                if ui.button("Add Task").clicked() {
                    if !self.new_task.is_empty() {
                        // Add a new task to the list
                        self.tasks.push(Task {
                            description: self.new_task.clone(),
                            completed: false,
                        });
                        self.new_task.clear();
                        self.save_tasks(); // Save tasks after adding
                    }
                }
            });

            ui.separator();

            // Task list
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| {
                    ui.label("Task list:");
                    let mut to_remove = Vec::new();

                    for (i, task) in self.tasks.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut task.completed, "");

                            // Check if the task is being edited
                            if self.selected_task.map_or(false, |selected| selected == i) {
                                ui.horizontal(|ui| {
                                    ui.label("✏️");
                                    ui.add(
                                        egui::TextEdit::multiline(&mut task.description)
                                            .desired_rows(3) // Editing the task in multiline format
                                            .desired_width(300.0),
                                    );
                                });
                            } else {
                                // Display the task in multiline format
                                ui.add(
                                    egui::TextEdit::multiline(&mut task.description)
                                        .desired_rows(3)  // Limit the number of rows
                                        .desired_width(300.0)
                                        .interactive(false), // Make the field read-only
                                );
                            }

                            // "Edit" button
                            if ui.button("Edit").clicked() {
                                self.selected_task = Some(i);
                            }

                            // "Delete" button
                            if ui.button("🗑").clicked() {
                                to_remove.push(i);
                            }
                        });
                    }

                    // Remove tasks after iteration
                    for index in to_remove.iter().rev() {
                        self.tasks.remove(*index);
                        self.save_tasks(); // Save after deleting
                    }
                });
            });

            // "Save Changes" button when editing
            if self.selected_task.is_some() {
                if ui.button("Save Changes").clicked() {
                    self.selected_task = None;
                    self.save_tasks(); // Save after editing
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let app = TodoApp::load_tasks(); // Load tasks at the start of the application

    eframe::run_native(
        "To-Do List",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}
