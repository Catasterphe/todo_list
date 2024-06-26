#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui;
use eframe;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 700.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Aster's Tasklist",
        options,
        Box::new(|cc| {
            let tasks = eframe::get_value::<Vec<Task>>(cc.storage.unwrap(), "tasks_stored").unwrap_or_else(|| Vec::new());
            let app = MyApp {
                tasks: tasks,
                ..Default::default()
            };
            Box::new(app)
        }),
    )
}

struct MyApp {
    new_task_name: String,
    tasks: Vec<Task>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            new_task_name: String::new(),
            tasks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: impl Into<String>) -> Self {
        Task {
            name: name.into(),
            completed: false,
        }
    }
}

impl eframe::App for MyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.tasks.retain(|task| !task.completed);
        eframe::set_value(storage, "tasks_stored", &self.tasks);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!(
                "Aster's Tasklist - {} task(s)",
                self.tasks.len()
            ));
            ui.horizontal(|ui| {
                let new_task_label = ui.label("Task Name:");
                ui.text_edit_singleline(&mut self.new_task_name)
                    .labelled_by(new_task_label.id);
                if ui.button("New Task").clicked() && !self.new_task_name.is_empty() {
                    // take from the task name that way it empties and we get the task name at the same time 
                    self.tasks.push(Task::new(std::mem::take(&mut self.new_task_name)));
                }
            });
            for i in (0..self.tasks.len()).rev() {
                let task = &mut self.tasks[i];
                ui.horizontal(|ui| {
                    // add one to the label that way it starts at 1 and not 0.
                    ui.label((i + 1).to_string());
                    ui.checkbox(&mut task.completed, "");
                    ui.label(format!("{}", &task.name));
                });
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui: &mut egui::Ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.strong("Completed tasks delete upon closing.");
                    ui.hyperlink_to("Source code", "https://github.com/Catasterphe/todo_list");
                });
            });
        });
    }
}