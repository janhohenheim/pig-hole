use crate::loading::FontAssets;
use crate::loading::MenuAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct JoiningLobbyPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for JoiningLobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ViewModel>();
        app.add_system_set(
            SystemSet::on_update(GameState::JoiningLobby)
                .with_system(setup_menu)
                .with_system(click_play_button),
        );
    }
}

#[derive(Default)]
struct ViewModel {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,
}

fn setup_menu(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<ViewModel>,
    mut is_initialized: Local<bool>,
) {
    let egui_texture_handle = ui_state
        .egui_texture_handle
        .get_or_insert_with(|| {
            egui_ctx
                .ctx_mut()
                .load_texture("example-image", egui::ColorImage::example())
        })
        .clone();

    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    if !*is_initialized {
        *is_initialized = true;
    }

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::widgets::Image::new(
                egui_texture_handle.id(),
                egui_texture_handle.size_vec2(),
            ));

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.heading("Egui Template");
        ui.hyperlink("https://github.com/emilk/egui_template");
        ui.add(egui::github_link_file_line!(
            "https://github.com/mvlabat/bevy_egui/blob/main/",
            "Direct link to source code."
        ));
        egui::warn_if_debug_build(ui);

        ui.separator();

        ui.heading("Central Panel");
        ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
        ui.label("It is often a great place for big things, like drawings:");
    });

    egui::Window::new("Window")
        .vscroll(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
        });

    if invert {
        ui_state.inverted = !ui_state.inverted;
    }
}

#[allow(clippy::type_complexity)]
fn click_play_button(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (button, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Clicked {
            commands.entity(button).despawn_recursive();
            state.set(GameState::JoiningLobby).unwrap();
        }
    }
}
