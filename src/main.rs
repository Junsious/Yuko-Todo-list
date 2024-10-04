use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::Local;  // Для получения системного времени

// Файл, в который будут сохраняться задачи
const SAVE_FILE: &str = "tasks.json";

#[derive(Default, Serialize, Deserialize)]
struct TodoApp {
    tasks: Vec<Task>,               // Список задач
    new_task: String,               // Ввод новой задачи
    selected_task: Option<usize>,   // Редактируемая задача
    show_completed: bool,           // Флаг отображения выполненных задач
    search_query: String,           // Поисковый запрос
    theme: Theme,                   // Текущая тема (светлая/темная)
}

#[derive(Default, Serialize, Deserialize)]
struct Task {
    description: String, // Описание задачи
    completed: bool,     // Статус выполнения задачи
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

    // Подсчет количества выполненных задач
    fn completed_tasks(&self) -> usize {
        self.tasks.iter().filter(|task| task.completed).count()
    }

    // Процент выполнения задач
    fn progress(&self) -> f32 {
        if self.tasks.is_empty() {
            0.0
        } else {
            (self.completed_tasks() as f32 / self.tasks.len() as f32) * 100.0
        }
    }

    // Переключение темы
    fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
        self.save_tasks();
    }

    // Получение текущего времени в формате строки
    fn current_time() -> String {
        let now = Local::now();
        now.format("%H:%M:%S").to_string()  // Текущее время в формате ЧЧ:ММ:СС
    }

    // Фильтрация задач по поисковому запросу
    fn filtered_tasks(&self) -> Vec<(usize, &Task)> {
        self.tasks.iter()
            .enumerate()
            .filter(|(_, task)| {
                task.description
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
            })
            .collect()
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Установка темы интерфейса
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

            // Отображение текущего времени в правом верхнем углу
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

            // Поле для ввода новой задачи
            ui.vertical(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.new_task)
                    .hint_text("Enter a new task...")  // Подсказка
                    .desired_rows(3)                 // Количество строк
                    .desired_width(300.0)            // Ширина поля
                );

                // Кнопка добавления задачи
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

            // Поисковая строка
            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
            });

            // Флажок отображения выполненных задач
            ui.checkbox(&mut self.show_completed, "Show Completed Tasks");

            // Список задач с фильтрацией
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut to_remove = Vec::new();
                let mut edit_task = None;

                // Индексы задач для редактирования или удаления
                let task_indices: Vec<usize> = self.filtered_tasks()
                    .iter()
                    .filter(|(_, task)| self.show_completed || !task.completed)
                    .map(|(i, _)| *i)
                    .collect();

                // Отображение задач
                for i in task_indices {
                    let task = &mut self.tasks[i];
                    ui.horizontal(|ui| {
                        // Чекбокс выполнения задачи
                        let checkbox_response = ui.checkbox(&mut task.completed, "");
                        checkbox_response.changed(); // Отслеживаем изменения

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
                            edit_task = Some(i);
                        }

                        // Кнопка "Delete"
                        if ui.button("🗑").on_hover_text("Delete Task").clicked() {
                            to_remove.push(i);
                        }
                    });
                }

                // Удаление задач
                for index in to_remove.iter().rev() {
                    self.tasks.remove(*index);
                    self.save_tasks(); // Автосохранение
                }

                // Режим редактирования задачи
                if let Some(task_index) = edit_task {
                    self.selected_task = Some(task_index);
                }
            });

            // Кнопка для удаления выполненных задач
            if ui.button("Clear Completed").on_hover_text("Remove all completed tasks").clicked() {
                self.tasks.retain(|task| !task.completed);
                self.save_tasks(); // Автосохранение
            }

            // Кнопка для сохранения изменений
            if self.selected_task.is_some() {
                if ui.button("Save Changes").on_hover_text("Save task changes").clicked() {
                    self.selected_task = None;
                    self.save_tasks(); // Автосохранение
                }
            }
        });

        // Перерисовка интерфейса для обновления времени
        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let app = TodoApp::load_tasks(); // Загрузка задач при старте приложения

    eframe::run_native(
        "To-Do List",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}
