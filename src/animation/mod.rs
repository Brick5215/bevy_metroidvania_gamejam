//===============================================================

use bevy::prelude::*;

pub mod animation_components;
mod animation_systems;

//===============================================================

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_event::<animation_components::ChangeAnimationEvent>()
            .add_event::<animation_components::AnimationFinishedEvent>()

            .add_system_to_stage(CoreStage::PreUpdate, animation_systems::set_sprite_auto)

            .add_system_to_stage(CoreStage::PostUpdate, 
                animation_systems::change_animation
                .label("ChangeAnimation")
            )
            .add_system_to_stage(CoreStage::PostUpdate, 
                animation_systems::animate_spritesheet
                .after("ChangeAnimation")
            )
        ;
    }
}

//===============================================================