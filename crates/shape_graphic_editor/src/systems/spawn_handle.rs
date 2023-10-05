use bevy::prelude::*;
use editable_shape_framework::*;

pub fn spawn_handle(
    mut commands: Commands,
    icon:Handle<Mesh>,
    color: Handle<Material>,
    handle_point:&HandlePoint
){
    commands.spawn((
        GraphicHandleBundle {
            pbr:PbrBundle{
                mesh: icon,
                material: color,
                ..Default::default()
            },
            index:handle_point.index,
        }
    ));

}