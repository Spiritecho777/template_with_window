use eframe::egui::{Context, CentralPanel, Ui};
use std::rc::Rc;
use std::cell::RefCell;

pub type ComponentFn = Box<dyn FnMut(&Context, &mut Ui)>;

/// Fenêtre générique modulaire
pub struct BaseWindow {
    pub components: Vec<ComponentFn>,
}

impl BaseWindow {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn render(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            for component in self.components.iter_mut() {
                component(ctx, ui);
            }
        });
    }

    // Ajouter un bouton avec action mutuelle
    pub fn add_button<F>(&mut self, label: &str, mut on_click: F)
    where
        F: FnMut() + 'static,
    {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            if ui.button(&label).clicked() {
                on_click();
            }
        }));
    }

    // Ajouter un label lié à une donnée dynamique
    pub fn add_label(&mut self, text_ref: Rc<RefCell<String>>) {
        self.components.push(Box::new(move |_ctx, ui| {
            let text = text_ref.borrow();
            ui.label(&*text);
        }));
    }

    // Ajouter une image ASCII (simple label multi-lignes)
    pub fn add_ascii_image(&mut self, ascii_art: &'static str) {
        let art = ascii_art.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            ui.monospace(&art);
        }));
    }
}