use bevy::prelude::*;
use conway_rs::{Simmetry, Stamp};

pub const COL_SIZE: usize = 90;
pub const ROW_SIZE: usize = 71;

pub const TICK_SECONDS: f32 = 0.5;

#[derive(Resource)]
pub struct SelectedStamp {
    pub stamp: Stamp,
    pub simmetry: Simmetry,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);

#[derive(Resource, Deref, DerefMut)]
struct NodeGrid(Timer);

#[derive(Component, Clone, Default)]
pub struct Item;

#[derive(Bundle, Clone, Default)]
pub struct ItemBundle {
    pub item: Item,
    pub node: NodeBundle,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct StampResource(pub Stamp);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct SimmetryResource(pub Simmetry);
