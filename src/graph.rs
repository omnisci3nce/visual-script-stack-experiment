//! Node graph system + egui_snarl integration

use bevy::prelude::{ResMut, Resource};
use bevy_egui::EguiContexts;
use egui::{Color32, Ui};
use egui_snarl::{
    ui::{PinInfo, SnarlStyle, SnarlViewer},
    Snarl,
};
use serde::{Deserialize, Serialize};

use crate::scripting::{AddNode, FunctionNode};

const UNTYPED_COLOR: Color32 = Color32::from_rgb(0xb0, 0x00, 0x00);

#[derive(Clone)]
pub enum ScriptNode {
    Trigger,
    Sink,
    ConstNum(i32),
    Function(AddNode),
}

struct ScriptViewer;

impl SnarlViewer<ScriptNode> for ScriptViewer {
    fn title(&mut self, node: &ScriptNode) -> String {
        match node {
            ScriptNode::Trigger => "Trigger".into(),
            ScriptNode::ConstNum(i) => format!("Number: {i}"),
            ScriptNode::Function(func) => func.name().into(),
            ScriptNode::Sink => "Sink".into(),
        }
    }

    fn outputs(&mut self, node: &ScriptNode) -> usize {
        match node {
            ScriptNode::Trigger => 1,
            ScriptNode::ConstNum(_) => 1,
            ScriptNode::Function(func) => 1,
            ScriptNode::Sink => 0,
        }
    }

    fn inputs(&mut self, node: &ScriptNode) -> usize {
        match node {
            ScriptNode::Trigger => 1,
            ScriptNode::ConstNum(_) => 0,
            ScriptNode::Function(func) => 2,
            ScriptNode::Sink => 1,
        }
    }

    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<ScriptNode>,
    ) -> egui_snarl::ui::PinInfo {
        ui.label("Input");
        PinInfo::square()
            .with_fill(UNTYPED_COLOR)
            .with_stroke(egui::Stroke {
                width: 0.0,
                color: UNTYPED_COLOR,
            })
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut egui_snarl::Snarl<ScriptNode>,
    ) -> egui_snarl::ui::PinInfo {
        ui.label("Output");
        PinInfo::square()
            .with_fill(UNTYPED_COLOR)
            .with_stroke(egui::Stroke {
                width: 0.0,
                color: UNTYPED_COLOR,
            })
    }
}

#[derive(Resource)]
pub struct GraphData {
    snarl: Snarl<ScriptNode>,
    snarl_ui_id: Option<egui::Id>,
}

impl Default for GraphData {
    fn default() -> Self {
        let mut data = Self {
            snarl: Snarl::new(),
            snarl_ui_id: None,
        };

        data.snarl
            .insert_node(egui::pos2(2., 10.), ScriptNode::ConstNum(10));
        data.snarl
            .insert_node(egui::pos2(2., 30.), ScriptNode::ConstNum(20));
        data.snarl
            .insert_node(egui::pos2(30., 20.), ScriptNode::Function(AddNode));
        data.snarl
            .insert_node(egui::pos2(40., 20.), ScriptNode::Sink);

        data
    }
}

pub fn ui_draw_script_graph(mut contexts: EguiContexts, mut graph: ResMut<GraphData>) {
    egui::Window::new("Script").show(contexts.ctx_mut(), |ui| {
        ui.label("Script Graph goes here");
        graph.snarl_ui_id = Some(ui.id());

        let style = SnarlStyle::default();

        graph.snarl.show(&mut ScriptViewer, &style, "snarl", ui);

        // self.snarl.show(&mut DemoViewer, &self.style, "snarl", ui);
    });
}
