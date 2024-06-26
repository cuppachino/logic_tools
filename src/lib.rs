use bevy::prelude::*;

pub mod logic;
pub mod systems;
pub mod components;
pub mod resources;
pub mod commands;
pub mod utils;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::logic::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::commands::prelude::*;
    pub use crate::utils::*;

    pub use super::{ LogicSimulationPlugin, LogicReflectPlugin };
}

/// A plugin group that adds all crate features to an [`App`].
#[derive(Default)]
pub struct LogicSimulationPlugin;

impl Plugin for LogicSimulationPlugin {
    fn build(&self, app: &mut App) {
        use prelude::*;

        app.add_plugins((LogicSchedulePlugin, LogicReflectPlugin, LogicGatePlugin))
            .insert_resource(Time::<LogicStep>::from_seconds(0.5))
            .init_resource::<LogicGraph>()
            .add_systems(
                LogicUpdate,
                (
                    systems::no_eval_output.in_set(LogicSystemSet::PropagateNoEval),
                    systems::step_logic.in_set(LogicSystemSet::StepLogic),
                ).chain()
            );
    }
}

/// A plugin that registers components' reflect data for a given [`App`].
pub struct LogicReflectPlugin;

impl Plugin for LogicReflectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Time<logic::schedule::LogicStep>>();

        app.register_type::<logic::signal::Signal>()
            .register_type::<components::Wire>()
            .register_type::<components::GateFan>()
            .register_type::<components::LogicGateFans>()
            .register_type::<resources::LogicGraph>();
    }
}
