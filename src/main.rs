#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

// Файл, в который будут сохраняться задачи
const SAVE_FILE: &str = "tasks.json";

#[derive(Default, Serialize, Deserialize)]
struct TodoApp {
    tasks: Vec<Task>,               // Список задач
    new_task: String,               // Новая задача
    selected_task: Option<usize>,   // Выбранная задача для редактирования
}

#[derive(Default, Serialize, Deserialize)]
struct Task {
    description: String, // Описание задачи
    completed: bool,     // Статус завершенности задачи
}

impl TodoApp {
    // Метод для загрузки задач из файла
    fn load_tasks() -> Self {
        if let Ok(data) = fs::read_to_string(SAVE_FILE) {
            if let Ok(app) = serde_json::from_str::<TodoApp>(&data) {
                return app;
            }
        }
        TodoApp::default() // Если чтение не удалось, возвращаем пустой список задач
    }

    // Метод для сохранения задач в файл
    fn save_tasks(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(SAVE_FILE, data); // Игнорируем ошибку при записи
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Устанавливаем стиль
        ctx.set_style(egui::Style {
            visuals: egui::Visuals::dark(),
            ..Default::default()
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");
            ui.separator();

            // Поле для ввода новой задачи
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.new_task)
                    .hint_text("Введите новую задачу...")
                    .desired_width(300.0));
                if ui.button("Добавить задачу").clicked() {
                    if !self.new_task.is_empty() {
                        // Добавляем новую задачу в список
                        self.tasks.push(Task {
                            description: self.new_task.clone(),
                            completed: false,
                        });
                        self.new_task.clear();
                        self.save_tasks(); // Сохраняем задачи после добавления
                    }
                }
            });

            ui.separator();

            // Список задач
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.group(|ui| {
                    ui.label("Список задач:");
                    let mut to_remove = Vec::new();

                    for (i, task) in self.tasks.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut task.completed, "");

                            // Проверка, редактируется ли задача
                            if self.selected_task.map_or(false, |selected| selected == i) {
                                ui.horizontal(|ui| {
                                    ui.label("✏️");
                                    ui.text_edit_singleline(&mut task.description);
                                });
                            } else {
                                ui.label(&task.description);
                            }

                            // Кнопка "Редактировать"
                            if ui.button("Редактировать").clicked() {
                                self.selected_task = Some(i);
                            }

                            // Кнопка "Удалить"
                            if ui.button("🗑").clicked() {
                                to_remove.push(i);
                            }
                        });
                    }

                    // Удаление задач после итерации
                    for index in to_remove.iter().rev() {
                        self.tasks.remove(*index);
                        self.save_tasks(); // Сохраняем после удаления
                    }
                });
            });

            // Кнопка "Сохранить изменения" при редактировании
            if self.selected_task.is_some() {
                if ui.button("Сохранить изменения").clicked() {
                    self.selected_task = None;
                    self.save_tasks(); // Сохраняем после редактирования
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let app = TodoApp::load_tasks(); // Загружаем задачи при старте приложения

    eframe::run_native(
        "To-Do List",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}
