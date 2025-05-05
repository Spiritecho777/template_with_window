use eframe::egui::{Context, CentralPanel, Ui, Slider, ComboBox, Checkbox};
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

    /// Bouton
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

    /// Ajouter un label lié à une donnée dynamique
    pub fn add_label(&mut self, text_ref: Rc<RefCell<String>>) {
        self.components.push(Box::new(move |_ctx, ui| {
            let text = text_ref.borrow();
            ui.label(&*text);
        }));
    }

    /// Ajouter une image ASCII (simple label multi-lignes)
    pub fn add_ascii_image(&mut self, ascii_art: &'static str) {
        let art = ascii_art.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            ui.monospace(&art);
        }));
    }

    /// Textbox
    pub fn add_textbox(&mut self, text_ref: Rc<RefCell<String>>) {
        self.components.push(Box::new(move |_ctx, ui| {
            let mut text = text_ref.borrow_mut();
            ui.text_edit_multiline(&mut *text);
        }));
    }

    /// Checkbox
    pub fn add_checkbox(&mut self, label: &str, value_ref: Rc<RefCell<bool>>) {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            let mut value = value_ref.borrow_mut();
            ui.checkbox(&mut *value, &label);
        }));
    }

    /// Slider
    pub fn add_slider(&mut self, label: &str, value_ref: Rc<RefCell<i32>>,range:std::ops::RangeInclusive<i32>) {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            let mut value = value_ref.borrow_mut();
            ui.add(Slider::new(&mut *value, range.clone()).text(&label));
        }));
    }
    
    /// Combobox
    pub fn add_combobox(&mut self, label: &str, selected: Rc<RefCell<String>>,options: Vec<Rc<String>>) {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            let mut selected_val = selected.borrow_mut();
            ComboBox::from_label(&label)
                .selected_text(&*selected_val)
                .show_ui(ui, |ui| {
                    for opt in &options {
                        let opt_str = &**opt;
                        if ui.selectable_label(&*selected_val == opt_str, opt_str).clicked() {
                            *selected_val = opt_str.to_string();
                        }
                    }
                });
        }))
    }
}
