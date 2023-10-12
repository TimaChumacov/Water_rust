#![windows_subsystem = "windows"]

mod simulation;

use egui::Vec2;
use eframe::egui;
use crate::simulation::*;

pub struct WaterRooms {
    simulation_state: SimulationState
}

impl eframe::App for WaterRooms {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        on_simulation_update(&mut self.simulation_state);

        let grid_text =
            egui::RichText::new(format!("{}", self.simulation_state.grid_str)).monospace();

        let can_generate = self.simulation_state.can_generate;

        let generate_button_title = match can_generate {
            true => "generate layout",
            false => "can't generate",
        };

        let generate_button_text =
            egui::RichText::new(format!("{}", generate_button_title)).monospace();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(grid_text);
            ui.horizontal(|ui| {
                if ui.button(generate_button_text).clicked() && can_generate {
                    on_generate_layout(&mut self.simulation_state);
                }
                if ui.button("water ON/OFF").clicked() {
                    on_water_onoff(&mut self.simulation_state);
                }
                if ui.button("reset layout").clicked() {
                    self.simulation_state = SimulationState::default();
                }
                ui.label(egui::RichText::new("👇 expand the window! 👇").monospace());
            });

            ui.label("\nHi! This rust apps lets you generate interconnected \
            rooms and then see how they are filled up with water. \"generate layout\" button \
            can be pressed multiple times and it generates a room each time. \
            If there's little space left, the button will be disabled. \"water ON/OFF\" button \
            makes the layout prettier and starts the waterflow. \
            You can press again to to stop the water. \
            You can't generate more rooms after pouring water. \"reset layout\" button \
            resets everything.");
        });
        ctx.request_repaint(); // update the window even if the user doesn't move the cursor
    }
}

fn main() -> eframe::Result<()> {
    let default_options = eframe::NativeOptions::default();

    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(530 as f32, 680 as f32)),
        min_window_size: Option::from(Vec2::new(530 as f32, 680 as f32)),
        max_window_size: Option::from(
            // max size bigger vertically so you could see the description
            Vec2::new(530 as f32, 820 as f32)
        ),
        resizable: true,
        transparent: true,
        mouse_passthrough: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: default_options.hardware_acceleration,
        renderer: default_options.renderer,
        follow_system_theme: true,
        default_theme: default_options.default_theme,
        run_and_return: true,
        event_loop_builder: default_options.event_loop_builder,
        shader_version: default_options.shader_version,
        centered: false,
    };

    let initial_state = WaterRooms { simulation_state: SimulationState::default() };
    eframe::run_native("Water Rooms", options, Box::new(|_cc| Box::<WaterRooms>::new(initial_state)))
}

// KNOWN BUGS:
// - rooms can generate diagonally really closely but without a path to each other
// - by spamming ON/OFF water can sometimes pile up like sand. Happens only in very specific corners.
