use bevy::prelude::*;

pub trait Meshmaker{
    fn generate_mesh(&self);
}