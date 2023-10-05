use bevy::prelude::*;
use editable_shape_framework::*;

#[derive(Debug, Component)]
pub struct GrabbableEdge
{
    selected:bool
}