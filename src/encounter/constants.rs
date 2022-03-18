pub const ENCOUNTER_TABLE_SIZE_GRASS: usize = 12;
pub const ENCOUNTER_TABLE_SIZE_SURF: usize = 5;
pub const ENCOUNTER_TABLE_SIZE_ROCK_SMASH: usize = 5;
pub const ENCOUNTER_TABLE_SIZE_OLD_ROD: usize = 2;
pub const ENCOUNTER_TABLE_SIZE_GOOD_ROD: usize = 3;
pub const ENCOUNTER_TABLE_SIZE_SUPER_ROD: usize = 5;

pub const ENCOUNTER_CHANCES_GRASS: [u8; ENCOUNTER_TABLE_SIZE_GRASS] =
    [20, 20, 10, 10, 10, 10, 5, 5, 4, 4, 1, 1];
pub const ENCOUNTER_CHANCES_SURF: [u8; ENCOUNTER_TABLE_SIZE_SURF] =
    [60, 30, 5, 4, 1];
pub const ENCOUNTER_CHANCES_ROCK_SMASH: [u8; ENCOUNTER_TABLE_SIZE_ROCK_SMASH] =
    [60, 30, 5, 4, 1];
pub const ENCOUNTER_CHANCES_OLD_ROD: [u8; ENCOUNTER_TABLE_SIZE_OLD_ROD] =
    [70, 30];
pub const ENCOUNTER_CHANCES_GOOD_ROD: [u8; ENCOUNTER_TABLE_SIZE_GOOD_ROD] =
    [60, 20, 20];
pub const ENCOUNTER_CHANCES_SUPER_ROD: [u8; ENCOUNTER_TABLE_SIZE_SUPER_ROD] =
    [40, 40, 15, 4, 1];