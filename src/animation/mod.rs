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


            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_system(animation_systems::change_animation.label("ChangeAnimation"))
                    .with_system(animation_systems::animate_spritesheet.after("ChangeAnimation"))
                    .with_system(animation_systems::animate_simple_spritesheet)
            )
        ;
    }
}

//===============================================================