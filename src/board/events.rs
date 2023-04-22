use bevy::prelude::Entity;

use crate::board::components::Square;

#[derive(Clone, Copy)]
pub struct MoveMadeEvent {
    pub piece: Entity,
    pub destination: Square,
    pub origin: Square,
    pub taken: Option<Entity>,
}

pub struct ResetSelectedEvent;
