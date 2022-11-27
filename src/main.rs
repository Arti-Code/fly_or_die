mod arena;
mod background;

mod prelude {
    pub use crate::arena::*;
    pub use crate::background::*;
    pub use bevy::prelude::*; 
    pub use bevy_rapier2d::prelude::*;
    pub use rand::{thread_rng, Rng};
}

use crate::prelude::*;


fn main() {
    let mut app = App::new();
    
    app.insert_resource(ClearColor(Color::rgb_u8(0,0,0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Fly or Die".to_string(),
            width: ARENA_WIDTH,
            height: ARENA_HEIGHT,
            ..Default::default()
        },
        ..Default::default()
    }));

    app.add_plugin(BackgroundPlugin);
    app.add_plugin(particle_effects::ParticleEffectPlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0));
    app.add_plugin(InputManagerPlugin::<MenuAction>::default());


}
