use bevy::prelude::*;
use editable_shape_framework::*;

#[derive(Debug, Component)]
pub struct GrabbableHandle
{
    pub shape: &mut AbStractShape,
    pub vertex_handles:Vec<Entity>,
    pub edge_handles:Vec<Entity>,
    pub face_handles:Vec<Entity>,
}