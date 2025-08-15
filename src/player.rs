use godot::classes::ISprite2D;
use godot::classes::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world from Rust!!"); // Prints to the Godot console

        Self { base }
    }
}
