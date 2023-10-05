use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct HandlePoint
{
    pub transform:Transform,
    pub index:i32
}