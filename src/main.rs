use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::Local;  // Добавлено для получения системного времени

// File where tasks will be saved
const SAVE_FILE: &str = "tasks.json";

#[derive(Default, Serialize, Deserialize)]
struct TodoApp {
    tasks: Vec<Task>,               // List of tasks
    new_task: String,               // New task input
    selected_task: Option<usize>,   // Task currently being edited
    show_completed: bool,           // Option to toggle showing completed tasks
    theme: Theme,                   // Light/Dark theme toggle
}

#[derive(Default, Serialize, Deserialize)]
struct Task {
    description: String, // Task description
    completed: bool,     // Task completion status
}

#[derive(Serialize, Deserialize, PartialEq)]
enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
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

    // Method to count completed tasks
    fn completed_tasks(&self) -> usize {
        self.tasks.iter().filter(|task| task.completed).count()
    }

    // Method to calculate task completion percentage
    fn progress(&self) -> f32 {
        if self.tasks.is_empty() {
            0.0
        } else {
            (self.completed_tasks() as f32 / self.tasks.len() as f32) * 100.0
        }
    }

    // Method to toggle the theme
    fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
        self.save_tasks();
    }

    // Method to get the current time as a formatted string
    fn current_time() -> String {
        let now = Local::now();
        now.format("%H:%M:%S").to_string()  // Получаем текущее время в формате ЧЧ:ММ:СС
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the theme
        ctx.set_style(egui::Style {
            visuals: match self.theme {
                Theme::Dark => egui::Visuals::dark(),
                Theme::Light => egui::Visuals::light(),
            },
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");
            ui.separator();

            // Отображение времени в правом верхнем углу
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(TodoApp::current_time());
            });

            // Theme toggle button
            if ui.button("Toggle Theme").clicked() {
                self.toggle_theme();
            }

            ui.horizontal(|ui| {
                // Progress bar
                let progress = self.progress() / 100.0;
                ui.label(format!("Progress: {:.2}%", self.progress()));
                ui.add(egui::ProgressBar::new(progress).desired_width(300.0));
            });
            
            ui.separator();

            // Field for entering a new task
            ui.vertical(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.new_task)
                    .hint_text("Enter a new task...")
                    .desired_rows(3)
                    .desired_width(300.0));

                if ui.button("Add Task").clicked() {
                    if !self.new_task.is_empty() {
                        self.tasks.push(Task {
                            description: self.new_task.clone(),
                            completed: false,
                        });
                        self.new_task.clear();
                        self.save_tasks(); // Auto-save
                    }
                }
            });

            ui.separator();

            // Show/Hide completed tasks toggle
            ui.checkbox(&mut self.show_completed, "Show Completed Tasks");

            // Task list
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut to_remove = Vec::new();

                for (i, task) in self.tasks.iter_mut().enumerate() {
                    if !self.show_completed && task.completed {
                        continue; // Skip completed tasks if not showing them
                    }

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut task.completed, "");

                        // Check if the task is being edited
                        if self.selected_task.map_or(false, |selected| selected == i) {
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut task.description)
                                        .desired_rows(3)
                                        .desired_width(300.0),
                                );
                            });
                        } else {
                            ui.add(
                                egui::TextEdit::multiline(&mut task.description)
                                    .desired_rows(3)
                                    .desired_width(300.0)
                                    .interactive(false),
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
                    self.save_tasks(); // Auto-save
                }
            });

            // "Clear Completed" button
            if ui.button("Clear Completed").clicked() {
                self.tasks.retain(|task| !task.completed);
                self.save_tasks(); // Auto-save
            }

            // "Save Changes" button when editing
            if self.selected_task.is_some() {
                if ui.button("Save Changes").clicked() {
                    self.selected_task = None;
                    self.save_tasks(); // Auto-save
                }
            }
        });

        // Запрашиваем перерисовку каждый кадр, чтобы время обновлялось
        ctx.request_repaint();
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
