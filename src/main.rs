use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use mlua::Lua;

use visual_script_stack_experiment::graph::{ui_draw_script_graph, GraphData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let lua = Lua::new();

    // let map_table = lua.create_table()?;
    // map_table.set(1, "one")?;
    // map_table.set("two", 2)?;

    // lua.globals().set("map_table", map_table)?;

    // lua.load(
    //     "function add(a, b) return a + b end
    //           print(add(3,4))",
    // )
    // .exec()?;

    // Ok(())
    //
    let app_window = Some(Window {
        title: "Visual Scripting Prototype (Bevy + Egui + Lua)".into(),
        ..default()
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: app_window,
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, ui_draw_script_graph)
        .init_resource::<GraphData>()
        .run();

    Ok(())
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
