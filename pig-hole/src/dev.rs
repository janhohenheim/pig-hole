use bevy::prelude::*;

#[cfg(feature = "dev")]
use bevy_editor_pls::prelude::*;


#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;

/// Plugin with debugging utility intendend for use during development only.
/// Will not do anything when used in a release build.
pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        {
            app.add_plugin(EditorPlugin);
        }
        #[cfg(debug_assertions)]
        {
            app //.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
