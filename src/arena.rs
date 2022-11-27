use crate::prelude::*;

pub const ARENA_WIDTH: f32=800.0;
pub const ARENA_HEIGHT: f32=600.0;

#[derive(Debug, Resource)]
pub struct Arena {
    pub asteroid_spawn_timer: Timer,
    pub score: u32,
}

pub struct ArenaPlugin;