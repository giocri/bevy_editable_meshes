use bevy::prelude::*;
use editable_shape_framework::*;

#[derive(Debug, Component)]
pub struct GrabbableHandle
{
    pub handle: &mut HandlePoint,
    selected:bool
}