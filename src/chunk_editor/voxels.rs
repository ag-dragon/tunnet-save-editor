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
    Terraformed,
    Cobble,
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
        match *self {
            VoxelType::Air => VoxelVisibility::Empty,
            VoxelType::Air2 => VoxelVisibility::Empty,
            VoxelType::Air3 => VoxelVisibility::Empty,
            VoxelType::Air4 => VoxelVisibility::Empty,
            VoxelType::Air5 => VoxelVisibility::Empty,
            VoxelType::Air6 => VoxelVisibility::Empty,
            VoxelType::Air7 => VoxelVisibility::Empty,
            _ => VoxelVisibility::Opaque,
        }
    }
}

impl TryFrom<i32> for VoxelType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VoxelType::Air),
            1 => Ok(VoxelType::Dirt),
            2 => Ok(VoxelType::UnbreakableDirt),
            3 => Ok(VoxelType::Grass),
            4 => Ok(VoxelType::UnbreakableGrass),
            5 => Ok(VoxelType::UnbreakableRock),
            6 => Ok(VoxelType::MetalWall),
            7 => Ok(VoxelType::Tiles),
            8 => Ok(VoxelType::MetalSheet),
            9 => Ok(VoxelType::GrayBrick),
            10 => Ok(VoxelType::Terraformed),
            11 => Ok(VoxelType::Cobble),
            12 => Ok(VoxelType::RedGlyph),
            13 => Ok(VoxelType::FlowerGlyph),
            14 => Ok(VoxelType::WoodPlanks),
            15 => Ok(VoxelType::TilesAlt),
            16 => Ok(VoxelType::CorruptedMetalWall),
            17 => Ok(VoxelType::Air2),
            18 => Ok(VoxelType::Magma),
            19 => Ok(VoxelType::Air3),
            20 => Ok(VoxelType::WobblyAir1),
            21 => Ok(VoxelType::WobblyAir2),
            22 => Ok(VoxelType::Air4),
            23 => Ok(VoxelType::Watefall1),
            24 => Ok(VoxelType::Watefall2),
            25 => Ok(VoxelType::Watefall3),
            26 => Ok(VoxelType::Watefall4),
            27 => Ok(VoxelType::Watefall5),
            28 => Ok(VoxelType::Watefall6),
            29 => Ok(VoxelType::Watefall7),
            30 => Ok(VoxelType::Watefall8),
            31 => Ok(VoxelType::Watefall9),
            32 => Ok(VoxelType::Watefall10),
            33 => Ok(VoxelType::Watefall11),
            34 => Ok(VoxelType::Watefall12),
            35 => Ok(VoxelType::Watefall13),
            36 => Ok(VoxelType::Watefall14),
            37 => Ok(VoxelType::Watefall15),
            38 => Ok(VoxelType::Watefall16),
            39 => Ok(VoxelType::UnbreakableRock2),
            40 => Ok(VoxelType::Air5),
            41 => Ok(VoxelType::Air6),
            42 => Ok(VoxelType::Air7),
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
            VoxelType::Grass => [0.25, 0.25],
            VoxelType::UnbreakableGrass => [0.25, 0.25],
            VoxelType::MetalWall => [0.75, 0.25],
            VoxelType::Tiles => [0.0, 0.25],
            VoxelType::MetalSheet => [0.75, 0.0],
            VoxelType::GrayBrick => [0.0, 0.50],
            VoxelType::Terraformed => [0.25, 0.50],
            VoxelType::Cobble => [0.50, 0.0],
            VoxelType::RedGlyph => [0.50, 0.25],
            VoxelType::FlowerGlyph => [0.50, 0.50],
            VoxelType::WoodPlanks => [0.75, 0.50],
            VoxelType::TilesAlt => [0.50, 0.75],
            VoxelType::CorruptedMetalWall => [0.75, 0.75],
            VoxelType::Rock => [0.25, 0.0],
            VoxelType::DamagedRock => [0.0, 0.75],
            _ => [0.0, 0.0],
        }
    }
}
