use eframe::{egui::{Context}, App, Frame};
use crate::windows::{first_window::FirstWindow, second_window::SecondWindow,positionnement::Positionnement,};

pub enum AppState{
    First(FirstWindow),
    Second(SecondWindow),
    Position(Positionnement)
}

pub struct AppManager{
    pub state:AppState,
}

impl AppManager{
    pub fn new() -> Self{
        Self{
            state:AppState::First(FirstWindow::new())
        }
    }
}

impl App for AppManager {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        match &mut self.state {
            AppState::First(w) => {
                if let Some(next_state) = w.update_with_switch(ctx, frame) {
                    self.state = next_state;
                }
            }
            AppState::Second(w) => {
                if let Some(next_state) = w.update_with_switch(ctx, frame) {
                    self.state = next_state;
                }
            }
            AppState::Position(w) => {
                if let Some(next_state) = w.update_with_switch(ctx, frame) {
                    self.state = next_state;
                }
            }
        }
    }
}