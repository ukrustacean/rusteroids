use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameState {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameState::DespawnEntities,
                InGameState::UserInput,
                InGameState::EntityUpdates,
                InGameState::CollisionDetection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameState::DespawnEntities)
                .before(InGameState::UserInput),
        );
    }
}