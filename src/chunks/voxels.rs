use bevy::ecs::component::Component;
use block_mesh::{Voxel, VoxelVisibility};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum VoxelType {
    #[default]
    Air = 0,
    Dirt,
    UnbreakableDirt,
    Grass,
    UnbreakableGrass,
    UnbreakableRock,
    MetalWall,
    Tiles,
    MetalSheet,
    GrayBrick,
    Terraforming,
    UnbreakableCobble,
    RedGlyph,
    FlowerGlyph,
    WoodPlanks,
    TilesAlt,
    CorruptedMetalWall,
    Air2, // Appears in front of cook
    Magma,
    Air3,
    WobblyAir1,
    WobblyAir2,
    Air4,
    Watefall1,
    Watefall2,
    Watefall3,
    Watefall4,
    Watefall5,
    Watefall6,
    Watefall7,
    Watefall8,
    Watefall9,
    Watefall10,
    Watefall11,
    Watefall12,
    Watefall13,
    Watefall14,
    Watefall15,
    Watefall16,
    UnbreakableRock2,
    Air5,
    Air6,
    Air7,
    Rock,
    DamagedRock,
}

impl Voxel for VoxelType {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == VoxelType::Air {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl TryFrom<i32> for VoxelType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VoxelType::Air),
            1 => Ok(VoxelType::Dirt),
            6 => Ok(VoxelType::MetalWall),
            7 => Ok(VoxelType::Tiles),
            8 => Ok(VoxelType::MetalSheet),
            13 => Ok(VoxelType::FlowerGlyph),
            14 => Ok(VoxelType::WoodPlanks),
            15 => Ok(VoxelType::TilesAlt),
            17 => Ok(VoxelType::Air),
            19 => Ok(VoxelType::Air),
            40 => Ok(VoxelType::Air),
            41 => Ok(VoxelType::Air),
            42 => Ok(VoxelType::Air),
            43 => Ok(VoxelType::Rock),
            44 => Ok(VoxelType::DamagedRock),
            _ => Ok(VoxelType::Dirt),
        }
    }
}

impl VoxelType {
    pub fn atlas_coords(&self) -> [f32; 2] {
        match self {
            VoxelType::Dirt => [0.0, 0.0],
            VoxelType::MetalWall => [0.75, 0.25],
            VoxelType::Tiles => [0.0, 0.25],
            VoxelType::MetalSheet => [0.75, 0.0],
            VoxelType::FlowerGlyph => [0.50, 0.50],
            VoxelType::WoodPlanks => [0.75, 0.50],
            VoxelType::TilesAlt => [0.50, 0.75],
            VoxelType::Rock => [0.25, 0.0],
            VoxelType::DamagedRock => [0.0, 0.75],
            _ => [0.0, 0.0],
        }
    }
}
