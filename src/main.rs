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
        // Устанавливаем тему
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

            // Кнопка для смены темы
            if ui.button("Toggle Theme").clicked() {
                self.toggle_theme();
            }

            ui.horizontal(|ui| {
                // Полоса прогресса с анимацией
                let progress = self.progress() / 100.0;
                ui.label(format!("Progress: {:.2}%", self.progress()));
                ui.add(egui::ProgressBar::new(progress)
                    .animate(true)  // Включаем анимацию
                    .desired_width(300.0)
                );
            });

            ui.separator();

            // Поле ввода новой задачи с многострочным вводом
            ui.vertical(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.new_task)
                    .hint_text("Enter a new task...") // Подсказка
                    .desired_rows(3)                 // Три строки для ввода
                    .desired_width(300.0)             // Ширина поля ввода
                );

                // Кнопка "Add Task" рядом с многострочным вводом
                if ui.button("Add Task").clicked() {
                    if !self.new_task.is_empty() {
                        self.tasks.push(Task {
                            description: self.new_task.clone(),
                            completed: false,
                        });
                        self.new_task.clear();
                        self.save_tasks(); // Автосохранение
                    }
                }
            });

            ui.separator();

            // Флажок для отображения выполненных задач
            ui.checkbox(&mut self.show_completed, "Show Completed Tasks");

            // Список задач
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut to_remove = Vec::new();

                for (i, task) in self.tasks.iter_mut().enumerate() {
                    if !self.show_completed && task.completed {
                        continue; // Пропускаем выполненные задачи, если они не должны отображаться
                    }

                    ui.horizontal(|ui| {
                        // Чекбокс выполнения задачи
                        ui.checkbox(&mut task.completed, "");

                        // Если задача редактируется
                        if self.selected_task.map_or(false, |selected| selected == i) {
                            ui.add(egui::TextEdit::multiline(&mut task.description)
                                .desired_rows(3)
                                .desired_width(300.0),
                            );
                        } else {
                            let style = if task.completed {
                                egui::TextEdit::multiline(&mut task.description)
                                    .interactive(false)
                                    .desired_width(300.0)
                                    .text_color(egui::Color32::from_gray(120))  // Серый цвет для выполненных задач
                            } else {
                                egui::TextEdit::multiline(&mut task.description)
                                    .interactive(false)
                                    .desired_width(300.0)
                            };
                            ui.add(style);
                        }

                        // Кнопка "Edit"
                        if ui.button("✏️").on_hover_text("Edit Task").clicked() {
                            self.selected_task = Some(i);
                        }

                        // Кнопка "Delete" с подсказкой
                        if ui.button("🗑").on_hover_text("Delete Task").clicked() {
                            to_remove.push(i);
                        }
                    });
                }

                // Удаляем задачи после итерации
                for index in to_remove.iter().rev() {
                    self.tasks.remove(*index);
                    self.save_tasks(); // Автосохранение
                }
            });

            // Кнопка "Clear Completed"
            if ui.button("Clear Completed").on_hover_text("Remove all completed tasks").clicked() {
                self.tasks.retain(|task| !task.completed);
                self.save_tasks(); // Автосохранение
            }

            // Кнопка "Save Changes" при редактировании
            if self.selected_task.is_some() {
                if ui.button("Save Changes").on_hover_text("Save task changes").clicked() {
                    self.selected_task = None;
                    self.save_tasks(); // Автосохранение
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
