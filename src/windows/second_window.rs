use std::{rc::Rc,cell::RefCell};
use eframe::{egui, egui::{Color32, Context,}, Frame};
use crate::{app_manager::AppState, basewindow::BaseWindow, windows::{first_window::FirstWindow,positionnement::Positionnement}};

pub struct SecondWindow {
    window_mod: BaseWindow,
    switch_state: Rc<RefCell<Option<AppState>>>,
}

impl SecondWindow {
    pub fn new() -> Self {
        let mut window_mod = BaseWindow::new();

        // Exemple : texte
        let text = Rc::new(RefCell::new(String::from("Eh oui sa marche")));
        window_mod.add_label(text.clone(),None);

        // Exemple : Textbox
        let textbox_filler = Rc::new(RefCell::new(String::from("test")));
        window_mod.add_textbox(textbox_filler.clone(),None);

        // Exemple : bouton
        window_mod.add_button("Click", || {
            println!("Le test du click");
        },None);

        // Changement de page grâce a AppManager
        let switch_state = Rc::new(RefCell::new(None));
        let switch_state_clone = switch_state.clone();
        window_mod.add_button("Positionnement", move || {
            *switch_state_clone.borrow_mut() = Some(AppState::Position(Positionnement::new()));
        },None);
        
        let switch_state_clone = switch_state.clone();
        window_mod.add_button("Retour", move || {
            *switch_state_clone.borrow_mut() = Some(AppState::First(FirstWindow::new()));
        },None);

        Self {
            window_mod,
            switch_state,
        }
    }

     pub fn update_with_switch(&mut self, ctx: &Context, _frame: &mut Frame) -> Option<AppState> {
         ctx.set_visuals(egui::Visuals{
             override_text_color: None,
             widgets: egui::style::Widgets::default(),
             //dark_mode: true,
             //panel_fill: egui::Color32::from_rgb(255, 255, 255), // Couleur du Background
             panel_fill:Color32::LIGHT_GRAY, // Couleur du Background
             ..Default::default()
         });

        self.window_mod.render(ctx); //pour l'affichage des control
        self.switch_state.borrow_mut().take() //pour AppManager
    }
}