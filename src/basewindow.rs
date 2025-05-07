use eframe::{egui,egui::{Context, CentralPanel, Ui, Slider, ComboBox, Button, Color32, Stroke, ProgressBar,ColorImage,TextureHandle,TextureOptions}};
use std::{rc::Rc, cell::RefCell};

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
            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            let button = Button::new(&label)
                .fill(Color32::GRAY) // Couleur du Fond 
                .stroke(Stroke::new(2.0, Color32::GRAY)); // Couleur de la Bordure

            // Afficher le bouton avec les styles personnalisés
            if ui.add(button).clicked() {
                on_click();
            }
        }));
    }

    /// Ajouter un label lié à une donnée dynamique
    pub fn add_label(&mut self, text_ref: Rc<RefCell<String>>) {
        self.components.push(Box::new(move |_ctx, ui| {
            let text = text_ref.borrow();

            let rich_text = egui::RichText::new(&*text)
                .color(Color32::BLACK); //Couleur
                //.size(16.0); // Taille facultative
            
            ui.label(rich_text);
        }));
    }

    /// Textbox
    pub fn add_textbox(&mut self, text_ref: Rc<RefCell<String>>) {
        self.components.push(Box::new(move |_ctx, ui| {
            let mut text = text_ref.borrow_mut();

            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            ui.add_sized(
                [300.0, 20.0],
                egui::TextEdit::multiline(&mut *text)
                    .desired_rows(1)
                    .desired_width(f32::INFINITY)
            );
        }));
    }

    /// Checkbox
    pub fn add_checkbox(&mut self, label: &str, value_ref: Rc<RefCell<bool>>) {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            let mut value = value_ref.borrow_mut();

            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            ui.checkbox(&mut *value, &label);
        }));
    }

    /// Slider
    pub fn add_slider(&mut self, label: &str, value_ref: Rc<RefCell<i32>>,range:std::ops::RangeInclusive<i32>) {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            let mut value = value_ref.borrow_mut();

            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            ui.add(Slider::new(&mut *value, range.clone()).text(&label));
        }));
    }
    
    /// Combobox
    pub fn add_combobox(&mut self, label: &str, selected: Rc<RefCell<String>>,options: Vec<Rc<String>>) {
        let label = label.to_string();
        self.components.push(Box::new(move |_ctx, ui| {
            let mut selected_val = selected.borrow_mut();
            
            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
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
    
    /// Barre de chargement
    pub fn add_loading_bar(&mut self,progress:Rc<RefCell<f32>>){
        self.components.push(Box::new(move |_ctx, ui| {
            let progress_value = *progress.borrow();
            ui.add(
                ProgressBar::new(progress_value)
                    .text("Chargement...")
                    .fill(Color32::from_black_alpha(100))
            );
        }))
    }

    /// Ajout d'image
    pub fn add_image_viewer(&mut self,image_path: Rc<RefCell<String>>,texture:Rc<RefCell<Option<TextureHandle>>>,desired_width:u32,desired_height:u32){
    //pub fn add_image_viewer(&mut self,image_path: Option<String>,texture:Rc<RefCell<Option<TextureHandle>>>,desired_width:u32,desired_height:u32){
        self.components.push(Box::new(move |ctx, ui| {
            if texture.borrow().is_none() {
                let path = image_path.borrow().clone();
                if let Ok(mut image) = image::open(&path) {
                    image = image.resize(desired_width,desired_height,image::imageops::FilterType::Lanczos3);
                    let image = image.to_rgba8();
                    let (w,h) = image.dimensions();
                    let pixels = image.as_flat_samples();
                    let color_image = ColorImage::from_rgba_unmultiplied([w as usize,h as usize],pixels.as_slice());

                    let new_texture = ctx.load_texture("dynamic_image",color_image,TextureOptions::LINEAR);
                    *texture.borrow_mut() = Some(new_texture);
                }else{
                    ui.colored_label(Color32::RED, format!("Erreur : impossible de charger {}", path));
                }
            }
            if let Some(tex) = &*texture.borrow() {
                ui.image(tex);
            } else {
                ui.label("Aucune image à afficher");
            }

            /*if texture.borrow().is_none() {
                let image_bytes: Vec<u8>;  // Contient soit les bytes de l'image par défaut, soit ceux d'une image personnalisée

                // Si une image personnalisée est passée, on l'utilise. Sinon, on charge l'image par défaut du binaire
                if let Some(image_path) = &image_path {
                    // Charger l'image depuis le fichier passé en paramètre
                    let path = std::path::Path::new(&image_path);
                    match image::open(path) {
                        Ok(img) => {
                            image_bytes = img.to_rgba8().into_raw();
                        }
                        Err(_) => {
                            ui.colored_label(egui::Color32::RED, "Erreur : impossible de charger l'image personnalisée.");
                            return;
                        }
                    }
                } else {
                    // Intégration de l'image par défaut dans le binaire via `include_bytes!`
                    image_bytes = include_bytes!("../assets/ressources/test.png").to_vec();
                }

                // Charge l'image depuis les bytes
                let image = match image::load_from_memory(&image_bytes) {
                    Ok(mut img) => {
                        img = img.resize(desired_width, desired_height, image::imageops::FilterType::Lanczos3);
                        img.to_rgba8()
                    }
                    Err(_) => {
                        ui.colored_label(egui::Color32::RED, "Erreur : impossible de charger l'image.");
                        return;
                    }
                };

                // Créer la texture
                let (w, h) = image.dimensions();
                let pixels = image.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied([w as usize, h as usize], pixels.as_slice());
                let new_texture = ctx.load_texture("dynamic_image", color_image, TextureOptions::LINEAR);
                *texture.borrow_mut() = Some(new_texture);
            }

            // Affiche l'image
            if let Some(tex) = &*texture.borrow() {
                let size = egui::Vec2::new(desired_width as f32, desired_height as f32);
                ui.allocate_space(size);
                ui.image(tex);
            } else {
                ui.label("Aucune image à afficher");
            }*/
        }));
    }

    /// Personalisation ui
    fn apply_widget_style(visuals: &mut egui::Visuals, background: Color32, foreground: Color32, border: Color32) {
        use egui::Stroke;

        let mut style = visuals.widgets.inactive.clone();

        style.bg_fill = background;
        style.bg_stroke = Stroke::new(10.0, border);
        style.fg_stroke = Stroke::new(1.0, foreground);
        
        visuals.extreme_bg_color = background;
        visuals.widgets.inactive = style.clone();
        visuals.widgets.hovered = style.clone();
        visuals.widgets.open = style.clone();
        //visuals.widgets.active = style.clone();
    } 
}
