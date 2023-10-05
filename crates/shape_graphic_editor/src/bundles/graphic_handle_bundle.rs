use bevy::{prelude::*, window::PresentMode};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_transform_gizmo::TransformGizmoPlugin;
use editable_shape_framework::*;

#[derive(Bundle)]
struct GraphicHandleBundle {
    pbr:PbrBundle,
    index:i32,
    pickable:bevy_mod_picking::PickableBundle::default(),
    backends:bevy_mod_picking::backends::raycast::RaycastPickTarget::default(),
    gizmo:bevy_transform_gizmo::GizmoTransformable,
}