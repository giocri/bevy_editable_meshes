use bevy::{prelude::*, utils::HashMap};
use crate::components::handle_point::*;
#[derive (Debug)]
pub enum Linktype{
    Face,
    Edge,
    SplineSection
}
#[derive(Debug, Component)]
pub struct EditableShape
{
    handles:Vec<HandlePoint>,
    groups:HashMap<String,Vec<i32>>,
    links:Vec<(Linktype,Vec<i32>)>
}