use eframe::{egui,egui::{Context, CentralPanel, Ui, Slider, ComboBox, Button, Color32, Stroke, ProgressBar,ColorImage,TextureHandle,TextureOptions,Pos2}};
use std::{rc::Rc, cell::RefCell};

pub type ComponentFn = Box<dyn FnMut(&Context, &mut Ui)>;

/// Fenêtre générique modulaire
pub struct BaseWindow {
    pub flow_components: Vec<ComponentFn>,
    pub floating_components: Vec<(AnchorPosition,ComponentFn)>,
}

pub enum AnchorPosition{
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Custom(Pos2),
}

impl BaseWindow {
    pub fn new() -> Self {
        Self {
            flow_components: Vec::new(),
            floating_components:Vec::new(),
        }
    }

    pub fn render(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            for component in self.flow_components.iter_mut() {
                component(ctx, ui);
            }
        });

        // Rendu des composants positionnés
        let screen_rect = ctx.screen_rect();
        for (position, component) in self.floating_components.iter_mut() {
            let pos = match position {
                AnchorPosition::TopLeft => Pos2::new(10.0, 10.0),
                AnchorPosition::TopRight => Pos2::new(screen_rect.right() - 110.0, 10.0),
                AnchorPosition::BottomLeft => Pos2::new(10.0, screen_rect.bottom() - 30.0),
                AnchorPosition::BottomRight => Pos2::new(screen_rect.right() - 110.0, screen_rect.bottom() - 30.0),
                AnchorPosition::Center => Pos2::new(screen_rect.center().x - 50.0, screen_rect.center().y - 10.0),
                AnchorPosition::Custom(p) => *p,
            };

            egui::Area::new(egui::Id::new(format!("floating_{:?}", pos)))
                .fixed_pos(pos)
                .show(ctx, |ui| {
                    component(ctx, ui);
                });
        }
    }

    /// Bouton
    pub fn add_button<F>(&mut self, label: &str, mut on_click: F, position: Option<AnchorPosition>)
    where
        F: FnMut() + 'static,
    {
        let label = label.to_string();
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            let button = Button::new(&label)
                .fill(Color32::GRAY) // Couleur du Fond 
                .stroke(Stroke::new(2.0, Color32::GRAY)); // Couleur de la Bordure

            // Afficher le bouton avec les styles personnalisés
            if ui.add(button).clicked() {
                on_click();
            }
        });

        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
    }

    /// Ajouter un label lié à une donnée dynamique
    pub fn add_label(&mut self, text_ref: Rc<RefCell<String>>, position: Option<AnchorPosition>) {
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
            let text = text_ref.borrow();
            let rich_text = egui::RichText::new(&*text)
                .color(Color32::BLACK); //Couleur
                //.size(16.0); // Taille facultative
            
            ui.label(rich_text);
        });

        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
    }

    /// Textbox
    pub fn add_textbox(&mut self, text_ref: Rc<RefCell<String>>, position: Option<AnchorPosition>) {
        //self.components.push(Box::new(move |_ctx, ui| {
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
            let mut text = text_ref.borrow_mut();
    
            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
    
            ui.add_sized(
                [300.0, 20.0],
                egui::TextEdit::multiline(&mut *text)
                    .desired_rows(1)
                    .desired_width(f32::INFINITY)
            );
        //}));
        });

        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
}

    /// Checkbox
    pub fn add_checkbox(&mut self, label: &str, value_ref: Rc<RefCell<bool>>, position: Option<AnchorPosition>) {
        let label = label.to_string();
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
            let mut value = value_ref.borrow_mut();
            
            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            ui.checkbox(&mut *value, &label);
        });
        
        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
    }

    /// Slider
    pub fn add_slider(&mut self, label: &str, value_ref: Rc<RefCell<i32>>,range:std::ops::RangeInclusive<i32>, position: Option<AnchorPosition>) {
        let label = label.to_string();
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
            let mut value = value_ref.borrow_mut();

            let visuals = ui.visuals_mut();
            Self::apply_widget_style(visuals, Color32::GRAY, Color32::BLACK, Color32::GRAY);
            
            ui.add(Slider::new(&mut *value, range.clone()).text(&label));
        });
        
        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
    }
    
    /// Combobox
    pub fn add_combobox(&mut self, label: &str, selected: Rc<RefCell<String>>,options: Vec<Rc<String>>, position: Option<AnchorPosition>) {
        let label = label.to_string();
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
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
        });
        
        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
    }
    
    /// Barre de chargement
    pub fn add_loading_bar(&mut self,progress:Rc<RefCell<f32>>, position: Option<AnchorPosition>){
        let comp: ComponentFn = Box::new(move |_ctx, ui| {
            let progress_value = *progress.borrow();
            ui.add(
                ProgressBar::new(progress_value)
                    .text("Chargement...")
                    .fill(Color32::from_black_alpha(100))
            );
        });

        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
    }

    /// Ajout d'image
    pub fn add_image_viewer(&mut self,image_data: &'static[u8],texture:Rc<RefCell<Option<TextureHandle>>>,desired_width:u32,desired_height:u32, position: Option<AnchorPosition>){
        let comp: ComponentFn = Box::new(move |ctx, ui| {
            if texture.borrow().is_none() {
                if let Ok(image) = image::load_from_memory(image_data){
                    let image = image.resize(desired_width,desired_height,image::imageops::FilterType::Lanczos3);
                    let image = image.to_rgba8();
                    let (w,h) = image.dimensions();
                    let pixels = image.as_flat_samples();
                    let color_image = ColorImage::from_rgba_unmultiplied([w as usize,h as usize],pixels.as_slice());

                    let new_texture = ctx.load_texture("dynamic_image",color_image,TextureOptions::LINEAR);
                    *texture.borrow_mut() = Some(new_texture);
                }else{
                    ui.colored_label(Color32::RED, format!("Erreur : impossible de charger l'image",));
                }
            }
            if let Some(tex) = &*texture.borrow() {
                ui.image(tex);
            } else {
                ui.label("Aucune image à afficher");
            }
        });

        if let Some(pos) = position {
            self.floating_components.push((pos, comp));
        } else {
            self.flow_components.push(comp);
        }
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