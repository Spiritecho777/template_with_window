#![cfg_attr(windows, windows_subsystem = "windows")] //Pour supprimer l'affichage de la console sur windows

mod basewindow;

use basewindow::BaseWindow;
use eframe::{egui,egui::{ViewportBuilder,Context,Color32},App,Frame,NativeOptions,};
use std::{rc::Rc,cell::RefCell,};
use image::GenericImageView;

#[cfg(target_os = "windows")]
use std::{sync::Arc};

/// Gestion de la fenêtre
pub struct Application {
    window_mod: BaseWindow,
    
    //Ajout pour LoadingBar
    progress: Rc<RefCell<f32>>,
    start_time: std::time::Instant,
}

impl Application {
    pub fn new() -> Self {
        let mut window_mod = BaseWindow::new();

        // Exemple : texte
        let text = Rc::new(RefCell::new(String::from("Hello")));
        window_mod.add_label(text.clone());

        // Exemple : bouton
        window_mod.add_button("Click", || {
            println!("Bouton cliqué !");
        });

        // Exemple : Checkbox
        let is_checked = Rc::new(RefCell::new(false));
        window_mod.add_checkbox("Activer l'option", is_checked.clone());

        // Exemple : Slider
        let slider = Rc::new(RefCell::new(50));
        window_mod.add_slider("Volume", slider.clone(), 0..=100);

        // Exemple : ComboBox
        let selected_item = Rc::new(RefCell::new(String::from("Option 1")));
        let options = vec![
            Rc::new(String::from("Option 1")),
            Rc::new(String::from("Option 2")),
            Rc::new(String::from("Option 3")),
        ];
        window_mod.add_combobox("Choisissez une option", selected_item.clone(), options);

        // Exemple : Textbox
        let textbox_filler = Rc::new(RefCell::new(String::from("")));
        window_mod.add_textbox(textbox_filler.clone());
        
        //Exemple : LoadingBar
        let progress = Rc::new(RefCell::new(0.0));
        window_mod.add_loading_bar(progress.clone()); // Voir dans update et dans struct

        Self { 
            window_mod,
            
            // Ajout pour loadingBar
            progress,
            start_time: std::time::Instant::now(),
        }
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        ctx.set_visuals(egui::Visuals{
            override_text_color: None,
            widgets: egui::style::Widgets::default(),
            //dark_mode: true,
            //panel_fill: egui::Color32::from_rgb(255, 255, 255), // Couleur du Background
            panel_fill:Color32::LIGHT_GRAY, // Couleur du Background
            ..Default::default()
        });
        self.window_mod.render(ctx);
        
        //Ajout pour loadingBar
        let elapsed = self.start_time.elapsed().as_secs_f32();
        {
            let mut p = self.progress.borrow_mut();
            *p = (elapsed / 5.0).min(1.0);
        }
        self.window_mod.render(ctx);
        if *self.progress.borrow() < 1.0 { ctx.request_repaint(); }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options= {
        #[cfg(target_os = "windows")]
        {
            let image_bytes = include_bytes!("../assets/icon.png");
            let image = image::load_from_memory(image_bytes).expect("Could not load image");
            let (width, height) = image.dimensions();
            let rgba = image.into_rgba8().into_raw();

            let icon = Arc::new(egui::IconData {
                rgba,
                width,
                height,
            });

            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_inner_size([800.0, 600.0]) // Taille de la fenetre
                    .with_icon(icon),
                ..Default::default()
            }
        }

        #[cfg(target_os = "linux")]
        {
            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_inner_size([800.0, 600.0]), // Taille de la fenetre
                ..Default::default()
            }
        }
    };
    
    eframe::run_native(
        "", // Titre de la fenetre
        options,
        Box::new(|_cc| Ok(Box::new(Application::new())))
    )?;

    Ok(())
}
