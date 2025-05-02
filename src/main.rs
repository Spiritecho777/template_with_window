mod basewindow;

use basewindow::BaseWindow;
use eframe::egui::Context;
use eframe::{egui,App,Frame,};

// Gestion de la fenêtre
pub struct Application {
    window_mod: BaseWindow
}

// Ajout des contrôle 
impl Application {
    pub fn new() -> Self {
        let mut window_mod = BaseWindow::new();

        Self {
            window_mod
        }
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.window_mod.render(ctx);
    }
}
//fin gestion

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "", // Titre de la fenetre
        options,
        Box::new(|_cc| Ok(Box::new(Application::new())))
    );
    Ok(())
}
