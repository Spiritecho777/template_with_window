use std::{cell::RefCell,rc::Rc};
use eframe::{egui, egui::{Color32, Context,}, Frame};
use crate::{basewindow::BaseWindow, windows::second_window::SecondWindow, app_manager::AppState};

/// Gestion de la fenêtre
pub struct FirstWindow {
    window_mod: BaseWindow,
    switch_state: Rc<RefCell<Option<AppState>>>,
    
    //Ajout pour LoadingBar
    progress: Rc<RefCell<f32>>,
    start_time: std::time::Instant,
}

impl FirstWindow{
    pub fn new() -> Self {
        let mut window_mod = BaseWindow::new();

        // Exemple : texte
        let text = Rc::new(RefCell::new(String::from("Hello")));
        window_mod.add_label(text.clone(),None);

        // Exemple : bouton
        window_mod.add_button("Click", || {
            println!("Le test du click");
        },None);

        // Exemple : Checkbox
        let is_checked = Rc::new(RefCell::new(false));
        window_mod.add_checkbox("Activer l'option", is_checked.clone(),None);

        // Exemple : Slider
        let slider = Rc::new(RefCell::new(50));
        window_mod.add_slider("Volume", slider.clone(), 0..=100,None);

        // Exemple : ComboBox
        let selected_item = Rc::new(RefCell::new(String::from("Option 1")));
        let options = vec![
            Rc::new(String::from("Option 1")),
            Rc::new(String::from("Option 2")),
            Rc::new(String::from("Option 3")),
        ];
        window_mod.add_combobox("Choisissez une option", selected_item.clone(), options,None);

        // Exemple : Textbox
        let textbox_filler = Rc::new(RefCell::new(String::from("")));
        window_mod.add_textbox(textbox_filler.clone(),None);

        // Exemple : LoadingBar
        let progress = Rc::new(RefCell::new(0.0));
        window_mod.add_loading_bar(progress.clone(),None); // Voir dans update et dans struct

        // Exemple : ajout d'image
        let image_path:&[u8] = include_bytes!("../../assets/ressources/test.png"); //changer le chemin selon l'image voulu 
        let texture = Rc::new(RefCell::new(None));
        window_mod.add_image_viewer(image_path, texture.clone(),200,200,None); //definir la taille de l'image

        // Changement de page grâce a AppManager
        let switch_state = Rc::new(RefCell::new(None));
        let switch_state_clone = switch_state.clone();
        window_mod.add_button("Aller à la 2e fenêtre", move || {
            *switch_state_clone.borrow_mut() = Some(AppState::Second(SecondWindow::new()));
        },None);
        
        Self {
            window_mod,
            switch_state,
            
            // Ajout pour loadingBar
            progress,
            start_time: std::time::Instant::now(),
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

        // Ajout pour loadingBar
        let elapsed = self.start_time.elapsed().as_secs_f32();
        {
            let mut p = self.progress.borrow_mut();
            *p = (elapsed / 5.0).min(1.0);
        }

        if *self.progress.borrow() < 1.0 {
            ctx.request_repaint();
        }
        
        self.window_mod.render(ctx); //pour l'affichage des control
        self.switch_state.borrow_mut().take() //pour AppManager
    }
}