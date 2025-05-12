#![cfg_attr(windows, windows_subsystem = "windows")] //Pour supprimer l'affichage de la console sur windows

mod basewindow;
mod windows;
mod app_manager;

use eframe::{egui,egui::{ViewportBuilder,},NativeOptions,};
use std::{sync::Arc};
use image::GenericImageView;
use crate::app_manager::AppManager;

//#[cfg(target_os = "windows")]

pub fn build_native_options() -> NativeOptions {
    //#[cfg(target_os = "windows")]
    //{
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
    //}

    /*#[cfg(target_os = "linux")]
    {
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([800.0, 600.0]), // Taille de la fenetre
            ..Default::default()
        }
    }*/
}

fn main()-> Result<(),eframe::Error> {
    let options = build_native_options();
    eframe::run_native(
        "Template de GUI",
        options,
        Box::new(|_cc| Ok(Box::new(AppManager::new()))),
    )
}
