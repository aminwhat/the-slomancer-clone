use bevy::{
    app::{App, Update},
    ecs::{schedule::IntoScheduleConfigs, system::Res},
    prelude::{AppExtStates, Commands, OnEnter, Query, Resource, States, in_state},
    state::app::StatesPlugin,
};
use core::prelude::{
    AsPhysicsSystem, ErasedGd, ErasedGdResource, GodotScene, SystemDeltaTimer, bevy_app,
};
use godot::{
    builtin::{Transform2D, Vector2},
    classes::{ResourceLoader, Sprite2D},
};
use godot::{init::ExtensionLibrary, prelude::gdextension};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
}

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .init_resource::<MyAssets>()
        .add_systems(OnEnter(GameState::Playing), spawn_sprite)
        .add_systems(
            Update,
            move_sprite
                .as_physics_system()
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Resource, Debug)]
pub struct MyAssets {
    pub sprite: ErasedGdResource,
}

impl Default for MyAssets {
    fn default() -> Self {
        let mut resource_loader = ResourceLoader::singleton();
        let sprite = ErasedGdResource::new(resource_loader.load("./Scenes/sprite.tscn").unwrap());

        Self { sprite }
    }
}

fn spawn_sprite(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn(
        GodotScene::from_resource(assets.sprite.clone())
            .with_translation2d(Vector2 { x: 100.0, y: 100.0 })
            .with_transform2d(
                Transform2D::from(
                    GodotScene::from_resource(assets.sprite.clone()).get_transform2d(),
                )
                .scaled(Vector2 { x: 0.1, y: 0.1 }),
            ),
    );
}

fn move_sprite(mut sprite: Query<&mut ErasedGd>, mut delta: SystemDeltaTimer) {
    if let Ok(mut sprite) = sprite.single_mut() {
        let mut sprite = sprite.get::<Sprite2D>();
        let delta = delta.delta_seconds() * 20.0;
        let position = sprite.get_position();

        sprite.set_position(Vector2 {
            x: position.x + delta,
            y: position.y + delta,
        });
    }
}
