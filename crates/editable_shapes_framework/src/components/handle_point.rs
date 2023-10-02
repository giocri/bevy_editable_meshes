use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct HandlePoint
{
    transform:Transform,
    index:i32
}