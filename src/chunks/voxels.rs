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
    MetalSheets,
    GrayBrick,
    Terraforming,
    UnbreakableCobble,
    RedGlyph,
    FlowerGlyph,
    WoodPlanks,
    Tiles2,
    CorruptedMetalWall,
    Air2,
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
    Air8,
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
            14 => Ok(VoxelType::WoodPlanks),
            _ => Ok(VoxelType::Dirt),
        }
    }
}

impl VoxelType {
    pub fn atlas_coords(&self) -> [f32; 2] {
        match self {
            VoxelType::WoodPlanks => [0.75, 0.50],
            _ => [0.0, 0.0],
        }
    }
}
