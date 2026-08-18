use godot::{
    classes::{Engine, ProjectSettings},
    prelude::*,
};

use crate::rpc::DiscordRPC;

pub mod rpc;

const SETTING_PATH: &str = "editor/godot_rpc/application_id";
const DEFAULT_APP_ID: &str = "1539256249219158066";

struct GodotRPC;

#[gdextension]
unsafe impl ExtensionLibrary for GodotRPC {
    fn on_stage_init(stage: InitStage) {
        if !Engine::singleton().is_editor_hint() {
            return;
        }

        if stage == InitStage::Editor {
            DiscordRPC::singleton().bind_mut().setup(&get_app_id());
        }
    }

    fn on_main_loop_frame() {
        if Engine::singleton().is_editor_hint() {
            DiscordRPC::singleton().bind_mut().update();
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        if !Engine::singleton().is_editor_hint() {
            return;
        }

        if stage == InitStage::Editor {
            DiscordRPC::singleton().bind_mut().close();
        }
    }
}

fn get_app_id() -> String {
    let mut settings = ProjectSettings::singleton();
    let path = GString::from(SETTING_PATH);

    if !settings.has_setting(&path) {
        settings.set_setting(&path, &Variant::from(DEFAULT_APP_ID));
        settings.set_initial_value(&path, &Variant::from(DEFAULT_APP_ID));
    }

    settings.get_setting(&path).to_string()
}
