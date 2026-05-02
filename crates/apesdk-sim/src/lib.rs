#![allow(non_camel_case_types)]

//! Simulator constants and C-compatible public types for the ApeSDK Rust port.

use apesdk_toolkit::{
    array_add, array_number, array_object, math_random, math_random3, math_sine, math_tan, n_byte,
    n_byte2, n_byte4, n_int, n_spacetime, n_uint, n_vect2, object_array, object_number,
    object_object, object_parse_json, object_string, object_top_object, vect2_direction, vect2_dot,
    NFile, ObjectEntry, ObjectValue,
};

pub const SHORT_VERSION_NAME: &str = "Simulated Ape 0.708 ";
pub const FULL_DATE: &str = match option_env!("APESDK_FULL_DATE") {
    Some(value) => value,
    None => "May  1 2026",
};
pub const VERSION_NUMBER: n_byte2 = 708;
pub const COPYRIGHT_DATE: &str = "Copyright 1996 - 2026 ";
pub const FULL_VERSION_COPYRIGHT: &str = "Copyright Tom Barbalet, 1996-2026.";

pub const SIMULATED_APE_SIGNATURE: n_byte2 = (('N' as n_byte2) << 8) | ('A' as n_byte2);
pub const SIMULATED_WAR_SIGNATURE: n_byte2 = (('N' as n_byte2) << 8) | ('W' as n_byte2);

pub const BRAINCODE_SIZE: usize = 128;
pub const BRAINCODE_PROBES: usize = BRAINCODE_SIZE >> 3;
pub const BRAINCODE_PSPACE_REGISTERS: usize = 3;
pub const BRAINCODE_MAX_FREQUENCY: usize = 16;
pub const BRAINCODE_BYTES_PER_INSTRUCTION: usize = 3;
pub const BRAINCODE_BLOCK_COPY: usize = 16;
pub const BRAINCODE_MAX_ADDRESS: usize = BRAINCODE_SIZE * 2;
pub const BRAINCODE_CONSTANT0_BIT: n_byte = 64;
pub const BRAINCODE_CONSTANT1_BIT: n_byte = 128;
pub const BRAINPROBE_NATIVE_BYTES: usize = BRAINCODE_PROBES * 6;

pub const CHROMOSOMES: usize = 4;
pub const CHROMOSOME_Y: usize = 0;
pub const SEX_MALE: n_byte = 2;
pub const SEX_FEMALE: n_byte = 3;

pub const SOCIAL_SIZE: usize = 12;
pub const SOCIAL_SIZE_BEINGS: usize = SOCIAL_SIZE >> 1;
pub const EPISODIC_SIZE: usize = 12;
pub const MAX_FEATURESET_SIZE: usize = 16;
pub const IMMUNE_ANTIGENS: usize = 8;
pub const IMMUNE_POPULATION: usize = 16;
pub const IMMUNE_NATIVE_BYTES: usize =
    IMMUNE_ANTIGENS + IMMUNE_ANTIGENS + IMMUNE_POPULATION + IMMUNE_POPULATION + 4;
pub const NUMBER_OF_BODIES: usize = 10;
pub const INVENTORY_SIZE: usize = 8;
pub const INVENTORY_CHILD: n_byte2 = 1;
pub const INVENTORY_WOUND: n_byte2 = 2;
pub const INVENTORY_GROOMED: n_byte2 = 4;
pub const PREFERENCES: usize = 14;
pub const ATTENTION_SIZE: usize = 6;
pub const SHOUT_BYTES: usize = 6;
pub const DRIVES: usize = 4;

pub const MAP_BITS: usize = 9;
pub const MAP_TILES: usize = 1;
pub const MAP_DIMENSION: usize = 1 << MAP_BITS;
pub const MAP_AREA: usize = 1 << (2 * MAP_BITS);
pub const LAND_TOPOGRAPHY_BUFFERS: usize = 2;
pub const LAND_TOPOGRAPHY_PRIMARY: usize = 0;
pub const LAND_TOPOGRAPHY_WORKING: usize = 1;
pub const HI_RES_MAP_BITS: usize = MAP_BITS + 3;
pub const HI_RES_MAP_DIMENSION: usize = 1 << HI_RES_MAP_BITS;
pub const HI_RES_MAP_AREA: usize = 1 << (2 * HI_RES_MAP_BITS);
pub const APE_TO_MAP_BIT_RATIO: usize = 6;
pub const MAP_TO_TERRITORY_RATIO: usize = 5;
pub const MAP_APE_RESOLUTION_SIZE: n_byte2 = (MAP_DIMENSION << APE_TO_MAP_BIT_RATIO) as n_byte2;
pub const APESPACE_BOUNDS: n_byte2 = MAP_APE_RESOLUTION_SIZE - 1;
pub const TERRITORY_DIMENSION: usize = MAP_DIMENSION >> MAP_TO_TERRITORY_RATIO;
pub const TERRITORY_AREA: usize = TERRITORY_DIMENSION * TERRITORY_DIMENSION;
pub const WATER_MAP: n_int = 128;
pub const WATER_MAP2: n_int = WATER_MAP * 2;
pub const TIDE_AMPLITUDE_LUNAR: n_int = 8;
pub const TIDE_AMPLITUDE_SOLAR: n_int = 2;
pub const TIDE_MAX: n_int = WATER_MAP + TIDE_AMPLITUDE_LUNAR + TIDE_AMPLITUDE_SOLAR;
pub const LUNAR_ORBIT_MINS: n_int = 39_312;
pub const NEW_SD_MULTIPLE: n_int = 26_880;
pub const WEATHER_CLOUD: n_int = 32_768 >> 4;
pub const WEATHER_RAIN: n_int = WEATHER_CLOUD * 3;
pub const WEATHER_SEVEN_ERROR: n_int = -1;
pub const WEATHER_SEVEN_SUNNY_DAY: n_int = 0;
pub const WEATHER_SEVEN_CLOUDY_DAY: n_int = 1;
pub const WEATHER_SEVEN_RAINY_DAY: n_int = 2;
pub const WEATHER_SEVEN_CLEAR_NIGHT: n_int = 3;
pub const WEATHER_SEVEN_CLOUDY_NIGHT: n_int = 4;
pub const WEATHER_SEVEN_RAINY_NIGHT: n_int = 5;
pub const WEATHER_SEVEN_DAWN_DUSK: n_int = 6;
pub const FOOD_QUANTITY_MAX: n_byte = 255;
pub const FOOD_REGROWTH_INTERVAL_MINUTES: n_uint = (TIME_DAY_MINUTES / 16) as n_uint;

pub const TERRAIN_WINDOW_WIDTH: usize = 4096;
pub const TERRAIN_WINDOW_HEIGHT: usize = 3072;
pub const TERRAIN_WINDOW_AREA: usize = TERRAIN_WINDOW_WIDTH * TERRAIN_WINDOW_HEIGHT;
pub const CONTROL_WINDOW_WIDTH: usize = 2048;
pub const CONTROL_WINDOW_HEIGHT: usize = 2048;
pub const CONTROL_WINDOW_AREA: usize = CONTROL_WINDOW_WIDTH * CONTROL_WINDOW_HEIGHT;
pub const OFFSCREENSIZE: usize = MAP_AREA + TERRAIN_WINDOW_AREA + CONTROL_WINDOW_AREA;

pub const TIME_HOUR_MINUTES: usize = 60;
pub const TIME_DAY_MINUTES: usize = TIME_HOUR_MINUTES * 24;
pub const TIME_MONTH_MINUTES: usize = TIME_DAY_MINUTES * 28;
pub const TIME_YEAR_MINUTES: usize = TIME_MONTH_MINUTES * 13;
pub const TIME_YEAR_DAYS: usize = 7 * 52;
pub const TIME_CENTURY_DAYS: usize = TIME_YEAR_DAYS * 100;
pub const AGE_OF_MATURITY: n_byte4 = (12 * TIME_YEAR_DAYS) as n_byte4;

pub const BEING_DEAD: n_byte2 = 0;
pub const BEING_HUNGRY: n_byte2 = 10 * 128;
pub const BEING_FULL: n_byte2 = BEING_HUNGRY * 3;
pub const BEING_STATE_ASLEEP: n_byte2 = 0;
pub const BEING_STATE_AWAKE: n_byte2 = 1;
pub const BEING_STATE_HUNGRY: n_byte2 = 2;
pub const BEING_STATE_SWIMMING: n_byte2 = 4;
pub const BEING_STATE_EATING: n_byte2 = 8;
pub const BEING_STATE_MOVING: n_byte2 = 16;
pub const BEING_STATE_SPEAKING: n_byte2 = 32;
pub const BEING_STATE_SHOUTING: n_byte2 = 64;
pub const BEING_STATE_GROOMING: n_byte2 = 128;
pub const BEING_STATE_SUCKLING: n_byte2 = 256;
pub const BEING_STATE_SHOWFORCE: n_byte2 = 512;
pub const BEING_STATE_ATTACK: n_byte2 = 1024;
pub const BEING_STATE_NO_FOOD: n_byte2 = 2048;
pub const FULLY_ASLEEP: n_byte = 0;
pub const SLIGHTLY_AWAKE: n_byte = 1;
pub const FULLY_AWAKE: n_byte = 2;
pub const BIRTH_HEIGHT: n_byte2 = 2_000;
pub const BIRTH_MASS: n_byte2 = 100;
pub const MIN_CROWDING: n_byte = 1;
pub const MAX_CROWDING: n_byte = 3;
pub const SOCIAL_TOLLERANCE: n_byte = 1;
pub const DRIVES_MAX: n_byte = 255;
pub const DRIVE_HUNGER: usize = 0;
pub const DRIVE_SOCIAL: usize = 1;
pub const DRIVE_FATIGUE: usize = 2;
pub const DRIVE_SEX: usize = 3;
pub const PREFERENCE_MATE_HEIGHT_MALE: usize = 0;
pub const PREFERENCE_MATE_HEIGHT_FEMALE: usize = 1;
pub const PREFERENCE_MATE_PIGMENTATION_MALE: usize = 2;
pub const PREFERENCE_MATE_PIGMENTATION_FEMALE: usize = 3;
pub const PREFERENCE_MATE_HAIR_MALE: usize = 4;
pub const PREFERENCE_MATE_HAIR_FEMALE: usize = 5;
pub const PREFERENCE_MATE_FRAME_MALE: usize = 6;
pub const PREFERENCE_MATE_FRAME_FEMALE: usize = 7;
pub const PREFERENCE_GROOM_MALE: usize = 8;
pub const PREFERENCE_GROOM_FEMALE: usize = 9;
pub const PREFERENCE_ANECDOTE_EVENT_MUTATION: usize = 10;
pub const PREFERENCE_ANECDOTE_AFFECT_MUTATION: usize = 11;
pub const PREFERENCE_CHAT: usize = 12;
pub const PREFERENCE_SOCIAL: usize = 13;
pub const FATIGUE_SPEED_THRESHOLD: n_byte = 8;
pub const THRESHOLD_SEEK_MATE: n_byte = 100;
pub const BEING_MAX_MASS_G: n_byte2 = 7_000;
pub const BEING_MAX_MASS_FAT_G: n_byte2 = BEING_MAX_MASS_G >> 2;
pub const BEING_MAX_HEIGHT_MM: n_byte2 = 2_000;
pub const BEING_MAX_HEIGHT: n_byte2 = n_byte2::MAX;
pub const ENERGY_GRASS: n_byte2 = 50;
pub const ENERGY_BUSH: n_byte2 = 100;
pub const ENERGY_FRUIT: n_byte2 = 100;
pub const ENERGY_SEAWEED: n_byte2 = 30;
pub const ENERGY_SHELLFISH: n_byte2 = 300;
pub const ENERGY_NUT: n_byte2 = 200;
pub const ENERGY_FISH: n_byte2 = 600;
pub const ENERGY_BIRD_EGGS: n_byte2 = 800;
pub const ENERGY_LIZARD_EGGS: n_byte2 = 1_000;
pub const FOOD_VEGETABLE: n_byte = 0;
pub const FOOD_FRUIT: n_byte = 1;
pub const FOOD_SHELLFISH: n_byte = 2;
pub const FOOD_SEAWEED: n_byte = 3;
pub const FOOD_BIRD_EGGS: n_byte = 4;
pub const FOOD_LIZARD_EGGS: n_byte = 5;
pub const BODY_HEAD: n_byte = 0;
pub const BODY_TEETH: n_byte = 1;
pub const BODY_BACK: n_byte = 2;
pub const BODY_FRONT: n_byte = 3;
pub const BODY_LEFT_HAND: n_byte = 4;
pub const BODY_RIGHT_HAND: n_byte = 5;
pub const BODY_LEFT_FOOT: n_byte = 6;
pub const BODY_RIGHT_FOOT: n_byte = 7;
pub const INVENTORY_GRASS: n_byte2 = 4;
pub const INVENTORY_TWIG: n_byte2 = 8;
pub const INVENTORY_ROCK: n_byte2 = 16;
pub const INVENTORY_BRANCH: n_byte2 = 32;
pub const INVENTORY_SPEAR: n_byte2 = 64;
pub const INVENTORY_NUT: n_byte2 = 128;
pub const INVENTORY_NUT_CRACKED: n_byte2 = 256;
pub const INVENTORY_SCRAPER: n_byte2 = 512;
pub const INVENTORY_FISH: n_byte2 = 4_096;
pub const INVENTORY_BIRD_EGGS: n_byte2 = 8_192;
pub const INVENTORY_LIZARD_EGGS: n_byte2 = 16_384;
pub const ATTENTION_EXTERNAL: usize = 0;
pub const ATTENTION_ACTOR: usize = 1;
pub const ATTENTION_EPISODE: usize = 2;
pub const ATTENTION_BODY: usize = 3;
pub const ATTENTION_RELATIONSHIP: usize = 4;
pub const ATTENTION_TERRITORY: usize = 5;
pub const BEING_MEETER: usize = 0;
pub const BEING_MET: usize = 1;
pub const GOAL_NONE: n_byte2 = 0;
pub const GOAL_LOCATION: n_byte2 = 1;
pub const GOAL_MATE: n_byte2 = 2;
pub const RELATIONSHIP_SELF: n_byte = 1;
pub const SOCIAL_RESPECT_NORMAL: n_byte = 127;
pub const ENTITY_BEING: n_byte = 0;
pub const EVENT_EAT: n_byte = 1;
pub const EVENT_MATE: n_byte = 2;
pub const EVENT_HIT: n_byte = 3;
pub const EVENT_HIT_BY: n_byte = 4;
pub const EVENT_SWIM: n_byte = 5;
pub const EVENT_GROOM: n_byte = 6;
pub const EVENT_GROOMED: n_byte = 7;
pub const EVENT_CHAT: n_byte = 8;
pub const EVENT_SHOUT: n_byte = 9;
pub const EVENT_BIRTH: n_byte = 10;
pub const EVENT_CARRIED: n_byte = 11;
pub const EVENT_CARRIED_BY: n_byte = 12;
pub const EVENT_SUCKLED: n_byte = 13;
pub const EVENT_SUCKLED_BY: n_byte = 14;
pub const EVENT_SEEK_MATE: n_byte = 15;
pub const EVENT_WHACKED: n_byte = 16;
pub const EVENT_WHACKED_BY: n_byte = 17;
pub const EVENT_HURLED: n_byte = 18;
pub const EVENT_HURLED_BY: n_byte = 19;
pub const EVENT_HUGGED: n_byte = 20;
pub const EVENT_HUGGED_BY: n_byte = 21;
pub const EVENT_PRODDED: n_byte = 22;
pub const EVENT_PRODDED_BY: n_byte = 23;
pub const EVENT_DRAG: n_byte = 24;
pub const EVENT_BRANDISH: n_byte = 25;
pub const EVENT_DROP: n_byte = 26;
pub const EVENT_PICKUP: n_byte = 27;
pub const EVENT_GIVEN: n_byte = 28;
pub const EVENT_GIVEN_BY: n_byte = 29;
pub const EVENT_CHEW: n_byte = 30;
pub const EVENT_BASH_OBJECTS: n_byte = 31;
pub const EVENT_FISH: n_byte = 32;
pub const EVENT_SMILED: n_byte = 33;
pub const EVENT_SMILED_BY: n_byte = 34;
pub const EVENT_GLOWERED: n_byte = 35;
pub const EVENT_GLOWERED_BY: n_byte = 36;
pub const EVENT_PATTED: n_byte = 37;
pub const EVENT_PATTED_BY: n_byte = 38;
pub const EVENT_POINT: n_byte = 39;
pub const EVENT_POINTED: n_byte = 40;
pub const EVENT_TICKLED: n_byte = 41;
pub const EVENT_TICKLED_BY: n_byte = 42;
pub const EVENT_INTENTION: n_byte = 128;
pub const EPISODIC_AFFECT_ZERO: n_byte2 = 16_384;
pub const AFFECT_MATE: i32 = 1_000;
pub const AFFECT_CHAT: i32 = 100;
pub const AFFECT_GROOM: i32 = 100;
pub const AFFECT_SEEK_MATE: i32 = 600;
pub const AFFECT_SQUABBLE_VICTOR: i32 = 1_100;
pub const AFFECT_SQUABBLE_VANQUISHED: i32 = -800;
pub const FEATURESET_PIGMENTATION: n_byte = 0;
pub const FEATURESET_HAIR: n_byte = 1;
pub const FEATURESET_HEIGHT: n_byte = 2;
pub const FEATURESET_FAT: n_byte = 3;
pub const FEATURESET_EYE_SHAPE: n_byte = 4;
pub const FEATURESET_EYE_COLOR: n_byte = 5;
pub const FEATURESET_EYE_SEPARATION: n_byte = 6;
pub const FEATURESET_NOSE_SHAPE: n_byte = 7;
pub const FEATURESET_EAR_SHAPE: n_byte = 8;
pub const FEATURESET_EYEBROW_SHAPE: n_byte = 9;
pub const FEATURESET_MOUTH_SHAPE: n_byte = 10;
pub const MAX_FEATURE_FREQUENCY: n_byte2 = 2_048;
pub const MAX_FEATURESET_OBSERVATIONS: n_byte2 = 2_048;
pub const PAIR_BOND_THRESHOLD: n_byte = 8;
pub const GROOMING_PROB: n_byte2 = 10_000;
pub const GROOMING_PROB_HONOR: n_byte2 = 10;
pub const MAX_SPEED_WHILST_GROOMING: n_byte = 30;
pub const PARASITES_REMOVED: n_byte = 2;
pub const SQUABBLE_FLEE_SPEED: n_byte = 20;
pub const SQUABBLE_ENERGY_SHOWFORCE: i32 = 200;
pub const SQUABBLE_ENERGY_ATTACK: i32 = 500;
pub const SQUABBLE_DISRESPECT: n_byte = 20;
pub const SQUABBLE_HONOR_ADJUST: n_byte = 10;
pub const SQUABBLE_SHOW_FORCE_DISTANCE: n_int = 10;
pub const MINIMUM_GENETIC_VARIATION: n_int = 32;
pub const MATING_PROB: n_byte2 = 12;
pub const IMMUNE_FIT: n_byte = 5;
pub const MIN_ANTIBODIES: n_byte = 16;
pub const PATHOGEN_ENVIRONMENT_PROB: n_byte2 = 100;
pub const PATHOGEN_MUTATION_PROB: n_byte2 = 100;
pub const ANTIBODY_DEPLETION_PROB: n_byte2 = 100;
pub const PATHOGEN_TRANSMISSION_AIR: n_byte = 0;
pub const PATHOGEN_TRANSMISSION_SEX: n_byte = 2;
pub const PATHOGEN_TRANSMISSION_TOUCH: n_byte = 3;
pub const PATHOGEN_TRANSMISSION_FOOD_VEGETABLE: n_byte = 4;
pub const PATHOGEN_TRANSMISSION_TOTAL: n_byte2 = 8;
pub const BRAINCODE_DAT0: n_byte = 0;
pub const BRAINCODE_DAT1: n_byte = 1;
pub const BRAINCODE_ADD: n_byte = 2;
pub const BRAINCODE_SUB: n_byte = 3;
pub const BRAINCODE_MUL: n_byte = 4;
pub const BRAINCODE_DIV: n_byte = 5;
pub const BRAINCODE_MOD: n_byte = 6;
pub const BRAINCODE_MVB: n_byte = 7;
pub const BRAINCODE_MOV: n_byte = 8;
pub const BRAINCODE_JMP: n_byte = 9;
pub const BRAINCODE_CTR: n_byte = 10;
pub const BRAINCODE_SWP: n_byte = 11;
pub const BRAINCODE_INV: n_byte = 12;
pub const BRAINCODE_STP: n_byte = 13;
pub const BRAINCODE_LTP: n_byte = 14;
pub const BRAINCODE_JMZ: n_byte = 15;
pub const BRAINCODE_JMN: n_byte = 16;
pub const BRAINCODE_DJN: n_byte = 17;
pub const BRAINCODE_AND: n_byte = 18;
pub const BRAINCODE_OR: n_byte = 19;
pub const BRAINCODE_SEQ: n_byte = 20;
pub const BRAINCODE_SNE: n_byte = 21;
pub const BRAINCODE_SLT: n_byte = 22;
pub const BRAINCODE_SEN: n_byte = 23;
pub const BRAINCODE_SEN2: n_byte = 24;
pub const BRAINCODE_SEN3: n_byte = 25;
pub const BRAINCODE_ACT: n_byte = 26;
pub const BRAINCODE_ACT2: n_byte = 27;
pub const BRAINCODE_ACT3: n_byte = 28;
pub const BRAINCODE_ANE: n_byte = 29;
pub const BRAINCODE_INSTRUCTIONS: n_byte = 30;

pub const LARGE_SIM: n_uint = 256;
pub const INITIAL_POPULATION: usize = (LARGE_SIM as usize) >> 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_version {
    pub signature: n_byte2,
    pub version: n_byte2,
}

pub type n_genetics = n_byte4;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_feature {
    pub value: n_byte2,
    pub frequency: n_byte2,
    pub feature_type: n_byte,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_featureset {
    pub feature_number: n_byte2,
    pub features: [simulated_feature; MAX_FEATURESET_SIZE],
    pub observations: n_byte2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct simulated_isocial {
    pub space_time: n_spacetime,
    pub first_name: [n_byte2; 2],
    pub family_name: [n_byte2; 2],
    pub attraction: n_byte,
    pub friend_foe: n_byte,
    pub belief: n_byte2,
    pub familiarity: n_byte2,
    pub relationship: n_byte,
    pub entity_type: n_byte,
    pub classification: simulated_featureset,
    pub braincode: [n_byte; BRAINCODE_SIZE],
}

impl Default for simulated_isocial {
    fn default() -> Self {
        Self {
            space_time: n_spacetime::default(),
            first_name: [0; 2],
            family_name: [0; 2],
            attraction: 0,
            friend_foe: 0,
            belief: 0,
            familiarity: 0,
            relationship: 0,
            entity_type: 0,
            classification: simulated_featureset::default(),
            braincode: [0; BRAINCODE_SIZE],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_iepisodic {
    pub space_time: n_spacetime,
    pub first_name: [n_byte2; 2],
    pub family_name: [n_byte2; 2],
    pub event: n_byte,
    pub food: n_byte,
    pub affect: n_byte2,
    pub arg: n_byte2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_iplace {
    pub name: n_byte,
    pub familiarity: n_byte2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_ibrain_probe {
    pub probe_type: n_byte,
    pub position: n_byte,
    pub address: n_byte,
    pub frequency: n_byte,
    pub offset: n_byte,
    pub state: n_byte,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_immune_system {
    pub antigens: [n_byte; IMMUNE_ANTIGENS],
    pub shape_antigen: [n_byte; IMMUNE_ANTIGENS],
    pub antibodies: [n_byte; IMMUNE_POPULATION],
    pub shape_antibody: [n_byte; IMMUNE_POPULATION],
    pub random_seed: [n_byte2; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_idead_body {
    pub location: [n_byte2; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_remains {
    pub bodies: [simulated_idead_body; NUMBER_OF_BODIES],
    pub count: n_byte2,
    pub location: n_byte2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_being_constant {
    pub date_of_birth: n_byte4,
    pub generation_min: n_byte2,
    pub generation_max: n_byte2,
    pub name: [n_byte2; 2],
    pub genetics: [n_genetics; CHROMOSOMES],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_being_delta {
    pub location: [n_byte2; 2],
    pub direction_facing: n_byte,
    pub velocity: [n_byte; 10],
    pub stored_energy: n_byte2,
    pub random_seed: [n_byte2; 2],
    pub macro_state: n_byte2,
    pub parasites: n_byte,
    pub honor: n_byte,
    pub crowding: n_byte,
    pub height: n_byte2,
    pub mass: n_byte2,
    pub posture: n_byte,
    pub goal: [n_byte2; 4],
    pub social_coord_x: n_byte2,
    pub social_coord_y: n_byte2,
    pub social_coord_nx: n_byte2,
    pub social_coord_ny: n_byte2,
    pub awake: n_byte,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct simulated_being_events {
    pub social: [simulated_isocial; SOCIAL_SIZE],
    pub episodic: [simulated_iepisodic; EPISODIC_SIZE],
    pub territory: [simulated_iplace; TERRITORY_AREA],
}

impl Default for simulated_being_events {
    fn default() -> Self {
        Self {
            social: [simulated_isocial::default(); SOCIAL_SIZE],
            episodic: [simulated_iepisodic::default(); EPISODIC_SIZE],
            territory: [simulated_iplace::default(); TERRITORY_AREA],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_being_brain {
    pub braincode_register: [n_byte; BRAINCODE_PSPACE_REGISTERS],
    pub brainprobe: [simulated_ibrain_probe; BRAINCODE_PROBES],
    pub brain_state: [n_byte2; 6],
    pub script_overrides: n_byte2,
    pub attention: [n_byte; ATTENTION_SIZE],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_being_volatile {
    pub drives: [n_byte; DRIVES],
    pub shout: [n_byte; SHOUT_BYTES],
    pub inventory: [n_byte2; INVENTORY_SIZE],
    pub learned_preference: [n_byte; PREFERENCES],
    pub date_of_conception: n_byte4,
    pub fetal_genetics: [n_genetics; CHROMOSOMES],
    pub father_name: [n_byte2; 2],
    pub mother_name: [n_byte2; 2],
    pub child_generation_min: n_byte2,
    pub child_generation_max: n_byte2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct simulated_being {
    pub delta: simulated_being_delta,
    pub constant: simulated_being_constant,
    pub events: simulated_being_events,
    pub braindata: simulated_being_brain,
    pub changes: simulated_being_volatile,
    pub immune_system: simulated_immune_system,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KIND_OF_USE {
    KIND_PRE_STARTUP = -2,
    KIND_NOTHING_TO_RUN = -1,
    KIND_LOAD_FILE = 0,
    KIND_NEW_SIMULATION = 1,
    KIND_NEW_APES = 2,
    KIND_START_UP = 3,
    KIND_MEMORY_SETUP = 4,
}

pub fn banner() -> String {
    format!("\n *** {SHORT_VERSION_NAME}Console, {FULL_DATE} ***\n")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LandSnapshot {
    pub date: n_byte4,
    pub genetics: [n_byte2; 2],
    pub time: n_byte4,
}

impl LandSnapshot {
    pub const fn new(date: n_byte4, genetics: [n_byte2; 2], time: n_byte4) -> Self {
        Self {
            date,
            genetics,
            time,
        }
    }
}

const BIOLOGY_OPERATOR_AREA: usize = 0;
const BIOLOGY_OPERATOR_BUSH: usize = 6;
const BIOLOGY_OPERATOR_GRASS: usize = 7;
const BIOLOGY_OPERATOR_TREE: usize = 8;
const BIOLOGY_OPERATOR_SEAWEED: usize = 9;
const BIOLOGY_OPERATOR_ROCKPOOL: usize = 10;
const BIOLOGY_OPERATOR_BEACH: usize = 11;
const OFFSET_GRASS: n_int = 40;
const OFFSET_BUSH: n_int = 14;
const TERRAIN_OPERATOR_KIND: [&[u8; 6]; 17] = [
    b"+.....", b".+....", b"..+...", b"...+..", b"....+.", b".....+", b".-+.+-", b"..+.+-",
    b"-++.+-", b"-++.-+", b"+.-.-+", b"+-+-++", b"..+-.-", b"..--.-", b"..+++-", b"-+.-.-",
    b"..+-+-",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerrainSample {
    pub height: n_int,
    pub slope: n_vect2,
    pub map_position: [n_int; 2],
    pub water: bool,
    pub intertidal: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FoodSample {
    pub food_type: n_byte,
    pub max_energy: n_byte2,
    pub energy: n_byte2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BraincodeInstruction {
    pub opcode: n_byte,
    pub constant0: bool,
    pub constant1: bool,
    pub value0: n_byte,
    pub value1: n_byte,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BraincodeVm {
    local: [n_byte; BRAINCODE_SIZE],
    remote: [n_byte; BRAINCODE_SIZE],
    registers: [n_byte; BRAINCODE_PSPACE_REGISTERS],
    pc: usize,
}

impl BraincodeVm {
    pub const fn new(local: [n_byte; BRAINCODE_SIZE]) -> Self {
        Self {
            local,
            remote: [0; BRAINCODE_SIZE],
            registers: [0; BRAINCODE_PSPACE_REGISTERS],
            pc: 0,
        }
    }

    pub const fn local(&self) -> &[n_byte; BRAINCODE_SIZE] {
        &self.local
    }

    pub const fn registers(&self) -> [n_byte; BRAINCODE_PSPACE_REGISTERS] {
        self.registers
    }

    pub const fn pc(&self) -> usize {
        self.pc
    }

    pub fn set_registers(&mut self, registers: [n_byte; BRAINCODE_PSPACE_REGISTERS]) {
        self.registers = registers;
    }

    pub fn execute_step(&mut self) -> BraincodeInstruction {
        let instruction = braincode_decode(&self.local, self.pc);
        let addr0 = braincode_address(self.pc as n_int + n_int::from(instruction.value0));
        let addr1 = braincode_address(self.pc as n_int + n_int::from(instruction.value1));
        let read0 = self.read_address(addr0);
        let read1 = self.read_address(addr1);
        let value0 = if instruction.constant0 {
            instruction.value0
        } else {
            read0
        };
        let value1 = if instruction.constant1 {
            instruction.value1
        } else {
            read1
        };
        let comparison0 = if !instruction.constant0 && !instruction.constant1 {
            read0
        } else {
            instruction.value0
        };
        let mut next_pc = braincode_pc(self.pc as n_int + BRAINCODE_BYTES_PER_INSTRUCTION as n_int);

        match instruction.opcode {
            BRAINCODE_AND => {
                if instruction.constant0 {
                    self.write_address(addr0, read0 & read1);
                } else if read0 > 127 && read1 > 127 {
                    next_pc = braincode_pc(
                        self.pc as n_int + (BRAINCODE_BYTES_PER_INSTRUCTION * 2) as n_int,
                    );
                }
            }
            BRAINCODE_OR => {
                if instruction.constant0 {
                    self.write_address(addr0, read0 | read1);
                } else if read0 > 127 || read1 > 127 {
                    next_pc = braincode_pc(
                        self.pc as n_int + (BRAINCODE_BYTES_PER_INSTRUCTION * 2) as n_int,
                    );
                }
            }
            BRAINCODE_MOV => {
                self.write_address(
                    addr1,
                    if !instruction.constant0 && !instruction.constant1 {
                        read0
                    } else {
                        instruction.value0
                    },
                );
            }
            BRAINCODE_MVB => {
                let mut source = braincode_address(
                    self.pc as n_int
                        + (n_int::from(value0) * BRAINCODE_BYTES_PER_INSTRUCTION as n_int),
                );
                let mut destination = braincode_address(
                    self.pc as n_int
                        + (n_int::from(value0) * BRAINCODE_BYTES_PER_INSTRUCTION as n_int),
                );
                for _ in 0..=usize::from(self.registers[1] % BRAINCODE_BLOCK_COPY as n_byte) {
                    for offset in 0..BRAINCODE_BYTES_PER_INSTRUCTION {
                        let byte = self.read_address(braincode_address((source + offset) as n_int));
                        self.write_address(
                            braincode_address((destination + offset) as n_int),
                            byte,
                        );
                    }
                    source = braincode_address(
                        source as n_int + BRAINCODE_BYTES_PER_INSTRUCTION as n_int,
                    );
                    destination = braincode_address(
                        destination as n_int + BRAINCODE_BYTES_PER_INSTRUCTION as n_int,
                    );
                }
            }
            BRAINCODE_ADD => self.write_address(
                addr1,
                read1.wrapping_add(if !instruction.constant0 && !instruction.constant1 {
                    read0
                } else {
                    instruction.value0
                }),
            ),
            BRAINCODE_SUB => self.write_address(
                addr1,
                read1.wrapping_sub(if !instruction.constant0 && !instruction.constant1 {
                    read0
                } else {
                    instruction.value0
                }),
            ),
            BRAINCODE_MUL => self.write_address(
                addr1,
                read1.wrapping_mul(if !instruction.constant0 && !instruction.constant1 {
                    read0
                } else {
                    instruction.value0
                }),
            ),
            BRAINCODE_DIV => {
                let divisor = if !instruction.constant0 && !instruction.constant1 {
                    read0
                } else {
                    instruction.value0
                };
                self.write_address(addr1, read1 >> (divisor % 4));
            }
            BRAINCODE_MOD => {
                let divisor = if !instruction.constant0 && !instruction.constant1 {
                    read0
                } else {
                    instruction.value0
                };
                if divisor != 0 {
                    self.write_address(addr1, read1 % divisor);
                }
            }
            BRAINCODE_CTR => {
                self.write_address(
                    addr1,
                    if read0 > 127 {
                        read1.wrapping_add(1)
                    } else {
                        read1.wrapping_sub(1)
                    },
                );
            }
            BRAINCODE_JMP => {
                next_pc = self.jump_target(value0, value1);
            }
            BRAINCODE_JMZ if value0 == 0 => {
                next_pc = self.jump_target(0, value1);
            }
            BRAINCODE_JMN if value0 != 0 => {
                next_pc = self.jump_target(0, value1);
            }
            BRAINCODE_DJN if read0.wrapping_sub(1) != 0 => {
                self.write_address(addr0, read0.wrapping_sub(1));
                next_pc = self.jump_target(0, value1);
            }
            BRAINCODE_SEQ if read1 == comparison0 => {
                next_pc = self.skip_target();
            }
            BRAINCODE_SNE if read1 != comparison0 => {
                next_pc = self.skip_target();
            }
            BRAINCODE_SLT if read1 < comparison0 => {
                next_pc = self.skip_target();
            }
            BRAINCODE_SWP => {
                self.write_address(addr0, read1);
                self.write_address(addr1, read0);
            }
            BRAINCODE_INV => {
                if instruction.constant0 {
                    self.write_address(addr0, 255 - read0);
                } else {
                    self.write_address(addr1, 255 - read1);
                }
            }
            BRAINCODE_STP => {
                self.registers[usize::from(value0) % BRAINCODE_PSPACE_REGISTERS] = value1;
            }
            BRAINCODE_LTP => {
                self.write_address(
                    addr1,
                    self.registers[usize::from(value0) % BRAINCODE_PSPACE_REGISTERS],
                );
            }
            _ => {}
        }

        self.pc = next_pc;
        instruction
    }

    fn read_address(&self, address: usize) -> n_byte {
        if address < BRAINCODE_SIZE {
            self.local[address]
        } else {
            self.remote[address - BRAINCODE_SIZE]
        }
    }

    fn write_address(&mut self, address: usize, value: n_byte) {
        if address < BRAINCODE_SIZE {
            self.local[address] = value;
        } else {
            self.remote[address - BRAINCODE_SIZE] = value;
        }
    }

    fn jump_target(&self, high: n_byte, low: n_byte) -> usize {
        let mut target = braincode_pc(
            self.pc as n_int
                + (n_int::from((n_byte2::from(high) << 8) | n_byte2::from(low))
                    * BRAINCODE_BYTES_PER_INSTRUCTION as n_int),
        );
        if target <= self.pc && self.pc - target < (8 * BRAINCODE_BYTES_PER_INSTRUCTION) {
            target =
                braincode_pc(self.pc as n_int - (8 * BRAINCODE_BYTES_PER_INSTRUCTION) as n_int);
        }
        target
    }

    fn skip_target(&self) -> usize {
        braincode_pc(
            self.pc as n_int
                + (BRAINCODE_BYTES_PER_INSTRUCTION * (2 + usize::from(self.registers[0]))) as n_int,
        )
    }
}

pub fn braincode_decode(braincode: &[n_byte; BRAINCODE_SIZE], pc: usize) -> BraincodeInstruction {
    let command = braincode[pc % BRAINCODE_SIZE];
    BraincodeInstruction {
        opcode: (command & (BRAINCODE_CONSTANT0_BIT - 1)) % BRAINCODE_INSTRUCTIONS,
        constant0: command & BRAINCODE_CONSTANT0_BIT != 0,
        constant1: command & BRAINCODE_CONSTANT1_BIT != 0,
        value0: braincode[(pc + 1) % BRAINCODE_SIZE],
        value1: braincode[(pc + 2) % BRAINCODE_SIZE],
    }
}

fn braincode_address(value: n_int) -> usize {
    value.rem_euclid(BRAINCODE_MAX_ADDRESS as n_int) as usize
}

fn braincode_pc(value: n_int) -> usize {
    value.rem_euclid(BRAINCODE_SIZE as n_int) as usize
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerrainFoodFixtureSample {
    pub location: [n_byte2; 2],
    pub height: n_int,
    pub grass: n_int,
    pub trees: n_int,
    pub bush: n_int,
    pub seaweed: n_int,
    pub rockpool: n_int,
    pub beach: n_int,
    pub food_type: n_byte,
    pub max_energy: n_byte2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LandTile {
    genetics: [n_byte2; 2],
    topography: Vec<n_byte>,
}

impl LandTile {
    fn new() -> Self {
        Self {
            genetics: [0; 2],
            topography: vec![WATER_MAP as n_byte; LAND_TOPOGRAPHY_BUFFERS * MAP_AREA],
        }
    }

    pub fn genetics(&self) -> [n_byte2; 2] {
        self.genetics
    }

    pub fn topography_buffer(&self, buffer: usize) -> Option<&[n_byte]> {
        let start = buffer.checked_mul(MAP_AREA)?;
        let end = start.checked_add(MAP_AREA)?;
        self.topography.get(start..end)
    }

    fn topography_offset(buffer: usize, map_x: n_int, map_y: n_int) -> usize {
        (buffer * MAP_AREA) + map_index(map_x, map_y)
    }

    fn topography_at(&self, buffer: usize, map_x: n_int, map_y: n_int) -> n_byte {
        self.topography[Self::topography_offset(buffer, map_x, map_y)]
    }

    fn set_topography(&mut self, buffer: usize, map_x: n_int, map_y: n_int, value: n_byte) {
        let offset = Self::topography_offset(buffer, map_x, map_y);
        self.topography[offset] = value;
    }

    fn pack_topography(&mut self) {
        self.topography.fill(WATER_MAP as n_byte);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LandState {
    date: n_byte4,
    time: n_byte4,
    tiles: [LandTile; MAP_TILES],
    genetics: [n_byte2; 2],
    tide_level: n_byte,
    food_quantity: Vec<n_byte>,
    food_regrowth_minutes: n_uint,
}

impl LandState {
    pub fn new() -> Self {
        Self {
            date: 0,
            time: 0,
            tiles: std::array::from_fn(|_| LandTile::new()),
            genetics: [0; 2],
            tide_level: 0,
            food_quantity: vec![FOOD_QUANTITY_MAX; MAP_AREA],
            food_regrowth_minutes: 0,
        }
    }

    pub fn from_snapshot(snapshot: LandSnapshot) -> Self {
        let mut state = Self::new();
        state.date = snapshot.date;
        state.time = snapshot.time;
        state.tiles[0].genetics = snapshot.genetics;
        state.genetics = snapshot.genetics;
        state.regenerate_tiles();
        state.update_tide();
        state
    }

    pub const fn date(&self) -> n_byte4 {
        self.date
    }

    pub const fn time(&self) -> n_byte4 {
        self.time
    }

    pub const fn tide_level(&self) -> n_byte {
        self.tide_level
    }

    pub fn genetics(&self) -> [n_byte2; 2] {
        self.tiles[0].genetics
    }

    pub const fn planet_genetics(&self) -> [n_byte2; 2] {
        self.genetics
    }

    pub fn snapshot(&self) -> LandSnapshot {
        LandSnapshot::new(self.date, self.genetics(), self.time)
    }

    pub fn clear(&mut self, kind: KIND_OF_USE, start: n_byte4) {
        let tile_genetics = self.tile_genetics();
        *self = Self::new();
        for (tile, genetics) in self.tiles.iter_mut().zip(tile_genetics) {
            tile.genetics = genetics;
            tile.pack_topography();
        }
        self.regenerate_tiles();
        self.reset_food_quantities();
        if kind != KIND_OF_USE::KIND_LOAD_FILE {
            self.time = (5 * TIME_HOUR_MINUTES) as n_byte4;
            self.date = start;
            self.update_tide();
        }
    }

    pub fn cycle(&mut self) {
        self.time += 1;
        if self.time == TIME_DAY_MINUTES as n_byte4 {
            self.time = 0;
            self.date += 1;
        }
        self.update_tide();
        self.regrow_food_quantities(1);
    }

    pub fn advance_minutes(&mut self, minutes: n_uint) {
        let day_minutes = TIME_DAY_MINUTES as n_uint;
        let total_minutes = n_uint::from(self.time) + minutes;
        self.date = self
            .date
            .wrapping_add((total_minutes / day_minutes) as n_byte4);
        self.time = (total_minutes % day_minutes) as n_byte4;
        if minutes != 0 {
            self.update_tide();
            self.regrow_food_quantities(minutes);
        }
    }

    pub fn seed_genetics(&mut self, random: &mut [n_byte2; 2]) {
        for tile in &mut self.tiles {
            tile.genetics[0] = random_byte2(random);
            tile.genetics[1] = random_byte2(random);
            math_random3(random);
        }
        self.genetics[0] = random_byte2(random);
        self.genetics[1] = random_byte2(random);
        self.regenerate_tiles();
        self.reset_food_quantities();
    }

    pub fn update_tide(&mut self) {
        let current_time = n_int::try_from(
            n_uint::from(self.time)
                .wrapping_add(n_uint::from(self.date).wrapping_mul(TIME_DAY_MINUTES as n_uint)),
        )
        .unwrap_or(n_int::MAX);
        let lunar_mins = current_time.rem_euclid(LUNAR_ORBIT_MINS);
        let lunar_angle_256 = (((n_int::from(self.time) * 255) / 720)
            + ((lunar_mins * 255) / LUNAR_ORBIT_MINS))
            & 255;
        let solar_mins = current_time.rem_euclid((TIME_DAY_MINUTES * TIME_YEAR_DAYS) as n_int);
        let solar_angle_256 = (solar_mins * 255) / (TIME_DAY_MINUTES * TIME_YEAR_DAYS) as n_int;
        let lunar = math_sine(lunar_angle_256, NEW_SD_MULTIPLE / TIDE_AMPLITUDE_LUNAR);
        let solar = math_sine(solar_angle_256, NEW_SD_MULTIPLE / TIDE_AMPLITUDE_SOLAR);
        self.tide_level = (WATER_MAP + lunar + solar).clamp(0, 255) as n_byte;
    }

    pub fn height_at(&self, location: [n_byte2; 2]) -> n_int {
        self.land_location_map(
            apespace_to_mapspace(n_int::from(location[0])),
            apespace_to_mapspace(n_int::from(location[1])),
        )
    }

    pub fn terrain_sample(&self, location: [n_byte2; 2]) -> TerrainSample {
        let map_x = apespace_to_mapspace(n_int::from(location[0]));
        let map_y = apespace_to_mapspace(n_int::from(location[1]));
        let height = self.land_location_map(map_x, map_y);
        let slope = n_vect2::new(
            height - self.land_location_map(map_x + 1, map_y),
            height - self.land_location_map(map_x, map_y + 1),
        );
        TerrainSample {
            height,
            slope,
            map_position: [positive_map_coord(map_x), positive_map_coord(map_y)],
            water: water_test(height, self.tide_level),
            intertidal: height <= TIDE_MAX,
        }
    }

    pub fn biology_at(&self, location: [n_byte2; 2], operator: usize) -> n_int {
        let index = operator
            .saturating_sub(BIOLOGY_OPERATOR_AREA)
            .min(TERRAIN_OPERATOR_KIND.len() - 1);
        self.land_operator_interpolated(
            n_int::from(location[0]),
            n_int::from(location[1]),
            TERRAIN_OPERATOR_KIND[index],
        )
    }

    pub fn food_source_at(&self, location: [n_byte2; 2]) -> FoodSample {
        let height = self.height_at(location);
        let (food_type, max_energy) = if height > TIDE_MAX {
            self.land_food_source(location)
        } else {
            self.intertidal_food_source(location)
        };
        FoodSample {
            food_type,
            max_energy: scale_food_energy(max_energy, self.food_quantity_at(location)),
            energy: 0,
        }
    }

    pub fn terrain_food_fixture_sample(&self, location: [n_byte2; 2]) -> TerrainFoodFixtureSample {
        let (grass, trees, bush) = self.food_values(location);
        let seaweed = self.biology_at(location, BIOLOGY_OPERATOR_SEAWEED);
        let rockpool = self.biology_at(location, BIOLOGY_OPERATOR_ROCKPOOL);
        let mut beach = self.biology_at(location, BIOLOGY_OPERATOR_BEACH);
        beach += land_dither(seaweed, rockpool, beach);
        let food = self.food_source_at(location);
        TerrainFoodFixtureSample {
            location,
            height: self.height_at(location),
            grass,
            trees,
            bush,
            seaweed,
            rockpool,
            beach,
            food_type: food.food_type,
            max_energy: food.max_energy,
        }
    }

    pub fn tile(&self, tile: usize) -> Option<&LandTile> {
        self.tiles.get(tile)
    }

    pub fn topography_buffer(&self, tile: usize, buffer: usize) -> Option<&[n_byte]> {
        self.tiles
            .get(tile)
            .and_then(|tile| tile.topography_buffer(buffer))
    }

    pub fn topography_at_map(&self, map_x: n_int, map_y: n_int) -> n_byte {
        self.tiles[0].topography_at(LAND_TOPOGRAPHY_PRIMARY, map_x, map_y)
    }

    pub fn topography_highdef_at(&self, hires_x: n_int, hires_y: n_int) -> n_byte {
        let hires_x = positive_hires_coord(hires_x);
        let hires_y = positive_hires_coord(hires_y);
        let mic_x = hires_x & 7;
        let mic_y = hires_y & 7;
        let mac_x = hires_x >> 3;
        let mac_y = hires_y >> 3;

        let z00 = n_int::from(self.topography_at_map(mac_x, mac_y));
        let z01_raw = n_int::from(self.topography_at_map(mac_x + 1, mac_y));
        let z10_raw = n_int::from(self.topography_at_map(mac_x, mac_y + 1));
        let mut z10 = z10_raw - z00;
        let z11 = n_int::from(self.topography_at_map(mac_x + 1, mac_y + 1)) - z01_raw - z10;
        let z01 = (z01_raw - z00) << 3;
        z10 <<= 3;

        (z00 + (((z01 * mic_x) + (z10 * mic_y) + (z11 * mic_x * mic_y)) >> 6)) as n_byte
    }

    pub fn highres_tide_at(&self, hires_x: n_int, hires_y: n_int) -> bool {
        let topography = self.topography_highdef_at(hires_x, hires_y);
        topography > 105 && topography < 151
    }

    pub fn weather_pressure_at_map(&self, map_x: n_int, map_y: n_int) -> n_int {
        let center = self.land_location_map(map_x, map_y);
        let east = self.land_location_map(map_x + 1, map_y);
        let west = self.land_location_map(map_x - 1, map_y);
        let north = self.land_location_map(map_x, map_y - 1);
        let south = self.land_location_map(map_x, map_y + 1);
        let relief = (east - west).abs() + (south - north).abs() + (center - WATER_MAP).abs();
        let drift = math_sine(
            (map_x * 5 + map_y * 3 + n_int::from(self.time >> 4) + n_int::from(self.date)) & 255,
            512,
        ) + 512;
        (relief * 96 + drift).max(0)
    }

    pub fn weather_seven_at(&self, location: [n_byte2; 2]) -> n_int {
        self.weather_seven_at_map(
            apespace_to_mapspace(n_int::from(location[0])),
            apespace_to_mapspace(n_int::from(location[1])),
        )
    }

    pub fn weather_seven_at_map(&self, map_x: n_int, map_y: n_int) -> n_int {
        if is_dawndusk(self.time) {
            return WEATHER_SEVEN_DAWN_DUSK;
        }
        let ret_val = if is_night(self.time) {
            WEATHER_SEVEN_CLEAR_NIGHT
        } else {
            WEATHER_SEVEN_SUNNY_DAY
        };
        let pressure = self.weather_pressure_at_map(map_x, map_y);
        if pressure > WEATHER_RAIN {
            ret_val + 2
        } else if pressure > WEATHER_CLOUD {
            ret_val + 1
        } else {
            ret_val
        }
    }

    pub fn food_quantity_at(&self, location: [n_byte2; 2]) -> n_byte {
        let map_x = apespace_to_mapspace(n_int::from(location[0]));
        let map_y = apespace_to_mapspace(n_int::from(location[1]));
        self.food_quantity_at_map(map_x, map_y)
    }

    pub fn food_quantity_at_map(&self, map_x: n_int, map_y: n_int) -> n_byte {
        self.food_quantity[map_index(map_x, map_y)]
    }

    pub fn consume_food_at(&mut self, location: [n_byte2; 2], energy: n_byte2) -> n_byte {
        let map_x = apespace_to_mapspace(n_int::from(location[0]));
        let map_y = apespace_to_mapspace(n_int::from(location[1]));
        let index = map_index(map_x, map_y);
        if energy == BEING_DEAD {
            return self.food_quantity[index];
        }
        let depletion =
            ((n_uint::from(energy) + 31) / 32).clamp(1, n_uint::from(FOOD_QUANTITY_MAX)) as n_byte;
        self.food_quantity[index] = self.food_quantity[index].saturating_sub(depletion);
        self.food_quantity[index]
    }

    fn land_location_map(&self, map_x: n_int, map_y: n_int) -> n_int {
        n_int::from(self.topography_at_map(map_x, map_y))
    }

    fn regenerate_tiles(&mut self) {
        for tile_index in 0..MAP_TILES {
            let genetics = self.tiles[tile_index].genetics;
            let tile = &mut self.tiles[tile_index];
            for y in 0..MAP_DIMENSION {
                for x in 0..MAP_DIMENSION {
                    let value = Self::generated_topography_value(genetics, x as n_int, y as n_int);
                    tile.set_topography(LAND_TOPOGRAPHY_PRIMARY, x as n_int, y as n_int, value);
                    tile.set_topography(LAND_TOPOGRAPHY_WORKING, x as n_int, y as n_int, value);
                }
            }
        }
    }

    fn generated_topography_value(genetics: [n_byte2; 2], map_x: n_int, map_y: n_int) -> n_byte {
        let x = positive_map_coord(map_x);
        let y = positive_map_coord(map_y);
        let seed0 = n_int::from(genetics[0]);
        let seed1 = n_int::from(genetics[1]);
        let broad = math_sine((x * 3 + y * 5 + seed0) & 255, 512);
        let ridge = math_sine((x * 11 - y * 7 + seed1) & 255, 1_024);
        let dither = (((x * 37) ^ (y * 53) ^ seed0 ^ (seed1 << 1)) & 31) - 15;
        (WATER_MAP + broad + ridge + dither).clamp(0, 255) as n_byte
    }

    fn tile_genetics(&self) -> [[n_byte2; 2]; MAP_TILES] {
        std::array::from_fn(|index| self.tiles[index].genetics)
    }

    fn reset_food_quantities(&mut self) {
        self.food_quantity.fill(FOOD_QUANTITY_MAX);
        self.food_regrowth_minutes = 0;
    }

    fn regrow_food_quantities(&mut self, minutes: n_uint) {
        if minutes == 0 {
            return;
        }
        self.food_regrowth_minutes = self.food_regrowth_minutes.saturating_add(minutes);
        let regrowth = self.food_regrowth_minutes / FOOD_REGROWTH_INTERVAL_MINUTES;
        if regrowth == 0 {
            return;
        }
        self.food_regrowth_minutes %= FOOD_REGROWTH_INTERVAL_MINUTES;
        let regrowth = regrowth.min(n_uint::from(FOOD_QUANTITY_MAX)) as n_byte;
        for quantity in &mut self.food_quantity {
            *quantity = quantity.saturating_add(regrowth);
        }
    }

    fn land_operator_interpolated(&self, loc_x: n_int, loc_y: n_int, kind: &[u8; 6]) -> n_int {
        let loc_x = n_int::from(wrap_apespace(loc_x));
        let loc_y = n_int::from(wrap_apespace(loc_y));
        let map_x = apespace_to_mapspace(loc_x);
        let map_y = apespace_to_mapspace(loc_y);
        let local_x = loc_x - (map_x << APE_TO_MAP_BIT_RATIO);
        let local_y = loc_y - (map_y << APE_TO_MAP_BIT_RATIO);
        let next_x_weight = ((map_x + 1) << APE_TO_MAP_BIT_RATIO) - loc_x;
        let next_y_weight = ((map_y + 1) << APE_TO_MAP_BIT_RATIO) - loc_y;

        let mut interpolated =
            (self.land_operator(map_x + 1, map_y, kind) * local_x) >> APE_TO_MAP_BIT_RATIO;
        interpolated +=
            (self.land_operator(map_x - 1, map_y, kind) * next_x_weight) >> APE_TO_MAP_BIT_RATIO;
        interpolated +=
            (self.land_operator(map_x, map_y + 1, kind) * local_y) >> APE_TO_MAP_BIT_RATIO;
        interpolated +=
            (self.land_operator(map_x, map_y - 1, kind) * next_y_weight) >> APE_TO_MAP_BIT_RATIO;
        interpolated >> 1
    }

    fn land_operator(&self, loc_x: n_int, loc_y: n_int, kind: &[u8; 6]) -> n_int {
        let fg_raw = self.land_location_map(loc_x, loc_y);
        let dfg = (self.land_location_map(loc_x + 1, loc_y) - fg_raw) * 8;
        let fdg = (self.land_location_map(loc_x, loc_y + 1) - fg_raw) * 8;
        let fg = fg_raw - WATER_MAP;
        let mut total = 0;
        let mut number_sum = 0;

        for (index, operator) in kind.iter().copied().enumerate() {
            if operator == b'.' {
                continue;
            }
            let temp_add = match index {
                0 => ((dfg * dfg) + (fdg * fdg)) >> 6,
                1 => ((WATER_MAP + fg) * (WATER_MAP + fg)) >> 8,
                2 => ((WATER_MAP - fg) * (WATER_MAP - fg)) >> 8,
                3 => {
                    if is_night(self.time) {
                        continue;
                    }
                    let hour_angle = (((self.time << 6) as n_int / 180) + 32) & 255;
                    let weather = self.weather_seven_at_map(loc_x, loc_y).max(0) % 3;
                    let weather_divide = 105 + (weather * 30);
                    let time_weather = vect2_direction(hour_angle, weather_divide * 32);
                    let weather_offset = 840 / weather_divide;
                    operator_sun(
                        fg,
                        dfg,
                        fdg,
                        time_weather.x + weather_offset,
                        time_weather.y + weather_offset,
                    )
                }
                4 => operator_sun(fg, dfg, fdg, 11, 11),
                5 => {
                    let salt = -(fg - TIDE_AMPLITUDE_LUNAR - TIDE_AMPLITUDE_SOLAR);
                    if !(0..=((TIDE_AMPLITUDE_LUNAR + TIDE_AMPLITUDE_SOLAR) * 2)).contains(&salt) {
                        if operator == b'+' {
                            total = 0;
                        }
                        continue;
                    }
                    ((salt * salt) + (dfg * fdg)) >> 4
                }
                _ => continue,
            };
            number_sum += 1;
            if operator == b'+' {
                total += temp_add;
            } else {
                total += WATER_MAP2 - temp_add;
            }
        }

        if number_sum == 0 {
            0
        } else {
            total / number_sum
        }
    }

    fn food_values(&self, location: [n_byte2; 2]) -> (n_int, n_int, n_int) {
        let mut grass = self.biology_at(location, BIOLOGY_OPERATOR_GRASS) + OFFSET_GRASS;
        let trees = self.biology_at(location, BIOLOGY_OPERATOR_TREE);
        let bush = self.biology_at(location, BIOLOGY_OPERATOR_BUSH) + OFFSET_BUSH;
        grass += land_dither(grass, trees, bush);
        (grass, trees, bush)
    }

    fn land_food_source(&self, location: [n_byte2; 2]) -> (n_byte, n_byte2) {
        let (grass, trees, bush) = self.food_values(location);
        if grass > bush && grass > trees {
            (FOOD_VEGETABLE, ENERGY_GRASS)
        } else if bush > trees {
            (FOOD_VEGETABLE, ENERGY_BUSH)
        } else {
            (FOOD_FRUIT, ENERGY_FRUIT)
        }
    }

    fn intertidal_food_source(&self, location: [n_byte2; 2]) -> (n_byte, n_byte2) {
        let seaweed = self.biology_at(location, BIOLOGY_OPERATOR_SEAWEED);
        let rockpool = self.biology_at(location, BIOLOGY_OPERATOR_ROCKPOOL);
        let mut beach = self.biology_at(location, BIOLOGY_OPERATOR_BEACH);
        beach += land_dither(seaweed, rockpool, beach);
        if seaweed > rockpool && seaweed > beach {
            (FOOD_SEAWEED, ENERGY_SEAWEED)
        } else if rockpool > beach {
            (FOOD_SHELLFISH, ENERGY_SHELLFISH)
        } else {
            (FOOD_VEGETABLE, BEING_DEAD)
        }
    }
}

impl Default for LandState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BeingSummary {
    name: String,
    gender_name: n_byte2,
    family_name: n_byte2,
    date_of_birth: n_byte4,
    generation_min: n_byte2,
    generation_max: n_byte2,
    genetics: [n_genetics; CHROMOSOMES],
    location: [n_byte2; 2],
    facing: n_byte,
    random_seed: [n_byte2; 2],
    energy: n_byte2,
    velocity: [n_byte; 10],
    macro_state: n_byte2,
    parasites: n_byte,
    honor: n_byte,
    crowding: n_byte,
    height: n_byte2,
    mass: n_byte2,
    posture: n_byte,
    goal: [n_byte2; 4],
    social_coord: [n_byte2; 4],
    awake: bool,
    awake_level: n_byte,
    drives: [n_byte; DRIVES],
    braincode_register: [n_byte; BRAINCODE_PSPACE_REGISTERS],
    attention: [n_byte; ATTENTION_SIZE],
    brainprobe: [simulated_ibrain_probe; BRAINCODE_PROBES],
    brain_state: [n_byte2; 6],
    script_overrides: n_byte2,
    shout: [n_byte; SHOUT_BYTES],
    inventory: [n_byte2; INVENTORY_SIZE],
    learned_preference: [n_byte; PREFERENCES],
    date_of_conception: n_byte4,
    fetal_genetics: [n_genetics; CHROMOSOMES],
    father_name: [n_byte2; 2],
    mother_name: [n_byte2; 2],
    child_generation_min: n_byte2,
    child_generation_max: n_byte2,
    social_memory: [simulated_isocial; SOCIAL_SIZE],
    episodic_memory: [simulated_iepisodic; EPISODIC_SIZE],
    immune_antigens: [n_byte; IMMUNE_ANTIGENS],
    immune_shape_antigen: [n_byte; IMMUNE_ANTIGENS],
    immune_antibodies: [n_byte; IMMUNE_POPULATION],
    immune_shape_antibody: [n_byte; IMMUNE_POPULATION],
    immune_seed: [n_byte2; 2],
}

impl BeingSummary {
    pub fn new(
        name: String,
        gender_name: n_byte2,
        family_name: n_byte2,
        date_of_birth: n_byte4,
        genetics: [n_genetics; CHROMOSOMES],
    ) -> Self {
        let mut being = Self {
            name,
            gender_name,
            family_name,
            date_of_birth,
            generation_min: 0,
            generation_max: 0,
            genetics,
            location: [0; 2],
            facing: 0,
            random_seed: [0; 2],
            energy: 0,
            velocity: [0; 10],
            macro_state: 0,
            parasites: 0,
            honor: 0,
            crowding: MIN_CROWDING,
            height: BIRTH_HEIGHT,
            mass: BIRTH_MASS,
            posture: 0,
            goal: [0; 4],
            social_coord: [0; 4],
            awake: true,
            awake_level: FULLY_AWAKE,
            drives: [127; DRIVES],
            braincode_register: [0; BRAINCODE_PSPACE_REGISTERS],
            attention: [0; ATTENTION_SIZE],
            brainprobe: [simulated_ibrain_probe::default(); BRAINCODE_PROBES],
            brain_state: [0; 6],
            script_overrides: 0,
            shout: [0; SHOUT_BYTES],
            inventory: [0; INVENTORY_SIZE],
            learned_preference: [0; PREFERENCES],
            date_of_conception: 0,
            fetal_genetics: [0; CHROMOSOMES],
            father_name: [0; 2],
            mother_name: [0; 2],
            child_generation_min: 0,
            child_generation_max: 0,
            social_memory: [simulated_isocial::default(); SOCIAL_SIZE],
            episodic_memory: [simulated_iepisodic::default(); EPISODIC_SIZE],
            immune_antigens: [0; IMMUNE_ANTIGENS],
            immune_shape_antigen: [0; IMMUNE_ANTIGENS],
            immune_antibodies: [0; IMMUNE_POPULATION],
            immune_shape_antibody: [0; IMMUNE_POPULATION],
            immune_seed: [0; 2],
        };
        being.init_social_memory();
        being.init_episodic_memory();
        being
    }

    pub fn initial(index: usize, random: &mut [n_byte2; 2]) -> Self {
        math_random3(random);
        let male = index % 2 == 0;
        let genetics = random_genetics(random, male);
        math_random3(random);
        let first_name = random[0] & 255;
        let family_name = ((random[0] & 255) | ((random[1] & 255) << 8)) as n_byte2;
        let sex = if male { SEX_MALE } else { SEX_FEMALE };
        let gender_name = first_name | ((sex as n_byte2) << 8);
        let mut being = Self::new(
            format!("Ape {:03}", index + 1),
            gender_name,
            family_name,
            0,
            genetics,
        );
        math_random3(random);
        being.location = [random[0], random[1]];
        being.facing = (math_random(random) & 255) as n_byte;
        math_random3(random);
        being.random_seed = [random[0], random[1]];
        being.energy = BEING_FULL + ((index as n_byte2) % 512);
        being.set_speed((index % 16) as n_byte);
        being.macro_state = BEING_STATE_AWAKE;
        being.awake = true;
        being.awake_level = FULLY_AWAKE;
        being.parasites = 0;
        being.honor = (index % 100) as n_byte;
        being.crowding = MIN_CROWDING;
        being.height = BIRTH_HEIGHT + ((index as n_byte2) % 128);
        being.mass = BIRTH_MASS + ((index as n_byte2) % 64);
        being.posture = 0;
        being.goal = [0; 4];
        being.social_coord = [
            16_384 + ((random[0] >> 1) & 32_767),
            16_384 + ((random[1] >> 1) & 32_767),
            16_384 + ((random[0] >> 1) & 32_767),
            16_384 + ((random[1] >> 1) & 32_767),
        ];
        being.drives = [
            (64 + (index % 64)) as n_byte,
            (80 + (index % 48)) as n_byte,
            (96 + (index % 32)) as n_byte,
            (112 + (index % 16)) as n_byte,
        ];
        being.braincode_register = [
            (b'A' + (index % 26) as u8) as n_byte,
            (b'A' + ((index + 1) % 26) as u8) as n_byte,
            (b'A' + ((index + 2) % 26) as u8) as n_byte,
        ];
        being.immune_seed = [random[0], random[1]];
        being.init_immune();
        being
    }

    pub fn from_transfer_object(entries: &[ObjectEntry]) -> Result<Self, &'static str> {
        if optional_field(entries, "delta").is_some()
            || optional_field(entries, "constant").is_some()
        {
            return Self::from_native_transfer_object(entries);
        }

        let name = field_string(entries, "name")?.to_string();
        let gender_name = field_named_byte2(entries, "gender name")?;
        let family_name = field_named_byte2(entries, "family name")?;
        let date_of_birth = field_byte4(entries, "date of birth")?;
        let generation_min = optional_number_byte2(entries, "generation min")?.unwrap_or(0);
        let generation_max = optional_number_byte2(entries, "generation max")?.unwrap_or(0);
        let genetics = field_genetics4(entries, "genetics")?;
        let awake_level = optional_number_byte(entries, "awake")?.unwrap_or(FULLY_AWAKE);
        let velocity = if let Some(velocity) = optional_velocity(entries, "velocity")? {
            velocity
        } else {
            optional_number_byte(entries, "speed")?
                .map(speed_history)
                .unwrap_or([0; 10])
        };
        let mut being = Self {
            name,
            gender_name,
            family_name,
            date_of_birth,
            generation_min,
            generation_max,
            genetics,
            location: optional_array_byte2_2(entries, "location")?.unwrap_or([0; 2]),
            facing: optional_number_byte(entries, "facing")?.unwrap_or(0),
            random_seed: optional_array_byte2_2(entries, "random seed")?.unwrap_or([0; 2]),
            energy: optional_number_byte2(entries, "energy")?.unwrap_or(0),
            velocity,
            macro_state: optional_number_byte2(entries, "macro state")?.unwrap_or(0),
            parasites: optional_number_byte(entries, "parasites")?.unwrap_or(0),
            honor: optional_number_byte(entries, "honor")?.unwrap_or(0),
            crowding: optional_number_byte(entries, "crowding")?.unwrap_or(MIN_CROWDING),
            height: optional_number_byte2(entries, "height")?.unwrap_or(BIRTH_HEIGHT),
            mass: optional_number_byte2(entries, "mass")?.unwrap_or(BIRTH_MASS),
            posture: optional_number_byte(entries, "posture")?.unwrap_or(0),
            goal: optional_array_byte2_4(entries, "goal")?.unwrap_or([0; 4]),
            social_coord: optional_array_byte2_4(entries, "social coord")?.unwrap_or([0; 4]),
            awake: awake_level != FULLY_ASLEEP,
            awake_level,
            drives: [127; DRIVES],
            braincode_register: [0; BRAINCODE_PSPACE_REGISTERS],
            attention: [0; ATTENTION_SIZE],
            brainprobe: [simulated_ibrain_probe::default(); BRAINCODE_PROBES],
            brain_state: [0; 6],
            script_overrides: 0,
            shout: [0; SHOUT_BYTES],
            inventory: [0; INVENTORY_SIZE],
            learned_preference: [0; PREFERENCES],
            date_of_conception: 0,
            fetal_genetics: [0; CHROMOSOMES],
            father_name: [0; 2],
            mother_name: [0; 2],
            child_generation_min: 0,
            child_generation_max: 0,
            social_memory: [simulated_isocial::default(); SOCIAL_SIZE],
            episodic_memory: [simulated_iepisodic::default(); EPISODIC_SIZE],
            immune_antigens: [0; IMMUNE_ANTIGENS],
            immune_shape_antigen: [0; IMMUNE_ANTIGENS],
            immune_antibodies: [0; IMMUNE_POPULATION],
            immune_shape_antibody: [0; IMMUNE_POPULATION],
            immune_seed: optional_array_byte2_2(entries, "immune seed")?.unwrap_or([0; 2]),
        };
        if let Some(drives) = optional_array_byte(entries, "drives", DRIVES)? {
            being.drives.copy_from_slice(&drives);
        }
        if let Some(registers) =
            optional_array_byte(entries, "braincode register", BRAINCODE_PSPACE_REGISTERS)?
        {
            being.braincode_register.copy_from_slice(&registers);
        }
        if let Some(attention) = optional_array_byte(entries, "attention", ATTENTION_SIZE)? {
            being.attention.copy_from_slice(&attention);
        }
        if !being.load_social_memory(entries)? {
            being.init_social_memory();
        }
        if !being.load_episodic_memory(entries)? {
            being.init_episodic_memory();
        }
        Ok(being)
    }

    pub fn transfer_object(&self) -> Vec<ObjectEntry> {
        self.native_transfer_object()
    }

    fn from_native_transfer_object(entries: &[ObjectEntry]) -> Result<Self, &'static str> {
        let name = field_string(entries, "name")?.to_string();
        let delta = field_object(entries, "delta")?;
        let constant = field_object(entries, "constant")?;
        let genetics = field_genetics4(constant, "genetics")?;
        let identity = optional_array_byte2_2(constant, "name")?;
        let generation_range =
            optional_array_byte2_2(constant, "generation_range")?.unwrap_or([0; 2]);
        let gender_name = optional_number_byte2(entries, "gender name")?
            .or_else(|| identity.map(|identity| identity[0]))
            .unwrap_or_else(|| inferred_gender_name(&name, genetics));
        let family_name = optional_number_byte2(entries, "family name")?
            .or_else(|| identity.map(|identity| identity[1]))
            .unwrap_or(0);
        let energy = optional_number_byte2(delta, "stored_energy")?.unwrap_or(BEING_DEAD);
        let velocity = optional_velocity(delta, "velocity")?.unwrap_or([0; 10]);
        let awake_level = optional_number_byte(delta, "awake")?.unwrap_or(if energy > BEING_DEAD {
            FULLY_AWAKE
        } else {
            FULLY_ASLEEP
        });
        let mut being = Self {
            name,
            gender_name,
            family_name,
            date_of_birth: field_byte4(constant, "date_of_birth")?,
            generation_min: generation_range[0],
            generation_max: generation_range[1],
            genetics,
            location: optional_array_byte2_2(delta, "location")?.unwrap_or([0; 2]),
            facing: optional_number_byte(delta, "direction_facing")?.unwrap_or(0),
            random_seed: optional_array_byte2_2(delta, "random_seed")?.unwrap_or([0; 2]),
            energy,
            velocity,
            macro_state: optional_number_byte2(delta, "macro_state")?.unwrap_or(0),
            parasites: optional_number_byte(delta, "parasites")?.unwrap_or(0),
            honor: optional_number_byte(delta, "honor")?.unwrap_or(0),
            crowding: optional_number_byte(delta, "crowding")?.unwrap_or(MIN_CROWDING),
            height: optional_number_byte2(delta, "height")?.unwrap_or(BIRTH_HEIGHT),
            mass: optional_number_byte2(delta, "mass")?.unwrap_or(BIRTH_MASS),
            posture: optional_number_byte(delta, "posture")?.unwrap_or(0),
            goal: optional_array_byte2_4(delta, "goal")?.unwrap_or([0; 4]),
            social_coord: optional_array_byte2_4(delta, "social_coord")?.unwrap_or([0; 4]),
            awake: awake_level != FULLY_ASLEEP,
            awake_level,
            drives: [127; DRIVES],
            braincode_register: [0; BRAINCODE_PSPACE_REGISTERS],
            attention: [0; ATTENTION_SIZE],
            brainprobe: [simulated_ibrain_probe::default(); BRAINCODE_PROBES],
            brain_state: [0; 6],
            script_overrides: 0,
            shout: [0; SHOUT_BYTES],
            inventory: [0; INVENTORY_SIZE],
            learned_preference: [0; PREFERENCES],
            date_of_conception: 0,
            fetal_genetics: [0; CHROMOSOMES],
            father_name: [0; 2],
            mother_name: [0; 2],
            child_generation_min: 0,
            child_generation_max: 0,
            social_memory: [simulated_isocial::default(); SOCIAL_SIZE],
            episodic_memory: [simulated_iepisodic::default(); EPISODIC_SIZE],
            immune_antigens: [0; IMMUNE_ANTIGENS],
            immune_shape_antigen: [0; IMMUNE_ANTIGENS],
            immune_antibodies: [0; IMMUNE_POPULATION],
            immune_shape_antibody: [0; IMMUNE_POPULATION],
            immune_seed: [0; 2],
        };

        if let Some(changes) = optional_object(entries, "changes")? {
            if let Some(drives) = optional_array_byte(changes, "drives", DRIVES)? {
                being.drives.copy_from_slice(&drives);
            }
            if let Some(shout) = optional_array_byte(changes, "shout", SHOUT_BYTES)? {
                being.shout.copy_from_slice(&shout);
            }
            if let Some(inventory) = optional_array_byte2(changes, "inventory", INVENTORY_SIZE)? {
                being.inventory.copy_from_slice(&inventory);
            }
            if let Some(preferences) =
                optional_array_byte(changes, "learned_preference", PREFERENCES)?
            {
                being.learned_preference.copy_from_slice(&preferences);
            }
            being.date_of_conception =
                optional_number_byte4(changes, "date_of_conception")?.unwrap_or(0);
            if let Some(fetal_genetics) = optional_genetics4(changes, "fetal_genetics")? {
                being.fetal_genetics = fetal_genetics;
            }
            being.father_name = optional_array_byte2_2(changes, "father_name")?.unwrap_or([0; 2]);
            being.mother_name = optional_array_byte2_2(changes, "mother_name")?.unwrap_or([0; 2]);
            let child_generation =
                optional_array_byte2_2(changes, "child_generation_range")?.unwrap_or([0; 2]);
            being.child_generation_min = child_generation[0];
            being.child_generation_max = child_generation[1];
        }
        if let Some(brain) = optional_object(entries, "braindata")? {
            if let Some(registers) =
                optional_array_byte(brain, "braincode_register", BRAINCODE_PSPACE_REGISTERS)?
            {
                being.braincode_register.copy_from_slice(&registers);
            }
            if let Some(attention) = optional_array_byte(brain, "attention", ATTENTION_SIZE)? {
                being.attention.copy_from_slice(&attention);
            }
            if let Some(brain_state) = optional_array_byte2(brain, "brain_state", 6)? {
                being.brain_state.copy_from_slice(&brain_state);
            }
            being.script_overrides = optional_number_byte2(brain, "script_overrides")?.unwrap_or(0);
            if let Some(probes) = optional_brainprobe_array(brain, "brainprobe")? {
                being.brainprobe = probes;
            }
        }
        if let Some(immune) = optional_object(entries, "immune_system")? {
            being.immune_seed = optional_array_byte2_2(immune, "random_seed")?.unwrap_or([0; 2]);
            if let Some(values) = optional_array_byte(immune, "antigens", IMMUNE_ANTIGENS)? {
                being.immune_antigens.copy_from_slice(&values);
            }
            if let Some(values) = optional_array_byte(immune, "shape_antigen", IMMUNE_ANTIGENS)? {
                being.immune_shape_antigen.copy_from_slice(&values);
            }
            if let Some(values) = optional_array_byte(immune, "antibodies", IMMUNE_POPULATION)? {
                being.immune_antibodies.copy_from_slice(&values);
            }
            if let Some(values) = optional_array_byte(immune, "shape_antibody", IMMUNE_POPULATION)?
            {
                being.immune_shape_antibody.copy_from_slice(&values);
            }
        }

        if !being.load_social_memory(entries)? {
            being.init_social_memory();
        }
        if !being.load_episodic_memory(entries)? {
            being.init_episodic_memory();
        }

        Ok(being)
    }

    fn native_transfer_object(&self) -> Vec<ObjectEntry> {
        let mut object = Vec::new();
        object_string(&mut object, "name", &self.name);
        object_object(&mut object, "delta", self.native_delta_object());
        object_object(&mut object, "constant", self.native_constant_object());
        object_object(&mut object, "events", self.native_events_object());
        object_object(&mut object, "changes", self.native_changes_object());
        object_object(&mut object, "braindata", self.native_brain_object());
        object_object(&mut object, "immune_system", self.native_immune_object());
        object
    }

    #[allow(dead_code)]
    fn legacy_transfer_object(&self) -> Vec<ObjectEntry> {
        let mut object = Vec::new();
        object_string(&mut object, "name", &self.name);
        object_number(&mut object, "gender name", self.gender_name.into());
        object_number(&mut object, "family name", self.family_name.into());
        object_number(&mut object, "date of birth", self.date_of_birth.into());
        object_number(&mut object, "generation min", self.generation_min.into());
        object_number(&mut object, "generation max", self.generation_max.into());
        object_number(&mut object, "facing", self.facing.into());
        object_array_byte2(&mut object, "random seed", &self.random_seed);
        object_number(&mut object, "energy", self.energy.into());
        object_number(&mut object, "speed", self.speed().into());
        object_number(&mut object, "macro state", self.macro_state.into());
        object_number(&mut object, "parasites", self.parasites.into());
        object_number(&mut object, "honor", self.honor.into());
        object_number(&mut object, "crowding", self.crowding.into());
        object_number(&mut object, "height", self.height.into());
        object_number(&mut object, "mass", self.mass.into());
        object_number(&mut object, "posture", self.posture.into());
        object_number(&mut object, "awake", u8::from(self.awake).into());

        object_array_byte2(&mut object, "location", &self.location);
        object_array_byte2(&mut object, "goal", &self.goal);
        object_array_byte2(&mut object, "social coord", &self.social_coord);

        let mut genetics = Vec::new();
        for gene in self.genetics {
            array_add(&mut genetics, array_number(gene.into()));
        }
        object_array(&mut object, "genetics", genetics);

        object_array_byte(&mut object, "drives", &self.drives);
        object_array_byte(&mut object, "braincode register", &self.braincode_register);
        object_array_byte2(&mut object, "immune seed", &self.immune_seed);
        object
    }

    fn native_delta_object(&self) -> Vec<ObjectEntry> {
        let mut delta = Vec::new();
        object_number(&mut delta, "direction_facing", self.facing.into());
        object_array_byte2(&mut delta, "location", &self.location);
        object_array_byte(&mut delta, "velocity", &self.velocity);
        object_number(&mut delta, "stored_energy", self.energy.into());
        object_array_byte2(&mut delta, "random_seed", &self.random_seed);
        object_number(&mut delta, "macro_state", self.macro_state.into());
        object_number(&mut delta, "parasites", self.parasites.into());
        object_number(&mut delta, "honor", self.honor.into());
        object_number(&mut delta, "crowding", self.crowding.into());
        object_number(&mut delta, "height", self.height.into());
        object_number(&mut delta, "mass", self.mass.into());
        object_number(&mut delta, "posture", self.posture.into());
        object_array_byte2(&mut delta, "goal", &self.goal);
        object_array_byte2(&mut delta, "social_coord", &self.social_coord);
        object_number(&mut delta, "awake", self.awake_level.into());
        delta
    }

    fn native_constant_object(&self) -> Vec<ObjectEntry> {
        let mut constant = Vec::new();
        object_number(&mut constant, "date_of_birth", self.date_of_birth.into());

        let mut genetics = Vec::new();
        for gene in self.genetics {
            array_add(&mut genetics, array_number(gene.into()));
        }
        object_array(&mut constant, "genetics", genetics);

        let generation = [self.generation_min, self.generation_max];
        let identity = [self.gender_name, self.family_name];
        object_array_byte2(&mut constant, "generation_range", &generation);
        object_array_byte2(&mut constant, "name", &identity);
        constant
    }

    fn native_changes_object(&self) -> Vec<ObjectEntry> {
        let mut changes = Vec::new();
        object_array_byte(&mut changes, "drives", &self.drives);
        object_array_byte(&mut changes, "shout", &self.shout);
        object_array_byte2(&mut changes, "inventory", &self.inventory);
        object_array_byte(&mut changes, "learned_preference", &self.learned_preference);
        object_number(
            &mut changes,
            "date_of_conception",
            self.date_of_conception.into(),
        );
        object_array_genetics(&mut changes, "fetal_genetics", &self.fetal_genetics);
        object_array_byte2(&mut changes, "father_name", &self.father_name);
        object_array_byte2(&mut changes, "mother_name", &self.mother_name);
        let child_generation = [self.child_generation_min, self.child_generation_max];
        object_array_byte2(&mut changes, "child_generation_range", &child_generation);
        changes
    }

    fn native_events_object(&self) -> Vec<ObjectEntry> {
        let mut events = Vec::new();
        object_array(&mut events, "social", self.native_social_array());
        object_array(&mut events, "episodic", self.native_episodic_array());
        events
    }

    fn native_brain_object(&self) -> Vec<ObjectEntry> {
        let mut brain = Vec::new();
        object_array_byte(&mut brain, "braincode_register", &self.braincode_register);
        object_array(&mut brain, "brainprobe", self.native_brainprobe_array());
        object_array_byte2(&mut brain, "brain_state", &self.brain_state);
        object_number(&mut brain, "script_overrides", self.script_overrides.into());
        object_array_byte(&mut brain, "attention", &self.attention);
        brain
    }

    fn native_immune_object(&self) -> Vec<ObjectEntry> {
        let mut immune = Vec::new();
        object_array_byte(&mut immune, "antigens", &self.immune_antigens);
        object_array_byte(&mut immune, "shape_antigen", &self.immune_shape_antigen);
        object_array_byte(&mut immune, "antibodies", &self.immune_antibodies);
        object_array_byte(&mut immune, "shape_antibody", &self.immune_shape_antibody);
        object_array_byte2(&mut immune, "random_seed", &self.immune_seed);
        immune
    }

    fn native_social_array(&self) -> Vec<ObjectValue> {
        self.social_memory
            .iter()
            .map(|entry| ObjectValue::Object(social_entry_to_object(entry)))
            .collect()
    }

    fn native_episodic_array(&self) -> Vec<ObjectValue> {
        self.episodic_memory
            .iter()
            .map(|entry| ObjectValue::Object(episodic_entry_to_object(entry)))
            .collect()
    }

    fn native_brainprobe_array(&self) -> Vec<ObjectValue> {
        self.brainprobe
            .iter()
            .map(|entry| ObjectValue::Object(brainprobe_entry_to_object(entry)))
            .collect()
    }

    fn load_social_memory(&mut self, entries: &[ObjectEntry]) -> Result<bool, &'static str> {
        let social = if let Some(events) = optional_object(entries, "events")? {
            optional_field(events, "social")
        } else {
            optional_field(entries, "social")
        };

        let Some(social) = social else {
            return Ok(false);
        };

        let ObjectValue::Array(values) = social else {
            return Err("social array expected");
        };

        for (index, value) in values.iter().take(SOCIAL_SIZE).enumerate() {
            let ObjectValue::Object(entries) = value else {
                return Err("social entry object expected");
            };
            self.social_memory[index] = social_entry_from_object(entries)?;
        }
        Ok(true)
    }

    fn load_episodic_memory(&mut self, entries: &[ObjectEntry]) -> Result<bool, &'static str> {
        let episodic = if let Some(events) = optional_object(entries, "events")? {
            optional_field(events, "episodic")
        } else {
            optional_field(entries, "episodic")
        };

        let Some(episodic) = episodic else {
            return Ok(false);
        };

        let ObjectValue::Array(values) = episodic else {
            return Err("episodic array expected");
        };

        self.init_episodic_memory();
        for (index, value) in values.iter().take(EPISODIC_SIZE).enumerate() {
            let ObjectValue::Object(entries) = value else {
                return Err("episodic entry object expected");
            };
            self.episodic_memory[index] = episodic_entry_from_object(entries)?;
        }
        Ok(true)
    }

    pub fn to_simulated_being(&self) -> simulated_being {
        let mut being = simulated_being::default();
        being.constant.date_of_birth = self.date_of_birth;
        being.constant.generation_min = self.generation_min;
        being.constant.generation_max = self.generation_max;
        being.constant.name = [self.gender_name, self.family_name];
        being.constant.genetics = self.genetics;
        being.delta.location = self.location;
        being.delta.direction_facing = self.facing;
        being.delta.velocity = self.velocity;
        being.delta.stored_energy = self.energy;
        being.delta.random_seed = self.random_seed;
        being.delta.macro_state = self.macro_state;
        being.delta.parasites = self.parasites;
        being.delta.honor = self.honor;
        being.delta.crowding = self.crowding;
        being.delta.height = self.height;
        being.delta.mass = self.mass;
        being.delta.posture = self.posture;
        being.delta.goal = self.goal;
        being.delta.social_coord_x = self.social_coord[0];
        being.delta.social_coord_y = self.social_coord[1];
        being.delta.social_coord_nx = self.social_coord[2];
        being.delta.social_coord_ny = self.social_coord[3];
        being.delta.awake = self.awake_level;
        being.events.social = self.social_memory;
        being.events.episodic = self.episodic_memory;
        being.changes.drives = self.drives;
        being.changes.shout = self.shout;
        being.changes.inventory = self.inventory;
        being.changes.learned_preference = self.learned_preference;
        being.changes.date_of_conception = self.date_of_conception;
        being.changes.fetal_genetics = self.fetal_genetics;
        being.changes.father_name = self.father_name;
        being.changes.mother_name = self.mother_name;
        being.changes.child_generation_min = self.child_generation_min;
        being.changes.child_generation_max = self.child_generation_max;
        being.braindata.braincode_register = self.braincode_register;
        being.braindata.brainprobe = self.brainprobe;
        being.braindata.brain_state = self.brain_state;
        being.braindata.script_overrides = self.script_overrides;
        being.braindata.attention = self.attention;
        being.immune_system.antigens = self.immune_antigens;
        being.immune_system.shape_antigen = self.immune_shape_antigen;
        being.immune_system.antibodies = self.immune_antibodies;
        being.immune_system.shape_antibody = self.immune_shape_antibody;
        being.immune_system.random_seed = self.immune_seed;
        being
    }

    pub fn from_simulated_being(name: String, native: &simulated_being) -> Self {
        Self {
            name,
            gender_name: native.constant.name[0],
            family_name: native.constant.name[1],
            date_of_birth: native.constant.date_of_birth,
            generation_min: native.constant.generation_min,
            generation_max: native.constant.generation_max,
            genetics: native.constant.genetics,
            location: native.delta.location,
            facing: native.delta.direction_facing,
            random_seed: native.delta.random_seed,
            energy: native.delta.stored_energy,
            velocity: native.delta.velocity,
            macro_state: native.delta.macro_state,
            parasites: native.delta.parasites,
            honor: native.delta.honor,
            crowding: native.delta.crowding,
            height: native.delta.height,
            mass: native.delta.mass,
            posture: native.delta.posture,
            goal: native.delta.goal,
            social_coord: [
                native.delta.social_coord_x,
                native.delta.social_coord_y,
                native.delta.social_coord_nx,
                native.delta.social_coord_ny,
            ],
            awake: native.delta.awake != FULLY_ASLEEP,
            awake_level: native.delta.awake,
            drives: native.changes.drives,
            braincode_register: native.braindata.braincode_register,
            attention: native.braindata.attention,
            brainprobe: native.braindata.brainprobe,
            brain_state: native.braindata.brain_state,
            script_overrides: native.braindata.script_overrides,
            shout: native.changes.shout,
            inventory: native.changes.inventory,
            learned_preference: native.changes.learned_preference,
            date_of_conception: native.changes.date_of_conception,
            fetal_genetics: native.changes.fetal_genetics,
            father_name: native.changes.father_name,
            mother_name: native.changes.mother_name,
            child_generation_min: native.changes.child_generation_min,
            child_generation_max: native.changes.child_generation_max,
            social_memory: native.events.social,
            episodic_memory: native.events.episodic,
            immune_antigens: native.immune_system.antigens,
            immune_shape_antigen: native.immune_system.shape_antigen,
            immune_antibodies: native.immune_system.antibodies,
            immune_shape_antibody: native.immune_system.shape_antibody,
            immune_seed: native.immune_system.random_seed,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn gender_name(&self) -> n_byte2 {
        self.gender_name
    }

    pub const fn family_name(&self) -> n_byte2 {
        self.family_name
    }

    pub const fn date_of_birth(&self) -> n_byte4 {
        self.date_of_birth
    }

    pub const fn genetics(&self) -> [n_genetics; CHROMOSOMES] {
        self.genetics
    }

    pub const fn location(&self) -> [n_byte2; 2] {
        self.location
    }

    pub const fn facing(&self) -> n_byte {
        self.facing
    }

    pub const fn random_seed(&self) -> [n_byte2; 2] {
        self.random_seed
    }

    pub const fn energy(&self) -> n_byte2 {
        self.energy
    }

    pub const fn speed(&self) -> n_byte {
        self.velocity[0]
    }

    pub const fn velocity(&self) -> [n_byte; 10] {
        self.velocity
    }

    pub fn ten_minute_distance(&self) -> n_int {
        self.velocity.iter().map(|speed| n_int::from(*speed)).sum()
    }

    pub const fn macro_state(&self) -> n_byte2 {
        self.macro_state
    }

    pub const fn parasites(&self) -> n_byte {
        self.parasites
    }

    pub const fn honor(&self) -> n_byte {
        self.honor
    }

    pub const fn crowding(&self) -> n_byte {
        self.crowding
    }

    pub const fn height(&self) -> n_byte2 {
        self.height
    }

    pub const fn mass(&self) -> n_byte2 {
        self.mass
    }

    pub const fn posture(&self) -> n_byte {
        self.posture
    }

    pub const fn goal(&self) -> [n_byte2; 4] {
        self.goal
    }

    pub const fn social_coord(&self) -> [n_byte2; 4] {
        self.social_coord
    }

    pub const fn awake(&self) -> bool {
        self.awake
    }

    pub const fn awake_level(&self) -> n_byte {
        self.awake_level
    }

    pub const fn drives(&self) -> [n_byte; DRIVES] {
        self.drives
    }

    pub const fn braincode_register(&self) -> [n_byte; BRAINCODE_PSPACE_REGISTERS] {
        self.braincode_register
    }

    pub const fn attention(&self) -> [n_byte; ATTENTION_SIZE] {
        self.attention
    }

    pub const fn brainprobe(&self) -> [simulated_ibrain_probe; BRAINCODE_PROBES] {
        self.brainprobe
    }

    pub const fn brain_state(&self) -> [n_byte2; 6] {
        self.brain_state
    }

    pub const fn script_overrides(&self) -> n_byte2 {
        self.script_overrides
    }

    pub const fn body_attention(&self) -> n_byte {
        self.attention[ATTENTION_BODY]
    }

    pub fn body_attention_description(&self) -> &'static str {
        body_inventory_description(self.body_attention())
    }

    pub const fn social_memory(&self) -> [simulated_isocial; SOCIAL_SIZE] {
        self.social_memory
    }

    pub fn social_memory_mut(&mut self) -> &mut [simulated_isocial; SOCIAL_SIZE] {
        &mut self.social_memory
    }

    pub const fn episodic_memory(&self) -> [simulated_iepisodic; EPISODIC_SIZE] {
        self.episodic_memory
    }

    pub fn episodic_memory_mut(&mut self) -> &mut [simulated_iepisodic; EPISODIC_SIZE] {
        &mut self.episodic_memory
    }

    pub const fn shout(&self) -> [n_byte; SHOUT_BYTES] {
        self.shout
    }

    pub const fn inventory(&self) -> [n_byte2; INVENTORY_SIZE] {
        self.inventory
    }

    pub const fn learned_preference(&self) -> [n_byte; PREFERENCES] {
        self.learned_preference
    }

    pub const fn immune_seed(&self) -> [n_byte2; 2] {
        self.immune_seed
    }

    pub const fn immune_antigens(&self) -> [n_byte; IMMUNE_ANTIGENS] {
        self.immune_antigens
    }

    pub const fn immune_shape_antigen(&self) -> [n_byte; IMMUNE_ANTIGENS] {
        self.immune_shape_antigen
    }

    pub const fn immune_antibodies(&self) -> [n_byte; IMMUNE_POPULATION] {
        self.immune_antibodies
    }

    pub const fn immune_shape_antibody(&self) -> [n_byte; IMMUNE_POPULATION] {
        self.immune_shape_antibody
    }

    pub const fn is_female(&self) -> bool {
        ((self.gender_name >> 8) as n_byte) == SEX_FEMALE
    }

    pub fn state_description(&self) -> String {
        being_state_description(self.macro_state)
    }

    pub fn drive_description(&self) -> &'static str {
        drive_description(self.drives)
    }

    pub fn real_height_mm(&self) -> n_byte2 {
        ((n_uint::from(self.height) * n_uint::from(BEING_MAX_HEIGHT_MM))
            / n_uint::from(BEING_MAX_HEIGHT)) as n_byte2
    }

    pub fn body_frame(&self) -> n_byte {
        gene_frame(self.genetics)
    }

    pub fn body_fat(&self) -> n_byte2 {
        body_fat(self.genetics, self.energy)
    }

    pub fn facing_vector(&self, divisor: n_int) -> n_vect2 {
        let divisor = divisor.max(1) * 32;
        vect2_direction(n_int::from(self.facing), divisor)
    }

    pub fn set_facing_towards(&mut self, vector: n_vect2) {
        self.facing = (math_tan(vector) & 255) as n_byte;
    }

    pub fn wander(&mut self, wander: n_int) {
        self.facing = ((n_int::from(self.facing) + 256 + wander) & 255) as n_byte;
    }

    pub fn set_speed(&mut self, speed: n_byte) {
        self.velocity[0] = speed;
    }

    pub fn speed_advance(&mut self) {
        for index in (1..self.velocity.len()).rev() {
            self.velocity[index] = self.velocity[index - 1];
        }
    }

    pub fn energy_less_than(&self, less_than: n_byte2) -> bool {
        self.energy < less_than
    }

    pub fn energy_delta(&mut self, delta: i32) {
        self.energy = (i32::from(self.energy) + delta)
            .clamp(i32::from(BEING_DEAD), i32::from(n_byte2::MAX)) as n_byte2;
    }

    pub fn drive(&self, drive: usize) -> n_byte {
        self.drives[drive]
    }

    pub fn inc_drive(&mut self, drive: usize) {
        self.drives[drive] = self.drives[drive].saturating_add(1);
    }

    pub fn dec_drive(&mut self, drive: usize) {
        self.drives[drive] = self.drives[drive].saturating_sub(1);
    }

    pub fn reset_drive(&mut self, drive: usize) {
        self.drives[drive] = 0;
    }

    fn advance_minute(&mut self, land: &mut LandState) {
        let land_date = land.date();
        let land_time = land.time();
        self.awake_level = self.awake_level_for_time(land_time);
        self.awake = self.awake_level != FULLY_ASLEEP;
        self.cycle_universal();

        if self.energy == BEING_DEAD || self.awake_level == FULLY_ASLEEP {
            self.set_speed(0);
            return;
        }

        self.cycle_awake(land);
        self.cycle_episodic();
        self.cycle_drives(land_date);
        self.speed_advance();

        if land_time == 0 {
            self.honor = self.honor.saturating_add(1);
        }
    }

    fn awake_level_for_time(&self, land_time: n_byte4) -> n_byte {
        if self.energy_less_than(BEING_DEAD + 1) {
            return FULLY_ASLEEP;
        }
        if !is_night(land_time) {
            return FULLY_AWAKE;
        }
        if self.energy_less_than(BEING_HUNGRY + 1) || self.speed() > 0 {
            return SLIGHTLY_AWAKE;
        }
        FULLY_ASLEEP
    }

    fn cycle_universal(&mut self) {
        let immune_energy_used = self.immune_response();
        if immune_energy_used > 0 {
            self.energy_delta(-immune_energy_used);
        }
        if self.awake_level == FULLY_ASLEEP {
            self.macro_state = BEING_STATE_ASLEEP;
            self.reset_drive(DRIVE_FATIGUE);
        }
        if self.energy == BEING_DEAD {
            self.awake_level = FULLY_ASLEEP;
            self.awake = false;
            self.macro_state = BEING_STATE_ASLEEP;
            self.set_speed(0);
        }
    }

    fn cycle_awake(&mut self, land: &mut LandState) {
        let land_date = land.date();
        let land_time = land.time();
        let terrain = land.terrain_sample(self.location);
        let mut state = BEING_STATE_AWAKE;
        if terrain.water {
            state |= BEING_STATE_SWIMMING;
        }
        if self.energy_less_than(BEING_HUNGRY + 1) {
            state |= BEING_STATE_HUNGRY;
        }

        let (mut target_speed, water_ahead) = self.temporary_speed(land);
        if state & BEING_STATE_SWIMMING != 0 || water_ahead {
            self.turn_away_from_water(land);
            target_speed = (target_speed * (n_int::from(gene_swim(self.genetics)) + 8)) >> 4;
            state |= BEING_STATE_SWIMMING;
            self.posture = 0;
            for (index, item) in self.inventory.iter_mut().enumerate() {
                if index != usize::from(BODY_HEAD) && index != usize::from(BODY_BACK) {
                    *item = 0;
                }
            }
            self.parasites = self.parasites.saturating_sub(1);
        } else {
            target_speed = (target_speed * (n_int::from(gene_speed(self.genetics)) + 8)) >> 3;
        }

        if state & BEING_STATE_HUNGRY != 0 {
            if self.speed() == 0 {
                let food = self.eat_food(land);
                if food.energy > 0 {
                    self.energy_delta(i32::from(food.energy));
                    self.record_episodic_food(food.energy, food.food_type, land_date, land_time);
                    self.reset_drive(DRIVE_HUNGER);
                    state |= BEING_STATE_EATING;
                    if age_days_at(land_date, self.date_of_birth) < AGE_OF_MATURITY
                        && self.height < BEING_MAX_HEIGHT
                    {
                        self.height = self
                            .height
                            .saturating_add(energy_to_growth(food.energy))
                            .min(BEING_MAX_HEIGHT);
                    }
                } else {
                    state |= BEING_STATE_NO_FOOD;
                }
            } else {
                target_speed = 0;
            }
        } else if self.speed() == 0 {
            target_speed = target_speed.max(10);
        }

        self.calculate_speed(target_speed, state);
        self.genetic_wandering();
        self.move_forward();
        let energy_cost = self.move_energy(land);
        if energy_cost > 0 {
            self.energy_delta(-energy_cost);
        }
        if self.speed() > 0 {
            state |= BEING_STATE_MOVING;
        }
        self.macro_state = state;
        self.mass = self.calculated_mass();
    }

    fn temporary_speed(&self, land: &LandState) -> (n_int, bool) {
        let facing_vector = self.facing_vector(4);
        let terrain = land.terrain_sample(self.location);
        let looking = [
            wrap_apespace(n_int::from(self.location[0]) + facing_vector.x),
            wrap_apespace(n_int::from(self.location[1]) + facing_vector.y),
        ];
        let water_ahead = land.terrain_sample(looking).water;
        let delta_z = vect2_dot(terrain.slope, facing_vector, 1, 24);
        (((delta_z + 280) >> 4), water_ahead)
    }

    fn calculate_speed(&mut self, target_speed: n_int, state: n_byte2) {
        let mut target_speed = target_speed.clamp(0, 39);
        let mut speed = n_int::from(self.speed());
        if self.awake_level != FULLY_AWAKE
            || ((state & (BEING_STATE_HUNGRY | BEING_STATE_NO_FOOD)) == BEING_STATE_HUNGRY)
        {
            if state & BEING_STATE_NO_FOOD == 0 {
                target_speed = 0;
            }
        }

        if target_speed > speed {
            speed += 1;
        }
        for _ in 0..3 {
            if target_speed < speed {
                speed -= 1;
            }
        }
        self.set_speed(speed.clamp(0, 39) as n_byte);
    }

    fn move_forward(&mut self) {
        let speed = n_int::from(self.speed());
        if speed == 0 {
            return;
        }
        let facing_vector = self.facing_vector(1);
        let dx = (facing_vector.x * speed) / 512;
        let dy = (facing_vector.y * speed) / 512;
        self.location[0] = wrap_apespace(n_int::from(self.location[0]) + dx);
        self.location[1] = wrap_apespace(n_int::from(self.location[1]) + dy);
    }

    fn move_energy(&self, land: &LandState) -> i32 {
        let speed = n_int::from(self.speed());
        let terrain = land.terrain_sample(self.location);
        let facing_vector = self.facing_vector(1);
        let delta_z = vect2_dot(terrain.slope, facing_vector, 1, 96);
        let mut delta_energy = ((512 - delta_z) * speed) / 80;

        if terrain.water {
            delta_energy = (delta_energy * delta_energy) >> 9;
            let insulation = (n_int::from(self.body_fat().min(BEING_MAX_MASS_FAT_G)) * 5)
                / n_int::from(BEING_MAX_MASS_FAT_G);
            ((delta_energy + 10 - insulation).max(0) >> 3) as i32
        } else {
            if delta_z > 0 {
                delta_energy += n_int::from(gene_hill_climb(self.genetics));
            }
            delta_energy = (delta_energy * delta_energy) >> 9;
            ((delta_energy + 4 + ((n_int::from(self.mass) * 5) / n_int::from(BEING_MAX_MASS_G)))
                >> 2) as i32
        }
    }

    fn eat_food(&mut self, land: &mut LandState) -> FoodSample {
        if let Some(source) = self.consume_inventory_food() {
            return source;
        }
        let source = land.food_source_at(self.location);
        let energy = self.food_absorption(source.max_energy, source.food_type);
        if energy > BEING_DEAD {
            land.consume_food_at(self.location, energy);
        }
        FoodSample { energy, ..source }
    }

    fn consume_inventory_food(&mut self) -> Option<FoodSample> {
        let slot = self
            .inventory
            .iter()
            .position(|item| inventory_food_source(*item).is_some())?;
        let source = inventory_food_source(self.inventory[slot])?;
        let energy = self.food_absorption(source.max_energy, source.food_type);
        if energy > BEING_DEAD {
            self.inventory[slot] = 0;
        }
        Some(FoodSample { energy, ..source })
    }

    fn food_absorption(&mut self, max_energy: n_byte2, food_type: n_byte) -> n_byte2 {
        if max_energy == BEING_DEAD {
            self.immune_ingest_pathogen(food_type);
            return 0;
        }
        let vegetable = gene_energy_from_vegetables(self.genetics);
        let fruit = gene_energy_from_fruits(self.genetics);
        let shellfish = gene_energy_from_shellfish(self.genetics);
        let seaweed = gene_energy_from_seaweed(self.genetics);
        let bird_eggs = gene_energy_from_bird_eggs(self.genetics);
        let lizard_eggs = gene_energy_from_lizard_eggs(self.genetics);
        let denominator = 1 + vegetable + fruit + seaweed + bird_eggs + lizard_eggs;
        self.immune_ingest_pathogen(food_type);
        let absorbed = match food_type {
            FOOD_VEGETABLE => (vegetable << 4) / denominator,
            FOOD_FRUIT => (fruit << 4) / denominator,
            FOOD_SHELLFISH => (shellfish << 4) / denominator,
            FOOD_SEAWEED => (seaweed << 4) / denominator,
            FOOD_BIRD_EGGS => (bird_eggs << 4) / denominator,
            FOOD_LIZARD_EGGS => (lizard_eggs << 4) / denominator,
            _ => return 0,
        };
        ((n_uint::from(max_energy) * n_uint::from(1 + absorbed)) >> 3).min(320) as n_byte2
    }

    fn turn_away_from_water(&mut self, land: &LandState) {
        let base_location = self.location;
        for water_turn in 0..7 {
            let turn = 8 - water_turn;
            let plus = ((n_int::from(self.facing) + turn) & 255) as n_byte;
            let minus = ((n_int::from(self.facing) + 256 - turn) & 255) as n_byte;
            let plus_height = land.height_at(lookahead_location(base_location, plus));
            let minus_height = land.height_at(lookahead_location(base_location, minus));
            if minus_height > plus_height {
                self.wander(-turn);
            } else if minus_height < plus_height {
                self.wander(turn);
            }
        }
    }

    fn genetic_wandering(&mut self) {
        if self.goal != [0; 4] {
            return;
        }
        let threshold = 1_000 + (3_600 * n_int::from(gene_stagger(self.genetics)));
        if n_int::from(math_random(&mut self.random_seed)) < threshold {
            let value = (math_random(&mut self.random_seed) & 7) as n_byte;
            self.wander(math_spread_byte(value));
        }
    }

    fn calculated_mass(&self) -> n_byte2 {
        let lean_mass = (n_uint::from(BEING_MAX_MASS_G) * n_uint::from(self.height))
            / n_uint::from(BEING_MAX_HEIGHT);
        (lean_mass + n_uint::from(self.body_fat())).min(n_uint::from(n_byte2::MAX)) as n_byte2
    }

    fn cycle_drives(&mut self, land_date: n_byte4) {
        if self.energy_less_than(BEING_HUNGRY) {
            self.inc_drive(DRIVE_HUNGER);
            self.dec_drive(DRIVE_SEX);
        } else {
            self.dec_drive(DRIVE_HUNGER);
        }

        if self.crowding < MAX_CROWDING {
            self.inc_drive(DRIVE_SOCIAL);
            self.crowding = self.crowding.saturating_add(1).min(MAX_CROWDING);
        } else if self.crowding > MIN_CROWDING {
            self.dec_drive(DRIVE_SOCIAL);
            self.crowding = self.crowding.saturating_sub(1).max(MIN_CROWDING);
        }

        if age_days_at(land_date, self.date_of_birth) > AGE_OF_MATURITY {
            if self.awake_level != FULLY_ASLEEP {
                self.inc_drive(DRIVE_SEX);
            } else {
                self.dec_drive(DRIVE_SEX);
            }
            if self.drive(DRIVE_SEX) < THRESHOLD_SEEK_MATE && self.goal[0] != 0 {
                self.goal = [0; 4];
            }
        }

        if self.speed() > FATIGUE_SPEED_THRESHOLD {
            self.inc_drive(DRIVE_FATIGUE);
            if self.macro_state & BEING_STATE_SWIMMING != 0 {
                self.inc_drive(DRIVE_FATIGUE);
            }
            self.dec_drive(DRIVE_SEX);
        } else {
            self.dec_drive(DRIVE_FATIGUE);
        }
    }

    fn init_social_memory(&mut self) {
        self.social_memory = [simulated_isocial::default(); SOCIAL_SIZE];
        for entry in &mut self.social_memory {
            entry.entity_type = ENTITY_BEING;
            entry.friend_foe = SOCIAL_RESPECT_NORMAL;
        }
        self.social_memory[0].relationship = RELATIONSHIP_SELF;
    }

    fn init_episodic_memory(&mut self) {
        self.episodic_memory = [simulated_iepisodic::default(); EPISODIC_SIZE];
        for entry in &mut self.episodic_memory {
            entry.affect = EPISODIC_AFFECT_ZERO;
        }
    }

    fn ensure_social_self(&mut self) {
        self.social_memory[0].relationship = RELATIONSHIP_SELF;
        self.social_memory[0].entity_type = ENTITY_BEING;
        self.social_memory[0].friend_foe = SOCIAL_RESPECT_NORMAL;
    }

    fn social_index_for(&self, gender_name: n_byte2, family_name: n_byte2) -> Option<usize> {
        self.social_memory
            .iter()
            .take(SOCIAL_SIZE_BEINGS)
            .position(|entry| {
                entry.first_name[BEING_MET] == gender_name
                    && entry.family_name[BEING_MET] == family_name
            })
    }

    fn social_replacement_index(&self) -> usize {
        self.social_memory
            .iter()
            .take(SOCIAL_SIZE_BEINGS)
            .enumerate()
            .skip(1)
            .find(|(_, entry)| social_entry_empty(entry))
            .map(|(index, _)| index)
            .unwrap_or_else(|| {
                self.social_memory
                    .iter()
                    .take(SOCIAL_SIZE_BEINGS)
                    .enumerate()
                    .skip(1)
                    .min_by_key(|(_, entry)| entry.familiarity)
                    .map(|(index, _)| index)
                    .unwrap_or(1)
            })
    }

    fn initial_friend_foe_for(&self, other: &BeingSummary) -> n_byte {
        (n_int::from(SOCIAL_RESPECT_NORMAL) - self.social_attraction_pheromone(other)
            + self.social_attraction_pigmentation(other)
            + self.social_attraction_height(other)
            + self.social_attraction_frame(other)
            + self.social_attraction_hair(other))
        .clamp(0, 255) as n_byte
    }

    fn social_attraction_score(&self, other: &BeingSummary) -> n_int {
        1 + self.social_attraction_pheromone(other)
            + self.social_attraction_pigmentation(other)
            + self.social_attraction_height(other)
            + self.social_attraction_frame(other)
            + self.social_attraction_hair(other)
    }

    fn social_attraction_pheromone(&self, other: &BeingSummary) -> n_int {
        let different = self
            .genetics
            .iter()
            .zip(other.genetics.iter())
            .map(|(left, right)| (left ^ right).count_ones() as n_int)
            .sum::<n_int>();
        if different < MINIMUM_GENETIC_VARIATION {
            -n_int::from(gene_incest_aversion(self.genetics))
        } else {
            1
        }
    }

    fn social_attraction_pigmentation(&self, other: &BeingSummary) -> n_int {
        let offset = usize::from(self.is_female());
        let preference = nature_nurture(
            gene_pigmentation_preference(self.genetics),
            self.learned_preference[PREFERENCE_MATE_PIGMENTATION_MALE + offset],
        );
        attraction_from_difference(n_int::from(gene_pigmentation(other.genetics)), preference)
    }

    fn social_attraction_hair(&self, other: &BeingSummary) -> n_int {
        let offset = usize::from(self.is_female());
        let preference = nature_nurture(
            gene_hair_preference(self.genetics),
            self.learned_preference[PREFERENCE_MATE_HAIR_MALE + offset],
        );
        attraction_from_difference(n_int::from(gene_hair(other.genetics)), preference)
    }

    fn social_attraction_height(&self, other: &BeingSummary) -> n_int {
        let offset = usize::from(self.is_female());
        let preference = nature_nurture(
            gene_height_preference(self.genetics),
            self.learned_preference[PREFERENCE_MATE_HEIGHT_MALE + offset],
        );
        if preference >= 12 && other.height > self.height {
            1
        } else if (8..12).contains(&preference) && other.height < self.height {
            1
        } else {
            0
        }
    }

    fn social_attraction_frame(&self, other: &BeingSummary) -> n_int {
        let offset = usize::from(self.is_female());
        let preference = nature_nurture(
            gene_frame_preference(self.genetics),
            self.learned_preference[PREFERENCE_MATE_FRAME_MALE + offset],
        );
        if (7..=11).contains(&preference) && other.body_fat() > self.body_fat() {
            1
        } else if preference > 11 && other.body_fat() < self.body_fat() {
            1
        } else {
            0
        }
    }

    fn meet_being(
        &mut self,
        other: &BeingSummary,
        land_date: n_byte4,
        land_time: n_byte4,
    ) -> usize {
        self.ensure_social_self();
        let index = self
            .social_index_for(other.gender_name, other.family_name)
            .unwrap_or_else(|| self.social_replacement_index());
        let initial_friend_foe = self.initial_friend_foe_for(other);
        let entry = &mut self.social_memory[index];
        let newly_met = social_entry_empty(entry);
        social_observe_features(entry, other);
        entry.entity_type = ENTITY_BEING;
        entry.first_name[BEING_MEETER] = self.gender_name;
        entry.family_name[BEING_MEETER] = self.family_name;
        entry.first_name[BEING_MET] = other.gender_name;
        entry.family_name[BEING_MET] = other.family_name;
        entry.space_time.date = land_date;
        entry.space_time.time = land_time;
        entry.space_time.location = self.location;
        entry.belief = other.macro_state;
        entry.familiarity = entry.familiarity.saturating_add(1);
        if newly_met {
            entry.friend_foe = initial_friend_foe;
            entry.relationship = 0;
        }
        entry.friend_foe = entry.friend_foe.saturating_add(1);
        self.attention[ATTENTION_ACTOR] = index as n_byte;
        index
    }

    fn social_secondary_target(&self, beings: &[BeingSummary]) -> [n_byte2; 2] {
        let origin = [
            n_int::from(self.social_coord[0]),
            n_int::from(self.social_coord[1]),
        ];
        let mut sum = [0, 0];
        let mut count = 0;

        for entry in self.social_memory.iter().take(SOCIAL_SIZE_BEINGS).skip(1) {
            if social_entry_empty(entry) {
                continue;
            }
            let Some(other) = beings.iter().find(|being| {
                being.gender_name == entry.first_name[BEING_MET]
                    && being.family_name == entry.family_name[BEING_MET]
            }) else {
                continue;
            };
            let weight = n_int::from(entry.friend_foe) - n_int::from(SOCIAL_RESPECT_NORMAL);
            sum[0] += (n_int::from(other.social_coord[0]) - origin[0]) * weight;
            sum[1] += (n_int::from(other.social_coord[1]) - origin[1]) * weight;
            count += 1;
        }

        if count == 0 {
            return [self.social_coord[0], self.social_coord[1]];
        }

        [
            clamp_byte2(origin[0] + sum[0] / (count * 20)),
            clamp_byte2(origin[1] + sum[1] / (count * 20)),
        ]
    }

    fn social_memory_maintenance(&mut self) {
        self.ensure_social_self();
        if self
            .social_memory
            .iter()
            .take(SOCIAL_SIZE_BEINGS)
            .skip(1)
            .any(|entry| entry.familiarity > 65_534)
        {
            for entry in self
                .social_memory
                .iter_mut()
                .take(SOCIAL_SIZE_BEINGS)
                .skip(1)
            {
                entry.familiarity >>= 2;
            }
        }

        let respect_mean = self.social_respect_mean();
        for entry in self
            .social_memory
            .iter_mut()
            .take(SOCIAL_SIZE_BEINGS)
            .skip(1)
        {
            if social_entry_empty(entry) {
                continue;
            }
            if entry.familiarity > 16 && entry.friend_foe < respect_mean {
                entry.friend_foe = entry.friend_foe.saturating_add(1);
            } else if entry.familiarity == 0 && entry.friend_foe > SOCIAL_RESPECT_NORMAL {
                entry.friend_foe = entry.friend_foe.saturating_sub(1);
            }
        }
    }

    fn social_respect_mean(&self) -> n_byte {
        let mut total = n_uint::from(SOCIAL_RESPECT_NORMAL);
        let mut count = 1;
        for entry in self.social_memory.iter().take(SOCIAL_SIZE_BEINGS).skip(1) {
            if social_entry_empty(entry) {
                continue;
            }
            total += n_uint::from(entry.friend_foe);
            count += 1;
        }
        (total / count).min(255) as n_byte
    }

    fn record_episodic_food(
        &mut self,
        energy: n_byte2,
        food_type: n_byte,
        land_date: n_byte4,
        land_time: n_byte4,
    ) {
        self.record_episodic_full(
            EVENT_EAT,
            i32::from(energy),
            self.gender_name,
            self.family_name,
            0,
            0,
            0,
            food_type,
            land_date,
            land_time,
        );
    }

    fn record_episodic_interaction(
        &mut self,
        other: &BeingSummary,
        event: n_byte,
        affect: i32,
        arg: n_byte2,
        land_date: n_byte4,
        land_time: n_byte4,
    ) {
        self.record_episodic_full(
            event,
            affect,
            self.gender_name,
            self.family_name,
            other.gender_name,
            other.family_name,
            arg,
            0,
            land_date,
            land_time,
        );
    }

    fn record_episodic_full(
        &mut self,
        event: n_byte,
        affect: i32,
        name1: n_byte2,
        family1: n_byte2,
        name2: n_byte2,
        family2: n_byte2,
        arg: n_byte2,
        food: n_byte,
        land_date: n_byte4,
        land_time: n_byte4,
    ) {
        let replace = self.episodic_replacement_index(event, name1, family1, name2, family2);
        let entry = &mut self.episodic_memory[replace];
        entry.event = event;
        entry.affect =
            (i32::from(EPISODIC_AFFECT_ZERO) + affect).clamp(0, i32::from(n_byte2::MAX)) as n_byte2;
        entry.space_time.date = land_date;
        entry.space_time.time = land_time;
        entry.space_time.location = self.location;
        entry.first_name = [name1, name2];
        entry.family_name = [family1, family2];
        entry.food = food;
        entry.arg = arg;
        self.attention[ATTENTION_EPISODE] = replace as n_byte;
    }

    fn episodic_replacement_index(
        &self,
        event: n_byte,
        name1: n_byte2,
        family1: n_byte2,
        name2: n_byte2,
        family2: n_byte2,
    ) -> usize {
        self.episodic_memory
            .iter()
            .position(|entry| {
                entry.event == event
                    && entry.first_name == [name1, name2]
                    && entry.family_name == [family1, family2]
            })
            .or_else(|| {
                self.episodic_memory
                    .iter()
                    .position(|entry| entry.event == 0)
            })
            .unwrap_or_else(|| {
                self.episodic_memory
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, entry)| affect_distance(entry.affect))
                    .map(|(index, _)| index)
                    .unwrap_or(0)
            })
    }

    fn cycle_episodic(&mut self) {
        if self.awake_level == FULLY_ASLEEP {
            return;
        }
        for entry in &mut self.episodic_memory {
            if entry.event == 0 {
                continue;
            }
            if entry.affect < EPISODIC_AFFECT_ZERO {
                entry.affect = entry.affect.saturating_add(1);
            } else if entry.affect > EPISODIC_AFFECT_ZERO {
                entry.affect = entry.affect.saturating_sub(1);
            }
        }
    }

    fn init_immune(&mut self) {
        let mut seed = self.immune_seed;
        for index in (0..IMMUNE_ANTIGENS).step_by(2) {
            self.immune_antigens[index] = 0;
            self.immune_antigens[index + 1] = 0;
            math_random3(&mut seed);
            self.immune_shape_antigen[index] = (seed[0] & 255) as n_byte;
            self.immune_shape_antigen[index + 1] = (seed[1] & 255) as n_byte;
        }
        for index in (0..IMMUNE_POPULATION).step_by(2) {
            self.immune_antibodies[index] = 0;
            self.immune_antibodies[index + 1] = 0;
            math_random3(&mut seed);
            self.immune_shape_antibody[index] = (seed[0] & 255) as n_byte;
            self.immune_shape_antibody[index + 1] = (seed[1] & 255) as n_byte;
        }
        self.immune_seed = seed;
    }

    fn immune_response(&mut self) -> i32 {
        math_random3(&mut self.immune_seed);
        if self.immune_seed[0] < ANTIBODY_DEPLETION_PROB {
            let index = self.immune_seed[1] as usize % IMMUNE_POPULATION;
            self.immune_antibodies[index] = self.immune_antibodies[index].saturating_sub(1);
        }

        math_random3(&mut self.immune_seed);
        let antigen_index = self.immune_seed[0] as usize % IMMUNE_ANTIGENS;
        if self.immune_antigens[antigen_index] != 0 {
            if self.immune_seed[1] < PATHOGEN_MUTATION_PROB {
                math_random3(&mut self.immune_seed);
                let bit = (self.immune_seed[0] & 7) as n_byte;
                if self.immune_shape_antigen[antigen_index] & (1 << bit) != 0 {
                    self.immune_shape_antigen[antigen_index] ^= bit;
                } else {
                    self.immune_shape_antigen[antigen_index] |= bit;
                }
            }

            let mut max_bits_matched = 0;
            let mut best_match = 0;
            for index in 0..IMMUNE_POPULATION {
                let antibody = self.immune_shape_antibody[index];
                let antigen = self.immune_shape_antigen[antigen_index];
                let matched = (antibody & antigen) | ((!antibody) & (!antigen));
                let bits_matched = matched.count_ones() as n_byte;
                if bits_matched > max_bits_matched {
                    max_bits_matched = bits_matched;
                    best_match = index;
                }
            }

            let mut min_index = 0;
            let mut min_antibodies = self.immune_antibodies[0];
            for index in 1..IMMUNE_POPULATION {
                if self.immune_antibodies[index] < min_antibodies {
                    min_antibodies = self.immune_antibodies[index];
                    min_index = index;
                }
            }

            if max_bits_matched > IMMUNE_FIT {
                self.immune_antibodies[best_match] =
                    self.immune_antibodies[best_match].saturating_add(max_bits_matched);
                if self.immune_antibodies[best_match] < MIN_ANTIBODIES {
                    self.immune_antibodies[best_match] = MIN_ANTIBODIES;
                }
                let honor_immune = self.honor_immune();
                self.immune_antigens[antigen_index] =
                    self.immune_antigens[antigen_index].saturating_sub(honor_immune);
                if min_index != best_match {
                    self.immune_antibodies[min_index] = 1;
                    let mut cloned = self.immune_shape_antibody[best_match];
                    math_random3(&mut self.immune_seed);
                    let bit = (self.immune_seed[0] & 7) as n_byte;
                    if cloned & (1 << bit) != 0 {
                        cloned ^= bit;
                    } else {
                        cloned |= bit;
                    }
                    self.immune_shape_antibody[min_index] = cloned;
                }
            } else {
                self.immune_antigens[antigen_index] =
                    self.immune_antigens[antigen_index].saturating_add(1);
                math_random3(&mut self.immune_seed);
                if self.immune_seed[0] < self.energy {
                    math_random3(&mut self.immune_seed);
                    self.immune_shape_antibody[min_index] = (self.immune_seed[0] & 255) as n_byte;
                    self.immune_antibodies[min_index] = (self.immune_seed[1] & 7) as n_byte;
                }
            }
        }

        let total_antigens: n_byte2 = self
            .immune_antigens
            .iter()
            .map(|value| n_byte2::from(*value))
            .sum();
        let max_severity = self.immune_shape_antigen.iter().copied().max().unwrap_or(0);
        math_random3(&mut self.immune_seed);
        if self.immune_seed[0] < (total_antigens >> 2) && self.energy >= 1 {
            i32::from(pathogen_severity(max_severity))
        } else {
            0
        }
    }

    fn immune_ingest_pathogen(&mut self, food_type: n_byte) {
        let transmission_type = food_type.saturating_add(PATHOGEN_TRANSMISSION_FOOD_VEGETABLE);
        self.immune_acquire_pathogen(transmission_type);
    }

    fn immune_acquire_pathogen(&mut self, transmission_type: n_byte) {
        math_random3(&mut self.immune_seed);
        if self.immune_seed[0] >= PATHOGEN_ENVIRONMENT_PROB {
            return;
        }
        let index = self.immune_seed[1] as usize % IMMUNE_ANTIGENS;
        if self.immune_antigens[index] != 0 {
            return;
        }
        math_random3(&mut self.immune_seed);
        self.immune_antigens[index] = (self.immune_seed[0] & 7) as n_byte;
        self.immune_shape_antigen[index] = random_pathogen(self.immune_seed[1], transmission_type);
    }

    fn honor_immune(&self) -> n_byte {
        (self.honor / 8).max(1)
    }
}

fn random_genetics(random: &mut [n_byte2; 2], male: bool) -> [n_genetics; CHROMOSOMES] {
    math_random3(random);
    let mut genetics = [0; CHROMOSOMES];
    for gene in &mut genetics {
        for bit in 0..(std::mem::size_of::<n_genetics>() * 8) {
            if math_random(random) & 1 != 0 {
                *gene |= 1 << bit;
            }
        }
    }
    genetics[CHROMOSOME_Y] &= !1;
    genetics[CHROMOSOME_Y] |= if male {
        SEX_MALE as n_genetics
    } else {
        SEX_FEMALE as n_genetics
    };
    genetics
}

fn inferred_gender_name(name: &str, genetics: [n_genetics; CHROMOSOMES]) -> n_byte2 {
    let sex = if (genetics[CHROMOSOME_Y] & 3) == SEX_FEMALE as n_genetics {
        SEX_FEMALE
    } else {
        SEX_MALE
    };
    let first = name
        .bytes()
        .fold(0u8, |hash, byte| hash.wrapping_mul(31).wrapping_add(byte));
    n_byte2::from(first) | ((sex as n_byte2) << 8)
}

fn age_days_at(current_date: n_byte4, date_of_birth: n_byte4) -> n_byte4 {
    current_date.saturating_sub(date_of_birth)
}

fn wrap_apespace(value: n_int) -> n_byte2 {
    let bounds = n_int::from(APESPACE_BOUNDS);
    value.rem_euclid(bounds + 1) as n_byte2
}

fn lookahead_location(location: [n_byte2; 2], facing: n_byte) -> [n_byte2; 2] {
    let vector = vect2_direction(n_int::from(facing), 128);
    [
        wrap_apespace(n_int::from(location[0]) + vector.x),
        wrap_apespace(n_int::from(location[1]) + vector.y),
    ]
}

fn math_spread_byte(value: n_byte) -> n_int {
    let result = n_int::from(value >> 1);
    if value & 1 == 1 {
        -result
    } else {
        result
    }
}

fn apespace_to_mapspace(value: n_int) -> n_int {
    n_int::from(wrap_apespace(value)) >> APE_TO_MAP_BIT_RATIO
}

fn positive_map_coord(value: n_int) -> n_int {
    value.rem_euclid(MAP_DIMENSION as n_int)
}

fn positive_hires_coord(value: n_int) -> n_int {
    value.rem_euclid(HI_RES_MAP_DIMENSION as n_int)
}

fn map_index(map_x: n_int, map_y: n_int) -> usize {
    usize::try_from(positive_map_coord(map_x)).unwrap_or(0)
        + (usize::try_from(positive_map_coord(map_y)).unwrap_or(0) << MAP_BITS)
}

fn water_test(height: n_int, tide: n_byte) -> bool {
    height < n_int::from(tide)
}

fn scale_food_energy(max_energy: n_byte2, quantity: n_byte) -> n_byte2 {
    if max_energy == BEING_DEAD {
        return BEING_DEAD;
    }
    ((n_uint::from(max_energy) * n_uint::from(quantity)) / n_uint::from(FOOD_QUANTITY_MAX))
        as n_byte2
}

fn inventory_food_source(item: n_byte2) -> Option<FoodSample> {
    let (food_type, max_energy) = if item & INVENTORY_FISH != 0 {
        (FOOD_SHELLFISH, ENERGY_FISH)
    } else if item & INVENTORY_NUT_CRACKED != 0 {
        (FOOD_VEGETABLE, ENERGY_NUT)
    } else if item & INVENTORY_BIRD_EGGS != 0 {
        (FOOD_BIRD_EGGS, ENERGY_BIRD_EGGS)
    } else if item & INVENTORY_LIZARD_EGGS != 0 {
        (FOOD_LIZARD_EGGS, ENERGY_LIZARD_EGGS)
    } else if item & INVENTORY_GRASS != 0 {
        (FOOD_VEGETABLE, ENERGY_GRASS)
    } else {
        return None;
    };
    Some(FoodSample {
        food_type,
        max_energy,
        energy: 0,
    })
}

fn operator_sun(fg: n_int, dfg: n_int, _fdg: n_int, ct: n_int, st: n_int) -> n_int {
    (((ct * fg) + (st * dfg)) >> 4) + WATER_MAP
}

fn land_dither(x: n_int, y: n_int, z: n_int) -> n_int {
    ((x + y + z) & 15) - (((x & y) | z) & 7) - ((x | (y & z)) & 7)
}

fn clamp_byte2(value: n_int) -> n_byte2 {
    value.clamp(0, n_int::from(n_byte2::MAX)) as n_byte2
}

fn nature_nurture(nature: n_byte, nurture: n_byte) -> n_byte {
    ((n_uint::from(nature) + (n_uint::from(nurture) >> 4)) >> 1) as n_byte
}

fn attraction_from_difference(value: n_int, preference: n_byte) -> n_int {
    let difference = value - n_int::from(preference);
    if (-2..=2).contains(&difference) {
        3 - difference.abs()
    } else {
        0
    }
}

fn social_observe_features(entry: &mut simulated_isocial, other: &BeingSummary) {
    featureset_update(
        &mut entry.classification,
        FEATURESET_PIGMENTATION,
        n_byte2::from(gene_pigmentation(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_HAIR,
        n_byte2::from(gene_hair(other.genetics)),
    );
    featureset_update(&mut entry.classification, FEATURESET_HEIGHT, other.height);
    featureset_update(&mut entry.classification, FEATURESET_FAT, other.body_fat());
    featureset_update(
        &mut entry.classification,
        FEATURESET_EYE_SHAPE,
        n_byte2::from(gene_eye_shape(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_EYE_COLOR,
        n_byte2::from(gene_eye_color(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_EYE_SEPARATION,
        n_byte2::from(gene_eye_separation(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_NOSE_SHAPE,
        n_byte2::from(gene_nose_shape(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_EAR_SHAPE,
        n_byte2::from(gene_ear_shape(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_EYEBROW_SHAPE,
        n_byte2::from(gene_eyebrow_shape(other.genetics)),
    );
    featureset_update(
        &mut entry.classification,
        FEATURESET_MOUTH_SHAPE,
        n_byte2::from(gene_mouth_shape(other.genetics)),
    );
    entry.classification.observations = entry.classification.observations.saturating_add(1);
    if entry.classification.observations > MAX_FEATURESET_OBSERVATIONS {
        entry.classification.observations >>= 1;
        for feature in entry
            .classification
            .features
            .iter_mut()
            .take(entry.classification.feature_number as usize)
        {
            feature.frequency >>= 1;
            feature.frequency = feature.frequency.max(1);
        }
    }
}

fn featureset_update(set: &mut simulated_featureset, feature_type: n_byte, value: n_byte2) {
    let len = set.feature_number as usize;
    if let Some(index) = set
        .features
        .iter()
        .take(len)
        .position(|feature| feature.feature_type == feature_type)
    {
        set.features[index].value = value;
        set.features[index].frequency = set.features[index].frequency.saturating_add(1);
        if set.features[index].frequency > MAX_FEATURE_FREQUENCY {
            featureset_normalise(set);
        }
        return;
    }

    let insert = set
        .features
        .iter()
        .take(len)
        .position(|feature| feature.feature_type > feature_type)
        .unwrap_or(len);
    if len < MAX_FEATURESET_SIZE {
        for index in (insert..len).rev() {
            set.features[index + 1] = set.features[index];
        }
        set.feature_number += 1;
        set.features[insert] = simulated_feature {
            value,
            frequency: 1,
            feature_type,
        };
    } else if let Some((replace, _)) = set
        .features
        .iter()
        .enumerate()
        .min_by_key(|(_, feature)| feature.frequency)
    {
        set.features[replace] = simulated_feature {
            value,
            frequency: 1,
            feature_type,
        };
        set.features[..len].sort_by_key(|feature| feature.feature_type);
    }
}

fn featureset_normalise(set: &mut simulated_featureset) {
    let len = set.feature_number as usize;
    let total = set
        .features
        .iter()
        .take(len)
        .map(|feature| n_uint::from(feature.frequency))
        .sum::<n_uint>()
        .max(1);
    let max = n_uint::from(MAX_FEATURE_FREQUENCY >> 1);
    for feature in set.features.iter_mut().take(len) {
        feature.frequency = ((n_uint::from(feature.frequency) * max) / total).max(1) as n_byte2;
    }
}

fn social_entry_empty(entry: &simulated_isocial) -> bool {
    entry.first_name[BEING_MET] == 0
        && entry.family_name[BEING_MET] == 0
        && entry.relationship <= RELATIONSHIP_SELF
}

fn affect_distance(affect: n_byte2) -> n_byte2 {
    affect.abs_diff(EPISODIC_AFFECT_ZERO)
}

pub fn is_night(time: n_byte4) -> bool {
    let hourish = time >> 5;
    !(11..=36).contains(&hourish)
}

pub fn is_dawndusk(time: n_byte4) -> bool {
    let hourish = time >> 5;
    hourish == 11 || hourish == 36
}

pub fn being_state_description(state: n_byte2) -> String {
    if state == BEING_STATE_ASLEEP {
        return "Sleeping".to_string();
    }

    const DESCRIPTIONS: &[(n_byte2, &str)] = &[
        (BEING_STATE_HUNGRY, "Hungry"),
        (BEING_STATE_SWIMMING, "Swimming"),
        (BEING_STATE_EATING, "Eating"),
        (BEING_STATE_MOVING, "Moving"),
        (BEING_STATE_SPEAKING, "Speaking"),
        (BEING_STATE_SHOUTING, "Shouting"),
        (BEING_STATE_GROOMING, "Grooming"),
        (BEING_STATE_SUCKLING, "Suckling"),
        (BEING_STATE_SHOWFORCE, "Showing Force"),
        (BEING_STATE_ATTACK, "Attacking"),
        (BEING_STATE_NO_FOOD, "No Food"),
    ];

    let mut output = String::new();
    for (flag, description) in DESCRIPTIONS {
        if state & *flag != 0 {
            if !output.is_empty() {
                output.push_str(", ");
            }
            output.push_str(description);
        }
    }
    if output.is_empty() {
        "Awake".to_string()
    } else {
        output
    }
}

pub fn drive_description(drives: [n_byte; DRIVES]) -> &'static str {
    let mut max_value = 0;
    let mut dominant = None;
    for (index, drive) in drives.iter().enumerate() {
        if *drive > max_value {
            max_value = *drive;
            dominant = Some(index);
        }
    }

    for (index, drive) in drives.iter().enumerate() {
        if *drive > (max_value / 2) && Some(index) != dominant {
            return "Mixed drives";
        }
    }

    match dominant {
        Some(DRIVE_SEX) => "Find mate",
        Some(DRIVE_HUNGER) => "Find food",
        Some(DRIVE_SOCIAL) => "Find friends",
        Some(DRIVE_FATIGUE) => "Find rest",
        _ => "No Drive",
    }
}

pub fn body_inventory_description(index: n_byte) -> &'static str {
    const DESCRIPTIONS: [&str; INVENTORY_SIZE] = [
        "Head",
        "Teeth",
        "Back",
        "Front",
        "Left hand",
        "Right hand",
        "Left foot",
        "Right foot",
    ];
    DESCRIPTIONS[index as usize % INVENTORY_SIZE]
}

pub fn relationship_description(index: n_byte) -> &'static str {
    const DESCRIPTIONS: [&str; 26] = [
        "Associate",
        "Self",
        "Mother",
        "Father",
        "Daughter",
        "Son",
        "Granddaughter",
        "Grandson",
        "Sister",
        "Brother",
        "Maternal Grandmother",
        "Maternal Grandfather",
        "Paternal Grandmother",
        "Paternal Grandson",
        "Mother",
        "Father",
        "Daughter",
        "Son",
        "Granddaughter",
        "Grandson",
        "Sister",
        "Brother",
        "Maternal Grandmother",
        "Maternal Grandfather",
        "Paternal Grandmother",
        "Paternal Grandson",
    ];
    DESCRIPTIONS
        .get(index as usize)
        .copied()
        .unwrap_or("Associate")
}

fn energy_to_growth(energy: n_byte2) -> n_byte2 {
    (energy >> 3) * 3
}

fn speed_history(speed: n_byte) -> [n_byte; 10] {
    let mut velocity = [0; 10];
    velocity[0] = speed;
    velocity
}

fn get_nucleotide(genetics: [n_genetics; CHROMOSOMES], number: usize) -> n_byte {
    ((genetics[(number >> 3) & 3] >> ((number & 7) * 2)) & 3) as n_byte
}

fn gene_val(genetics: [n_genetics; CHROMOSOMES], number0: usize, number1: usize) -> n_byte {
    (get_nucleotide(genetics, number0) << 2) | get_nucleotide(genetics, number1)
}

fn gene_regulator(genetics: [n_genetics; CHROMOSOMES], a: usize, b: usize) -> usize {
    usize::from(1 + gene_val(genetics, 15 + a, 15 + b))
}

fn gene_val_reg(
    genetics: [n_genetics; CHROMOSOMES],
    a: usize,
    b: usize,
    c: usize,
    d: usize,
) -> n_byte {
    gene_val(
        genetics,
        gene_regulator(genetics, a, b),
        gene_regulator(genetics, c, d),
    )
}

fn gene_frame(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 10, 11, 1, 6)
}

fn gene_hair(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 12, 5, 12, 11)
}

fn gene_nose_shape(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 4, 5, 6, 8)
}

fn gene_mouth_shape(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 5, 8, 15)
}

fn gene_pigmentation(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 8, 9, 8, 3)
}

fn gene_ear_shape(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 12, 4, 14, 1)
}

fn gene_eyebrow_shape(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 10, 8, 4)
}

fn gene_eye_shape(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 12, 1, 5)
}

fn gene_eye_color(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 7, 3, 7)
}

fn gene_eye_separation(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 3, 2, 0, 14)
}

fn gene_swim(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 11, 13, 7)
}

fn gene_speed(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 14, 5, 12, 10)
}

fn gene_stagger(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 12, 14, 3, 11)
}

fn gene_hill_climb(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 4, 6, 5, 2)
}

fn gene_status_preference(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 15, 12, 10, 1)
}

fn gene_pigmentation_preference(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 5, 3, 11, 4)
}

fn gene_height_preference(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 8, 14, 10)
}

fn gene_frame_preference(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 9, 0, 8, 2)
}

fn gene_hair_preference(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 10, 7, 14, 15)
}

fn gene_groom(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 14, 2, 5, 10)
}

fn gene_aggression(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 11, 3, 5, 0)
}

fn gene_incest_aversion(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 10, 8, 4, 9)
}

fn gene_energy_from_vegetables(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 3, 13, 15, 3)
}

fn gene_energy_from_fruits(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 14, 7, 6, 4)
}

fn gene_energy_from_shellfish(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 10, 12, 12, 2)
}

fn gene_energy_from_seaweed(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 0, 9, 11, 12)
}

fn gene_energy_from_bird_eggs(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 7, 1, 9, 5)
}

fn gene_energy_from_lizard_eggs(genetics: [n_genetics; CHROMOSOMES]) -> n_byte {
    gene_val_reg(genetics, 15, 3, 12, 8)
}

fn body_fat(genetics: [n_genetics; CHROMOSOMES], energy: n_byte2) -> n_byte2 {
    let fat = (n_uint::from(BEING_MAX_MASS_FAT_G)
        * n_uint::from(energy)
        * n_uint::from(gene_frame(genetics)))
        >> 16;
    fat.min(n_uint::from(BEING_MAX_MASS_FAT_G)) as n_byte2
}

fn pathogen_severity(pathogen: n_byte) -> n_byte {
    ((u16::from(pathogen) * u16::from(pathogen)) >> 11) as n_byte
}

fn random_pathogen(seed: n_byte2, pathogen_type: n_byte) -> n_byte {
    (((seed % (255 / PATHOGEN_TRANSMISSION_TOTAL)) * PATHOGEN_TRANSMISSION_TOTAL)
        + n_byte2::from(pathogen_type)) as n_byte
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopulationState {
    beings: Vec<BeingSummary>,
    selected: Option<usize>,
    max: usize,
}

impl PopulationState {
    pub fn empty(max: usize) -> Self {
        Self {
            beings: Vec::new(),
            selected: None,
            max,
        }
    }

    pub fn initial(random: &mut [n_byte2; 2], count: usize, max: usize) -> Self {
        let count = count.min(max);
        let mut beings = Vec::with_capacity(count);
        for index in 0..count {
            beings.push(BeingSummary::initial(index, random));
        }
        let selected = (!beings.is_empty()).then_some(0);
        Self {
            beings,
            selected,
            max,
        }
    }

    pub fn from_beings(beings: Vec<BeingSummary>, max: usize) -> Self {
        let selected = (!beings.is_empty()).then_some(0);
        Self {
            beings,
            selected,
            max,
        }
    }

    pub fn len(&self) -> usize {
        self.beings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.beings.is_empty()
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn beings(&self) -> &[BeingSummary] {
        &self.beings
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected(&self) -> Option<&BeingSummary> {
        self.selected.and_then(|index| self.beings.get(index))
    }

    pub fn select_by_name(&mut self, name: &str) -> bool {
        if let Some(index) = self.beings.iter().position(|being| being.name() == name) {
            self.selected = Some(index);
            true
        } else {
            false
        }
    }

    pub fn select_next(&mut self) {
        if self.beings.is_empty() {
            self.selected = None;
            return;
        }
        self.selected = Some(match self.selected {
            Some(index) if index + 1 < self.beings.len() => index + 1,
            _ => 0,
        });
    }

    pub fn select_previous(&mut self) {
        if self.beings.is_empty() {
            self.selected = None;
            return;
        }
        self.selected = Some(match self.selected {
            Some(0) | None => self.beings.len() - 1,
            Some(index) => index - 1,
        });
    }

    fn advance_minute(&mut self, land: &mut LandState) {
        let land_date = land.date();
        let land_time = land.time();
        for being in &mut self.beings {
            being.advance_minute(land);
        }
        self.social_initial_loop(land_date, land_time);
        self.social_secondary_loop_no_sim();
    }

    fn social_initial_loop(&mut self, land_date: n_byte4, land_time: n_byte4) {
        let snapshot = self.beings.clone();
        let targets = snapshot
            .iter()
            .map(|being| being.social_secondary_target(&snapshot))
            .collect::<Vec<_>>();

        for (being, target) in self.beings.iter_mut().zip(targets) {
            being.ensure_social_self();
            being.social_coord[2] = target[0];
            being.social_coord[3] = target[1];
        }

        let len = self.beings.len();
        for first in 0..len {
            for second in first + 1..len {
                if social_distance_under(&self.beings[first], &self.beings[second], 256) {
                    let (left, right) = self.beings.split_at_mut(second);
                    let first_being = &mut left[first];
                    let second_being = &mut right[0];
                    social_pair_cycle(first_being, second_being, land_date, land_time);
                }
            }
        }
    }

    fn social_secondary_loop_no_sim(&mut self) {
        for being in &mut self.beings {
            being.social_memory_maintenance();
            being.social_coord[0] = being.social_coord[2];
            being.social_coord[1] = being.social_coord[3];
        }
    }
}

fn social_distance_under(first: &BeingSummary, second: &BeingSummary, distance: n_int) -> bool {
    let dx = n_int::from(first.location[0]).abs_diff(n_int::from(second.location[0]));
    let dy = n_int::from(first.location[1]).abs_diff(n_int::from(second.location[1]));
    let distance = distance as u64;
    dx * dx + dy * dy <= distance * distance
}

fn social_pair_cycle(
    first: &mut BeingSummary,
    second: &mut BeingSummary,
    land_date: n_byte4,
    land_time: n_byte4,
) {
    let first_index = first.meet_being(second, land_date, land_time);
    let second_index = second.meet_being(first, land_date, land_time);
    first.immune_acquire_pathogen(PATHOGEN_TRANSMISSION_AIR);
    second.immune_acquire_pathogen(PATHOGEN_TRANSMISSION_AIR);
    let distance = social_distance(first, second);

    let first_familiarity = first.social_memory[first_index].familiarity;
    let second_familiarity = second.social_memory[second_index].familiarity;
    let groomed = social_groom_native(
        first,
        second,
        first_index,
        second_index,
        distance,
        first_familiarity,
        land_date,
        land_time,
    ) || social_groom_native(
        second,
        first,
        second_index,
        first_index,
        distance,
        second_familiarity,
        land_date,
        land_time,
    );

    if !groomed
        && !social_squabble_native(
            first,
            second,
            first_index,
            second_index,
            distance,
            land_date,
            land_time,
        )
    {
        social_mate_native(first, second, first_index, distance, land_date, land_time);
        social_mate_native(second, first, second_index, distance, land_date, land_time);
        social_chat_native(first, second, first_index, land_date, land_time);
        social_chat_native(second, first, second_index, land_date, land_time);
    }
}

fn social_distance(first: &BeingSummary, second: &BeingSummary) -> n_int {
    let dx = n_int::from(first.location[0]) - n_int::from(second.location[0]);
    let dy = n_int::from(first.location[1]) - n_int::from(second.location[1]);
    ((dx * dx + dy * dy) as f64).sqrt() as n_int
}

fn social_groom_native(
    groomer: &mut BeingSummary,
    groomee: &mut BeingSummary,
    groomer_index: usize,
    groomee_index: usize,
    distance: n_int,
    familiarity: n_byte2,
    land_date: n_byte4,
    land_time: n_byte4,
) -> bool {
    if groomer.awake_level == FULLY_ASLEEP
        || distance > 128
        || groomer.speed() >= MAX_SPEED_WHILST_GROOMING
    {
        return false;
    }

    let familiarity = familiarity.min(16);
    let preference_index = PREFERENCE_GROOM_MALE + usize::from(groomee.is_female());
    let preference = nature_nurture(
        gene_groom(groomer.genetics),
        groomer.learned_preference[preference_index],
    );
    let threshold = n_uint::from(GROOMING_PROB)
        + n_uint::from(preference)
            * n_uint::from(1 + familiarity)
            * n_uint::from(GROOMING_PROB_HONOR)
            * (n_uint::from(groomee.honor) + 1);
    let roll = n_uint::from(math_random(&mut groomer.random_seed) & 16_383);
    if roll >= threshold {
        return false;
    }

    groomer.immune_acquire_pathogen(PATHOGEN_TRANSMISSION_TOUCH);
    groomee.immune_acquire_pathogen(PATHOGEN_TRANSMISSION_TOUCH);

    let mut groomloc = usize::from(groomer.attention[ATTENTION_BODY]) % INVENTORY_SIZE;
    for _ in 0..4 {
        if groomee.inventory[groomloc] & INVENTORY_GROOMED == 0 {
            break;
        }
        groomloc = usize::from(math_random(&mut groomer.random_seed) as n_byte) % INVENTORY_SIZE;
    }
    if groomee.inventory[groomloc] & INVENTORY_WOUND != 0 {
        groomee.inventory[groomloc] = INVENTORY_GROOMED;
    } else {
        groomee.inventory[groomloc] |= INVENTORY_GROOMED;
    }

    groomer.social_memory[groomer_index].friend_foe = groomer.social_memory[groomer_index]
        .friend_foe
        .saturating_add(1);
    groomee.social_memory[groomee_index].friend_foe = groomee.social_memory[groomee_index]
        .friend_foe
        .saturating_add(1);
    groomer.macro_state |= BEING_STATE_GROOMING;
    groomee.macro_state |= BEING_STATE_GROOMING;
    groomer.honor = groomer.honor.saturating_add(1);
    groomee.honor = groomee.honor.saturating_sub(1);
    groomee.parasites = groomee.parasites.saturating_sub(PARASITES_REMOVED);
    groomer.attention[ATTENTION_BODY] = groomloc as n_byte;
    groomee.attention[ATTENTION_BODY] = groomloc as n_byte;
    groomer.record_episodic_interaction(
        groomee,
        EVENT_GROOM,
        AFFECT_GROOM,
        groomloc as n_byte2,
        land_date,
        land_time,
    );
    groomee.record_episodic_interaction(
        groomer,
        EVENT_GROOMED,
        AFFECT_GROOM,
        groomloc as n_byte2,
        land_date,
        land_time,
    );
    true
}

fn social_squabble_native(
    first: &mut BeingSummary,
    second: &mut BeingSummary,
    first_index: usize,
    second_index: usize,
    distance: n_int,
    land_date: n_byte4,
    land_time: n_byte4,
) -> bool {
    let forced = first.social_memory[first_index].friend_foe < SOCIAL_RESPECT_NORMAL - 32
        || second.social_memory[second_index].friend_foe < SOCIAL_RESPECT_NORMAL - 32;
    let mut aggression = n_uint::from(gene_aggression(first.genetics));
    if first.is_female() {
        aggression >>= 3;
    }
    let threshold = aggression * 4_096 + aggression * n_uint::from(first.honor) * 10;
    if !forced && n_uint::from(math_random(&mut first.random_seed)) >= threshold {
        return false;
    }

    let first_power =
        n_uint::from(math_random(&mut first.random_seed) & 7) * n_uint::from(first.energy);
    let second_power =
        n_uint::from(math_random(&mut first.random_seed) & 7) * n_uint::from(second.energy);
    let first_victor = first_power >= second_power;
    let punchloc = usize::from(math_random(&mut first.random_seed) as n_byte) % INVENTORY_SIZE;

    first.social_memory[first_index].friend_foe = first.social_memory[first_index]
        .friend_foe
        .saturating_sub(SQUABBLE_DISRESPECT);
    second.social_memory[second_index].friend_foe = second.social_memory[second_index]
        .friend_foe
        .saturating_sub(SQUABBLE_DISRESPECT);

    let state = if distance > SQUABBLE_SHOW_FORCE_DISTANCE {
        first.energy_delta(-SQUABBLE_ENERGY_SHOWFORCE);
        second.energy_delta(-SQUABBLE_ENERGY_SHOWFORCE);
        BEING_STATE_SHOWFORCE
    } else {
        first.energy_delta(-SQUABBLE_ENERGY_ATTACK);
        second.energy_delta(-SQUABBLE_ENERGY_ATTACK);
        if first_victor {
            second.inventory[punchloc] = INVENTORY_WOUND;
        } else {
            first.inventory[punchloc] = INVENTORY_WOUND;
        }
        BEING_STATE_ATTACK
    };

    if first_victor {
        first.honor = first.honor.saturating_add(SQUABBLE_HONOR_ADJUST);
        second.honor = second.honor.saturating_sub(SQUABBLE_HONOR_ADJUST);
        second.set_facing_towards(n_vect2::new(
            n_int::from(second.location[0]) - n_int::from(first.location[0]),
            n_int::from(second.location[1]) - n_int::from(first.location[1]),
        ));
        second.set_speed(SQUABBLE_FLEE_SPEED);
        first.macro_state |= state;
        second.macro_state |= state;
        first.record_episodic_interaction(
            second,
            EVENT_HIT,
            AFFECT_SQUABBLE_VICTOR,
            punchloc as n_byte2,
            land_date,
            land_time,
        );
        second.record_episodic_interaction(
            first,
            EVENT_HIT_BY,
            AFFECT_SQUABBLE_VANQUISHED,
            punchloc as n_byte2,
            land_date,
            land_time,
        );
    } else {
        second.honor = second.honor.saturating_add(SQUABBLE_HONOR_ADJUST);
        first.honor = first.honor.saturating_sub(SQUABBLE_HONOR_ADJUST);
        first.set_facing_towards(n_vect2::new(
            n_int::from(first.location[0]) - n_int::from(second.location[0]),
            n_int::from(first.location[1]) - n_int::from(second.location[1]),
        ));
        first.set_speed(SQUABBLE_FLEE_SPEED);
        first.macro_state |= state;
        second.macro_state |= state;
        second.record_episodic_interaction(
            first,
            EVENT_HIT,
            AFFECT_SQUABBLE_VICTOR,
            punchloc as n_byte2,
            land_date,
            land_time,
        );
        first.record_episodic_interaction(
            second,
            EVENT_HIT_BY,
            AFFECT_SQUABBLE_VANQUISHED,
            punchloc as n_byte2,
            land_date,
            land_time,
        );
    }
    true
}

fn social_mate_native(
    meeter: &mut BeingSummary,
    met: &mut BeingSummary,
    being_index: usize,
    distance: n_int,
    land_date: n_byte4,
    land_time: n_byte4,
) {
    if age_days_at(land_date, meeter.date_of_birth) < AGE_OF_MATURITY
        || age_days_at(land_date, met.date_of_birth) < AGE_OF_MATURITY
        || meeter.drive(DRIVE_SEX) <= THRESHOLD_SEEK_MATE
        || met.drive(DRIVE_SEX) <= THRESHOLD_SEEK_MATE
        || meeter.is_female() == met.is_female()
    {
        return;
    }

    let threshold = 32_000
        + n_uint::from(met.honor)
            * n_uint::from(gene_status_preference(meeter.genetics))
            * n_uint::from(MATING_PROB);
    if n_uint::from(math_random(&mut meeter.random_seed)) >= threshold {
        return;
    }

    let mut attraction = meeter.social_attraction_score(met);
    if meeter.social_memory[being_index].attraction > PAIR_BOND_THRESHOLD {
        attraction += 1;
        if distance < 16 {
            meeter.immune_acquire_pathogen(PATHOGEN_TRANSMISSION_SEX);
            met.immune_acquire_pathogen(PATHOGEN_TRANSMISSION_SEX);
        }
    } else {
        attraction -= 1;
    }

    let current = n_int::from(meeter.social_memory[being_index].attraction);
    let next = if attraction > 0 {
        if attraction < n_int::from(PAIR_BOND_THRESHOLD) * 4 {
            current.saturating_add(attraction).min(255)
        } else {
            current
        }
    } else {
        (current + attraction).max(0)
    };
    meeter.social_memory[being_index].attraction = next as n_byte;

    if attraction > 0 {
        meeter.goal = [GOAL_MATE, met.gender_name, met.family_name, 0];
        meeter.record_episodic_interaction(
            met,
            EVENT_SEEK_MATE,
            AFFECT_SEEK_MATE,
            n_byte2::from(meeter.social_memory[being_index].attraction),
            land_date,
            land_time,
        );
    }
}

fn social_chat_native(
    meeter: &mut BeingSummary,
    met: &BeingSummary,
    being_index: usize,
    land_date: n_byte4,
    land_time: n_byte4,
) {
    if meeter.social_memory[being_index].friend_foe < meeter.social_respect_mean() {
        return;
    }

    meeter.macro_state |= BEING_STATE_SPEAKING;
    meeter.reset_drive(DRIVE_SOCIAL);
    meeter.record_episodic_interaction(met, EVENT_CHAT, AFFECT_CHAT, 0, land_date, land_time);

    if let Some(anecdote) = met.episodic_memory.iter().find(|entry| entry.event != 0) {
        let mut copied = *anecdote;
        copied.first_name[BEING_MEETER] = meeter.gender_name;
        copied.family_name[BEING_MEETER] = meeter.family_name;
        copied.space_time.date = land_date;
        copied.space_time.time = land_time;
        copied.space_time.location = meeter.location;

        let event_roll = (math_random(&mut meeter.random_seed) & 255) as n_byte;
        if event_roll < meeter.learned_preference[PREFERENCE_ANECDOTE_EVENT_MUTATION] {
            copied.event ^= EVENT_INTENTION;
        }
        let affect_roll = (math_random(&mut meeter.random_seed) & 255) as n_byte;
        if affect_roll < meeter.learned_preference[PREFERENCE_ANECDOTE_AFFECT_MUTATION] {
            copied.affect = if copied.affect >= EPISODIC_AFFECT_ZERO {
                copied.affect.saturating_sub(1)
            } else {
                copied.affect.saturating_add(1)
            };
        }
        let replace = meeter
            .episodic_memory
            .iter()
            .position(|entry| entry.event == 0)
            .unwrap_or_else(|| {
                meeter
                    .episodic_memory
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, entry)| affect_distance(entry.affect))
                    .map(|(index, _)| index)
                    .unwrap_or(0)
            });
        meeter.episodic_memory[replace] = copied;
        meeter.attention[ATTENTION_EPISODE] = replace as n_byte;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimState {
    kind: KIND_OF_USE,
    land: LandState,
    random_seed: [n_byte2; 2],
    population: PopulationState,
}

impl SimState {
    pub fn init(kind: KIND_OF_USE, randomise: n_uint) -> Self {
        let mut random_seed = [
            ((randomise >> 16) & 0xffff) as n_byte2,
            (randomise & 0xffff) as n_byte2,
        ];
        math_random3(&mut random_seed);

        let mut land = LandState::new();
        if !matches!(
            kind,
            KIND_OF_USE::KIND_LOAD_FILE | KIND_OF_USE::KIND_MEMORY_SETUP
        ) {
            land.seed_genetics(&mut random_seed);
        }

        Self {
            kind,
            land,
            random_seed,
            population: PopulationState::empty(LARGE_SIM as usize),
        }
    }

    pub fn start_up(randomise: n_uint) -> Self {
        Self::init(KIND_OF_USE::KIND_START_UP, randomise)
    }

    pub fn from_startup_transfer(startup: &StartupTransfer) -> Self {
        let beings = startup
            .beings
            .iter()
            .filter_map(|entries| BeingSummary::from_transfer_object(entries).ok())
            .collect();
        Self {
            kind: KIND_OF_USE::KIND_LOAD_FILE,
            land: LandState::from_snapshot(startup.land),
            random_seed: [0; 2],
            population: PopulationState::from_beings(beings, LARGE_SIM as usize),
        }
    }

    pub fn load_startup_json(input: &[u8]) -> Result<Self, &'static str> {
        startup_transfer_from_json_bytes(input).map(|startup| Self::from_startup_transfer(&startup))
    }

    pub fn load_startup_bytes(input: &[u8]) -> Result<Self, &'static str> {
        match startup_transfer_from_json_bytes(input) {
            Ok(startup) => Ok(Self::from_startup_transfer(&startup)),
            Err(_) => startup_transfer_from_native_bytes(input)
                .map(|startup| Self::from_startup_transfer(&startup)),
        }
    }

    pub fn reset_new_simulation_from_land_seed(&mut self) {
        let mut seed = self.land.genetics();
        math_random3(&mut seed);
        let randomise = (n_uint::from(seed[0]) << 16) | n_uint::from(seed[1]);
        *self = Self::init(KIND_OF_USE::KIND_NEW_SIMULATION, randomise);
        self.populate_initial_from_land_seed();
    }

    pub fn step_empty(&mut self) {
        self.step_empty_by(1);
    }

    pub fn step_empty_by(&mut self, minutes: n_uint) {
        self.land.advance_minutes(minutes);
        self.kind = KIND_OF_USE::KIND_NOTHING_TO_RUN;
    }

    pub fn advance_minutes(&mut self, minutes: n_uint) {
        if self.population() == 0 {
            self.step_empty_by(minutes);
            return;
        }

        for _ in 0..minutes {
            self.land.advance_minutes(1);
            self.population.advance_minute(&mut self.land);
        }
        self.kind = KIND_OF_USE::KIND_NOTHING_TO_RUN;
    }

    pub const fn kind(&self) -> KIND_OF_USE {
        self.kind
    }

    pub const fn random_seed(&self) -> [n_byte2; 2] {
        self.random_seed
    }

    pub const fn land(&self) -> &LandState {
        &self.land
    }

    pub fn land_mut(&mut self) -> &mut LandState {
        &mut self.land
    }

    pub fn population(&self) -> usize {
        self.population.len()
    }

    pub fn max_population(&self) -> usize {
        self.population.max()
    }

    pub fn beings(&self) -> &[BeingSummary] {
        self.population.beings()
    }

    pub fn selected_being(&self) -> Option<&BeingSummary> {
        self.population.selected()
    }

    pub fn selected_name(&self) -> Option<&str> {
        self.selected_being().map(BeingSummary::name)
    }

    pub fn age_days(&self, being: &BeingSummary) -> n_byte4 {
        age_days_at(self.land.date(), being.date_of_birth())
    }

    pub fn adult_count(&self) -> usize {
        self.population
            .beings()
            .iter()
            .filter(|being| self.age_days(being) >= AGE_OF_MATURITY)
            .count()
    }

    pub fn juvenile_count(&self) -> usize {
        self.population().saturating_sub(self.adult_count())
    }

    pub fn select_by_name(&mut self, name: &str) -> bool {
        self.population.select_by_name(name)
    }

    pub fn select_next(&mut self) {
        self.population.select_next();
    }

    pub fn select_previous(&mut self) {
        self.population.select_previous();
    }

    pub fn land_snapshot(&self) -> LandSnapshot {
        self.land.snapshot()
    }

    pub fn startup_transfer(&self) -> StartupTransfer {
        StartupTransfer {
            land: self.land_snapshot(),
            beings: self
                .population
                .beings()
                .iter()
                .map(BeingSummary::transfer_object)
                .collect(),
        }
    }

    pub fn tranfer_startup_out_json(&self) -> NFile {
        tranfer_startup_out_json(&self.startup_transfer())
    }

    pub fn tranfer_startup_out_native(&self) -> NFile {
        tranfer_startup_out_native(&self.startup_transfer())
    }

    pub fn prepare_land_for_first_cycle(&mut self) {
        self.land.clear(self.kind, AGE_OF_MATURITY);
    }

    fn populate_initial_from_land_seed(&mut self) {
        let mut local_random = self.land.genetics();
        math_random3(&mut local_random);
        self.population =
            PopulationState::initial(&mut local_random, INITIAL_POPULATION, LARGE_SIM as usize);
    }
}

fn random_byte2(random: &mut [n_byte2; 2]) -> n_byte2 {
    ((math_random(random) & 255) << 8) | (math_random(random) & 255)
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartupTransfer {
    pub land: LandSnapshot,
    pub beings: Vec<Vec<ObjectEntry>>,
}

impl StartupTransfer {
    pub fn empty(land: LandSnapshot) -> Self {
        Self {
            land,
            beings: Vec::new(),
        }
    }
}

pub fn transfer_land_obj(land: LandSnapshot) -> Vec<ObjectEntry> {
    let mut simulated_iland = Vec::new();
    object_number(&mut simulated_iland, "date", land.date.into());

    let mut land_genetics = Vec::new();
    array_add(&mut land_genetics, array_number(land.genetics[0].into()));
    array_add(&mut land_genetics, array_number(land.genetics[1].into()));
    object_array(&mut simulated_iland, "genetics", land_genetics);

    object_number(&mut simulated_iland, "time", land.time.into());
    simulated_iland
}

pub fn transfer_sim_obj() -> Vec<ObjectEntry> {
    let mut simulated_isim_identifier = Vec::new();
    object_number(
        &mut simulated_isim_identifier,
        "signature",
        SIMULATED_APE_SIGNATURE.into(),
    );
    object_number(
        &mut simulated_isim_identifier,
        "version number",
        VERSION_NUMBER.into(),
    );
    object_string(
        &mut simulated_isim_identifier,
        "copyright",
        FULL_VERSION_COPYRIGHT,
    );
    object_string(&mut simulated_isim_identifier, "date", FULL_DATE);
    simulated_isim_identifier
}

pub fn transfer_startup_obj(startup: &StartupTransfer) -> Vec<ObjectEntry> {
    let mut simulation_object = Vec::new();
    object_object(&mut simulation_object, "information", transfer_sim_obj());
    object_object(
        &mut simulation_object,
        "land",
        transfer_land_obj(startup.land),
    );

    if !startup.beings.is_empty() {
        let beings = startup
            .beings
            .iter()
            .cloned()
            .map(ObjectValue::Object)
            .collect();
        object_array(&mut simulation_object, "beings", beings);
    }

    simulation_object
}

pub fn tranfer_startup_out_json(startup: &StartupTransfer) -> NFile {
    object_top_object(&transfer_startup_obj(startup))
}

pub fn tranfer_startup_out_native(startup: &StartupTransfer) -> NFile {
    let mut output = String::new();
    native_write_version(&mut output);
    native_write_land(&mut output, startup.land);
    for entries in &startup.beings {
        if let Ok(being) = BeingSummary::from_transfer_object(entries) {
            native_write_being(&mut output, &being);
        }
    }
    let mut file = NFile::new();
    let _ = file.write(output.as_bytes(), 0);
    file
}

fn native_write_version(output: &mut String) {
    output.push_str("simul{");
    native_write_field(output, "signa=", &[n_uint::from(SIMULATED_APE_SIGNATURE)]);
    native_write_field(output, "verio=", &[n_uint::from(VERSION_NUMBER)]);
    output.push_str("};\n");
}

fn native_write_land(output: &mut String, land: LandSnapshot) {
    output.push_str("landd{");
    native_write_field(output, "dated=", &[n_uint::from(land.date)]);
    native_write_field(output, "timed=", &[n_uint::from(land.time)]);
    native_write_field(
        output,
        "landg=",
        &[
            n_uint::from(land.genetics[0]),
            n_uint::from(land.genetics[1]),
        ],
    );
    output.push_str("};\n");
}

fn native_write_being(output: &mut String, being: &BeingSummary) {
    let native = being.to_simulated_being();
    let delta = native.delta;
    let constant = native.constant;
    let changes = native.changes;
    let brain = native.braindata;
    let immune = native.immune_system;

    output.push_str("being{");
    native_write_field(output, "locat=", &native_uints(&delta.location));
    native_write_field(output, "facin=", &[n_uint::from(delta.direction_facing)]);
    native_write_field(output, "speed=", &[n_uint::from(delta.velocity[0])]);
    native_write_field(output, "energ=", &[n_uint::from(delta.stored_energy)]);
    native_write_field(output, "datob=", &[n_uint::from(constant.date_of_birth)]);
    native_write_field(output, "rando=", &native_uints(&delta.random_seed));
    native_write_field(output, "state=", &[n_uint::from(delta.macro_state)]);
    native_write_field(output, "brast=", &native_uints(&brain.brain_state));
    native_write_field(output, "heigt=", &[n_uint::from(delta.height)]);
    native_write_field(output, "masss=", &[n_uint::from(delta.mass)]);
    native_write_field(output, "overr=", &[n_uint::from(brain.script_overrides)]);
    native_write_field(output, "shout=", &native_uints(&changes.shout));
    native_write_field(output, "crowd=", &[n_uint::from(delta.crowding)]);
    native_write_field(output, "postu=", &[n_uint::from(delta.posture)]);
    native_write_field(output, "inven=", &native_uints(&changes.inventory));
    native_write_field(output, "paras=", &[n_uint::from(delta.parasites)]);
    native_write_field(output, "honor=", &[n_uint::from(delta.honor)]);
    native_write_field(
        output,
        "conce=",
        &[n_uint::from(changes.date_of_conception)],
    );
    native_write_field(output, "atten=", &native_uints(&brain.attention));
    native_write_field(output, "genet=", &native_genetics_words(&constant.genetics));
    native_write_field(
        output,
        "fetag=",
        &native_genetics_words(&changes.fetal_genetics),
    );
    native_write_field(
        output,
        "fathn=",
        &[
            n_uint::from((changes.father_name[0] & 255) as n_byte),
            n_uint::from((changes.father_name[1] & 255) as n_byte),
        ],
    );
    native_write_field(
        output,
        "sosim=",
        &native_uints(&[
            delta.social_coord_x,
            delta.social_coord_y,
            delta.social_coord_nx,
            delta.social_coord_ny,
        ]),
    );
    native_write_field(output, "drive=", &native_uints(&changes.drives));
    native_write_field(output, "goals=", &native_uints(&delta.goal));
    native_write_field(output, "prefe=", &native_uints(&changes.learned_preference));
    native_write_field(output, "genex=", &[n_uint::from(constant.generation_max)]);
    native_write_field(output, "genen=", &[n_uint::from(constant.generation_min)]);
    native_write_field(
        output,
        "chigx=",
        &[n_uint::from(changes.child_generation_max)],
    );
    native_write_field(
        output,
        "chign=",
        &[n_uint::from(changes.child_generation_min)],
    );
    native_write_field(output, "immun=", &native_immune_bytes(&immune));
    native_write_field(output, "brreg=", &native_uints(&brain.braincode_register));
    native_write_field(
        output,
        "brpro=",
        &native_brainprobe_bytes(&brain.brainprobe),
    );
    output.push_str("};\n");

    for social in native
        .events
        .social
        .iter()
        .filter(|entry| !social_entry_empty(entry))
    {
        native_write_social(output, social);
    }
    for episodic in native
        .events
        .episodic
        .iter()
        .filter(|entry| entry.event != 0)
    {
        native_write_episodic(output, episodic);
    }
}

fn native_write_social(output: &mut String, entry: &simulated_isocial) {
    output.push_str("sgcia{");
    native_write_field(output, "sgloc=", &native_uints(&entry.space_time.location));
    native_write_field(output, "sgtim=", &[n_uint::from(entry.space_time.time)]);
    native_write_field(output, "sgdat=", &[n_uint::from(entry.space_time.date)]);
    native_write_field(
        output,
        "sgfin=",
        &[n_uint::from(entry.first_name[BEING_MET])],
    );
    native_write_field(
        output,
        "sgfan=",
        &[n_uint::from(entry.family_name[BEING_MET])],
    );
    native_write_field(output, "sgatt=", &[n_uint::from(entry.attraction)]);
    native_write_field(output, "sgfof=", &[n_uint::from(entry.friend_foe)]);
    native_write_field(output, "sgbel=", &[n_uint::from(entry.belief)]);
    native_write_field(output, "sgfam=", &[n_uint::from(entry.familiarity)]);
    native_write_field(output, "sgrel=", &[n_uint::from(entry.relationship)]);
    native_write_field(output, "sgtyp=", &[n_uint::from(entry.entity_type)]);
    if entry.braincode.iter().any(|byte| *byte != 0) {
        native_write_field(output, "sgbrc=", &native_uints(&entry.braincode));
    }
    output.push_str("};\n");
}

fn native_write_episodic(output: &mut String, entry: &simulated_iepisodic) {
    output.push_str("episo{");
    native_write_field(output, "eploc=", &native_uints(&entry.space_time.location));
    native_write_field(output, "eptim=", &[n_uint::from(entry.space_time.time)]);
    native_write_field(output, "epdat=", &[n_uint::from(entry.space_time.date)]);
    native_write_field(output, "epfin=", &native_uints(&entry.first_name));
    native_write_field(output, "epfan=", &native_uints(&entry.family_name));
    native_write_field(output, "epeve=", &[n_uint::from(entry.event)]);
    native_write_field(output, "epfoo=", &[n_uint::from(entry.food)]);
    native_write_field(output, "epafe=", &[n_uint::from(entry.affect)]);
    native_write_field(output, "eparg=", &[n_uint::from(entry.arg)]);
    output.push_str("};\n");
}

fn native_write_field(output: &mut String, token: &str, values: &[n_uint]) {
    output.push_str(token);
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        output.push_str(&value.to_string());
    }
    output.push(';');
}

fn native_uints<T>(values: &[T]) -> Vec<n_uint>
where
    T: Copy,
    n_uint: From<T>,
{
    values.iter().copied().map(n_uint::from).collect()
}

fn native_genetics_words(values: &[n_genetics; CHROMOSOMES]) -> Vec<n_uint> {
    let mut output = Vec::with_capacity(CHROMOSOMES * 2);
    for value in values {
        output.push(n_uint::from((value & 0xffff) as n_byte2));
        output.push(n_uint::from((value >> 16) as n_byte2));
    }
    output
}

fn native_brainprobe_bytes(values: &[simulated_ibrain_probe; BRAINCODE_PROBES]) -> Vec<n_uint> {
    let mut output = Vec::with_capacity(BRAINPROBE_NATIVE_BYTES);
    for probe in values {
        output.extend([
            n_uint::from(probe.probe_type),
            n_uint::from(probe.position),
            n_uint::from(probe.address),
            n_uint::from(probe.frequency),
            n_uint::from(probe.offset),
            n_uint::from(probe.state),
        ]);
    }
    output
}

fn native_immune_bytes(immune: &simulated_immune_system) -> Vec<n_uint> {
    let mut output = Vec::with_capacity(IMMUNE_NATIVE_BYTES);
    output.extend(native_uints(&immune.antigens));
    output.extend(native_uints(&immune.shape_antigen));
    output.extend(native_uints(&immune.antibodies));
    output.extend(native_uints(&immune.shape_antibody));
    for seed in immune.random_seed {
        output.push(n_uint::from((seed & 255) as n_byte));
        output.push(n_uint::from((seed >> 8) as n_byte));
    }
    output
}

fn object_array_byte2(object: &mut Vec<ObjectEntry>, name: &str, values: &[n_byte2]) {
    let mut array = Vec::new();
    for value in values {
        array_add(&mut array, array_number((*value).into()));
    }
    object_array(object, name, array);
}

fn object_array_byte(object: &mut Vec<ObjectEntry>, name: &str, values: &[n_byte]) {
    let mut array = Vec::new();
    for value in values {
        array_add(&mut array, array_number((*value).into()));
    }
    object_array(object, name, array);
}

fn object_array_genetics(object: &mut Vec<ObjectEntry>, name: &str, values: &[n_genetics]) {
    let mut array = Vec::new();
    for value in values {
        array_add(&mut array, array_number((*value).into()));
    }
    object_array(object, name, array);
}

pub fn startup_transfer_from_json_bytes(input: &[u8]) -> Result<StartupTransfer, &'static str> {
    let parsed = object_parse_json(input)?;
    let root = expect_object(&parsed, "root object expected")?;

    let information = field_object(root, "information")?;
    let signature = field_number(information, "signature")?;
    if signature != SIMULATED_APE_SIGNATURE.into() {
        return Err("not a simulated ape json");
    }
    let version = field_number(information, "version number")?;
    if version > VERSION_NUMBER.into() {
        return Err("json file newer than simulation");
    }

    let land = field_object(root, "land")?;
    let date = field_byte4(land, "date")?;
    let genetics = field_genetics(land)?;
    let time = field_byte4(land, "time")?;

    let beings = match optional_field(root, "beings") {
        Some(ObjectValue::Array(values)) => values
            .iter()
            .map(|value| match value {
                ObjectValue::Object(entries) => Ok(entries.clone()),
                _ => Err("being object expected"),
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err("beings array expected"),
        None => Vec::new(),
    };

    Ok(StartupTransfer {
        land: LandSnapshot::new(date, genetics, time),
        beings,
    })
}

pub fn startup_transfer_from_native_bytes(input: &[u8]) -> Result<StartupTransfer, &'static str> {
    let cleaned = native_strip_comments_and_whitespace(input);
    if cleaned.is_empty() {
        return Err("native file empty");
    }

    let mut parser = NativeFileParser::new(&cleaned);
    let Some(version_section) = parser.next_section()? else {
        return Err("signature not first in native file");
    };
    if version_section.token != *b"simul{" {
        return Err("signature not first in native file");
    }
    let signature = version_section
        .field_values(b"signa=")
        .and_then(|values| values.first())
        .copied()
        .ok_or("native signature missing")?;
    if signature != n_uint::from(SIMULATED_APE_SIGNATURE) {
        return Err("not a simulated ape native file");
    }
    let version = version_section
        .field_values(b"verio=")
        .and_then(|values| values.first())
        .copied()
        .ok_or("native version missing")?;
    if version > n_uint::from(VERSION_NUMBER) {
        return Err("native file newer than simulation");
    }

    let mut land = LandSnapshot::new(0, [0; 2], 0);
    let mut land_seen = false;
    let mut beings = Vec::new();
    let mut current_being = None;
    while let Some(section) = parser.next_section()? {
        match &section.token {
            b"landd{" => {
                land = native_land_from_section(&section)?;
                land_seen = true;
            }
            b"being{" => {
                beings.push(native_being_from_section(beings.len(), &section)?);
                current_being = Some(beings.len() - 1);
            }
            b"sgcia{" => {
                let Some(index) = current_being else {
                    return Err("native social section before being");
                };
                native_add_social_event(&mut beings[index], native_social_from_section(&section)?);
            }
            b"episo{" => {
                let Some(index) = current_being else {
                    return Err("native episodic section before being");
                };
                native_add_episodic_event(
                    &mut beings[index],
                    native_episodic_from_section(&section)?,
                );
            }
            b"weath{" => {
                return Err("native section not supported yet");
            }
            _ => return Err("unknown native file section"),
        }
    }

    if !land_seen {
        return Err("native land section missing");
    }
    Ok(StartupTransfer { land, beings })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeFileField {
    token: [u8; 6],
    values: Vec<n_uint>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeFileSection {
    token: [u8; 6],
    fields: Vec<NativeFileField>,
}

impl NativeFileSection {
    pub fn token_string(&self) -> String {
        String::from_utf8_lossy(&self.token).into_owned()
    }

    pub fn field_values(&self, token: &[u8; 6]) -> Option<&[n_uint]> {
        self.fields
            .iter()
            .find(|field| &field.token == token)
            .map(|field| field.values.as_slice())
    }
}

pub fn native_transfer_sections(input: &[u8]) -> Result<Vec<NativeFileSection>, &'static str> {
    let cleaned = native_strip_comments_and_whitespace(input);
    let mut parser = NativeFileParser::new(&cleaned);
    let mut sections = Vec::new();
    while let Some(section) = parser.next_section()? {
        sections.push(section);
    }
    Ok(sections)
}

struct NativeFileParser<'a> {
    input: &'a [u8],
    position: usize,
}

impl<'a> NativeFileParser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, position: 0 }
    }

    fn next_section(&mut self) -> Result<Option<NativeFileSection>, &'static str> {
        if self.position >= self.input.len() {
            return Ok(None);
        }
        let token = self.read_token()?;
        if token[5] != b'{' {
            return Err("native section start expected");
        }

        let mut fields = Vec::new();
        loop {
            if self.consume_section_end() {
                break;
            }
            let field_token = self.read_token()?;
            if field_token[5] != b'=' {
                return Err("native field assignment expected");
            }
            let values = self.read_values()?;
            fields.push(NativeFileField {
                token: field_token,
                values,
            });
        }

        Ok(Some(NativeFileSection { token, fields }))
    }

    fn read_token(&mut self) -> Result<[u8; 6], &'static str> {
        let end = self
            .position
            .checked_add(6)
            .ok_or("native token overflow")?;
        if end > self.input.len() {
            return Err("native token truncated");
        }
        let mut token = [0; 6];
        token.copy_from_slice(&self.input[self.position..end]);
        self.position = end;
        Ok(token)
    }

    fn consume_section_end(&mut self) -> bool {
        if self.input.get(self.position..self.position + 2) == Some(b"};") {
            self.position += 2;
            true
        } else {
            false
        }
    }

    fn read_values(&mut self) -> Result<Vec<n_uint>, &'static str> {
        let mut values = Vec::new();
        loop {
            let value_start = self.position;
            let mut value: n_uint = 0;
            while let Some(byte) = self.input.get(self.position).copied() {
                if !byte.is_ascii_digit() {
                    break;
                }
                value = value
                    .checked_mul(10)
                    .and_then(|acc| acc.checked_add(n_uint::from(byte - b'0')))
                    .ok_or("native number too large")?;
                self.position += 1;
            }
            if self.position == value_start {
                return Err("native number expected");
            }
            values.push(value);
            match self.input.get(self.position).copied() {
                Some(b',') => self.position += 1,
                Some(b';') => {
                    self.position += 1;
                    return Ok(values);
                }
                _ => return Err("native number terminator expected"),
            }
        }
    }
}

fn native_strip_comments_and_whitespace(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        let byte = input[index];
        if byte == b'/' && input.get(index + 1) == Some(&b'*') {
            index += 2;
            while index + 1 < input.len() && !(input[index] == b'*' && input[index + 1] == b'/') {
                index += 1;
            }
            index = (index + 2).min(input.len());
        } else if matches!(byte, b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c' | b' ') {
            index += 1;
        } else {
            output.push(byte);
            index += 1;
        }
    }
    output
}

fn native_land_from_section(section: &NativeFileSection) -> Result<LandSnapshot, &'static str> {
    let date = native_first_byte4(section, b"dated=")?.unwrap_or(0);
    let time = native_first_byte4(section, b"timed=")?.unwrap_or(0);
    let genetics = native_byte2_array::<2>(section, b"landg=")?.unwrap_or([0; 2]);
    Ok(LandSnapshot::new(date, genetics, time))
}

fn native_being_from_section(
    index: usize,
    section: &NativeFileSection,
) -> Result<Vec<ObjectEntry>, &'static str> {
    let location = native_byte2_array::<2>(section, b"locat=")?.unwrap_or([0; 2]);
    let facing = native_first_byte(section, b"facin=")?.unwrap_or(0);
    let speed = native_first_byte(section, b"speed=")?.unwrap_or(0);
    let energy = native_first_byte2(section, b"energ=")?.unwrap_or(BEING_DEAD);
    let date_of_birth = native_first_byte4(section, b"datob=")?.unwrap_or(0);
    let random_seed = native_byte2_array::<2>(section, b"rando=")?.unwrap_or([0; 2]);
    let macro_state = native_first_byte2(section, b"state=")?.unwrap_or(0);
    let brain_state = native_byte2_array::<6>(section, b"brast=")?.unwrap_or([0; 6]);
    let height = native_first_byte2(section, b"heigt=")?.unwrap_or(BIRTH_HEIGHT);
    let mass = native_first_byte2(section, b"masss=")?.unwrap_or(BIRTH_MASS);
    let script_overrides = native_first_byte2(section, b"overr=")?.unwrap_or(0);
    let shout = native_byte_array::<SHOUT_BYTES>(section, b"shout=")?.unwrap_or([0; SHOUT_BYTES]);
    let crowding = native_first_byte(section, b"crowd=")?.unwrap_or(MIN_CROWDING);
    let posture = native_first_byte(section, b"postu=")?.unwrap_or(0);
    let inventory =
        native_byte2_array::<INVENTORY_SIZE>(section, b"inven=")?.unwrap_or([0; INVENTORY_SIZE]);
    let parasites = native_first_byte(section, b"paras=")?.unwrap_or(0);
    let honor = native_first_byte(section, b"honor=")?.unwrap_or(0);
    let date_of_conception = native_first_byte4(section, b"conce=")?.unwrap_or(0);
    let attention =
        native_byte_array::<ATTENTION_SIZE>(section, b"atten=")?.unwrap_or([0; ATTENTION_SIZE]);
    let genetics_words = native_byte2_array::<{ CHROMOSOMES * 2 }>(section, b"genet=")?
        .unwrap_or([0; CHROMOSOMES * 2]);
    let fetal_words = native_byte2_array::<{ CHROMOSOMES * 2 }>(section, b"fetag=")?
        .unwrap_or([0; CHROMOSOMES * 2]);
    let father_bytes = native_byte_array::<2>(section, b"fathn=")?.unwrap_or([0; 2]);
    let social_coord = native_byte2_array::<4>(section, b"sosim=")?.unwrap_or([0; 4]);
    let drives = native_byte_array::<DRIVES>(section, b"drive=")?.unwrap_or([127; DRIVES]);
    let goal = native_byte2_array::<4>(section, b"goals=")?.unwrap_or([0; 4]);
    let preferences =
        native_byte_array::<PREFERENCES>(section, b"prefe=")?.unwrap_or([0; PREFERENCES]);
    let generation_max = native_first_byte2(section, b"genex=")?.unwrap_or(0);
    let generation_min = native_first_byte2(section, b"genen=")?.unwrap_or(0);
    let child_generation_max = native_first_byte2(section, b"chigx=")?.unwrap_or(0);
    let child_generation_min = native_first_byte2(section, b"chign=")?.unwrap_or(0);
    let braincode_register = native_byte_array::<BRAINCODE_PSPACE_REGISTERS>(section, b"brreg=")?
        .unwrap_or([0; BRAINCODE_PSPACE_REGISTERS]);
    let brainprobe = native_byte_array::<BRAINPROBE_NATIVE_BYTES>(section, b"brpro=")?
        .map(native_brainprobes_from_bytes)
        .unwrap_or([simulated_ibrain_probe::default(); BRAINCODE_PROBES]);
    let immune = native_byte_array::<IMMUNE_NATIVE_BYTES>(section, b"immun=")?
        .map(native_immune_from_bytes)
        .unwrap_or_default();

    let mut object = Vec::new();
    object_string(&mut object, "name", &format!("Ape {:03}", index + 1));

    let mut delta = Vec::new();
    object_number(&mut delta, "direction_facing", facing.into());
    object_array_byte2(&mut delta, "location", &location);
    object_number(&mut delta, "velocity", speed.into());
    object_number(&mut delta, "stored_energy", energy.into());
    object_array_byte2(&mut delta, "random_seed", &random_seed);
    object_number(&mut delta, "macro_state", macro_state.into());
    object_number(&mut delta, "parasites", parasites.into());
    object_number(&mut delta, "honor", honor.into());
    object_number(&mut delta, "crowding", crowding.into());
    object_number(&mut delta, "height", height.into());
    object_number(&mut delta, "mass", mass.into());
    object_number(&mut delta, "posture", posture.into());
    object_array_byte2(&mut delta, "goal", &goal);
    object_array_byte2(&mut delta, "social_coord", &social_coord);
    object_number(&mut delta, "awake", FULLY_AWAKE.into());
    object_object(&mut object, "delta", delta);

    let mut constant = Vec::new();
    object_number(&mut constant, "date_of_birth", date_of_birth.into());
    object_array_genetics(
        &mut constant,
        "genetics",
        &native_genetics_from_words(genetics_words),
    );
    object_array_byte2(
        &mut constant,
        "generation_range",
        &[generation_min, generation_max],
    );
    object_array_byte2(&mut constant, "name", &[0, 0]);
    object_object(&mut object, "constant", constant);

    let mut changes = Vec::new();
    object_array_byte(&mut changes, "drives", &drives);
    object_array_byte(&mut changes, "shout", &shout);
    object_array_byte2(&mut changes, "inventory", &inventory);
    object_array_byte(&mut changes, "learned_preference", &preferences);
    object_number(
        &mut changes,
        "date_of_conception",
        date_of_conception.into(),
    );
    object_array_genetics(
        &mut changes,
        "fetal_genetics",
        &native_genetics_from_words(fetal_words),
    );
    object_array_byte2(
        &mut changes,
        "father_name",
        &[
            n_byte2::from(father_bytes[0]),
            n_byte2::from(father_bytes[1]),
        ],
    );
    object_array_byte2(&mut changes, "mother_name", &[0, 0]);
    object_array_byte2(
        &mut changes,
        "child_generation_range",
        &[child_generation_min, child_generation_max],
    );
    object_object(&mut object, "changes", changes);

    let mut brain = Vec::new();
    object_array_byte(&mut brain, "braincode_register", &braincode_register);
    object_array_byte2(&mut brain, "brain_state", &brain_state);
    object_number(&mut brain, "script_overrides", script_overrides.into());
    object_array_byte(&mut brain, "attention", &attention);
    object_array(
        &mut brain,
        "brainprobe",
        brainprobe
            .iter()
            .map(|probe| ObjectValue::Object(brainprobe_entry_to_object(probe)))
            .collect(),
    );
    object_object(&mut object, "braindata", brain);

    object_object(&mut object, "immune_system", native_immune_object(&immune));

    Ok(object)
}

fn native_social_from_section(
    section: &NativeFileSection,
) -> Result<simulated_isocial, &'static str> {
    let mut entry = simulated_isocial::default();
    entry.space_time.location = native_byte2_array::<2>(section, b"sgloc=")?.unwrap_or([0; 2]);
    entry.space_time.time = n_byte4::from(native_first_byte2(section, b"sgtim=")?.unwrap_or(0));
    entry.space_time.date = native_first_byte4(section, b"sgdat=")?.unwrap_or(0);
    entry.first_name[BEING_MET] = native_first_byte2(section, b"sgfin=")?.unwrap_or(0);
    entry.family_name[BEING_MET] = native_first_byte2(section, b"sgfan=")?.unwrap_or(0);
    entry.attraction = native_first_byte(section, b"sgatt=")?.unwrap_or(0);
    entry.friend_foe = native_first_byte(section, b"sgfof=")?.unwrap_or(0);
    entry.belief = native_first_byte2(section, b"sgbel=")?.unwrap_or(0);
    entry.familiarity = native_first_byte2(section, b"sgfam=")?.unwrap_or(0);
    entry.relationship = native_first_byte(section, b"sgrel=")?.unwrap_or(0);
    entry.entity_type = native_first_byte(section, b"sgtyp=")?.unwrap_or(0);
    if let Some(braincode) = native_byte_array::<BRAINCODE_SIZE>(section, b"sgbrc=")? {
        entry.braincode = braincode;
    }
    Ok(entry)
}

fn native_episodic_from_section(
    section: &NativeFileSection,
) -> Result<simulated_iepisodic, &'static str> {
    let mut entry = simulated_iepisodic::default();
    entry.affect = EPISODIC_AFFECT_ZERO;
    entry.space_time.location = native_byte2_array::<2>(section, b"eploc=")?.unwrap_or([0; 2]);
    entry.space_time.time = n_byte4::from(native_first_byte2(section, b"eptim=")?.unwrap_or(0));
    entry.space_time.date = native_first_byte4(section, b"epdat=")?.unwrap_or(0);
    entry.first_name = native_byte2_array::<2>(section, b"epfin=")?.unwrap_or([0; 2]);
    entry.family_name = native_byte2_array::<2>(section, b"epfan=")?.unwrap_or([0; 2]);
    entry.event = native_first_byte(section, b"epeve=")?.unwrap_or(0);
    entry.food = native_first_byte(section, b"epfoo=")?.unwrap_or(0);
    entry.affect = native_first_byte2(section, b"epafe=")?.unwrap_or(EPISODIC_AFFECT_ZERO);
    entry.arg = native_first_byte2(section, b"eparg=")?.unwrap_or(0);
    Ok(entry)
}

fn native_add_social_event(being: &mut Vec<ObjectEntry>, entry: simulated_isocial) {
    native_push_event_object(being, "social", social_entry_to_object(&entry));
}

fn native_add_episodic_event(being: &mut Vec<ObjectEntry>, entry: simulated_iepisodic) {
    native_push_event_object(being, "episodic", episodic_entry_to_object(&entry));
}

fn native_push_event_object(
    being: &mut Vec<ObjectEntry>,
    array_name: &str,
    value: Vec<ObjectEntry>,
) {
    let events_index = if let Some(index) = being.iter().position(|entry| entry.name == "events") {
        index
    } else {
        being.push(ObjectEntry::new("events", ObjectValue::Object(Vec::new())));
        being.len() - 1
    };

    let ObjectValue::Object(events) = &mut being[events_index].value else {
        return;
    };
    let array_index = if let Some(index) = events.iter().position(|entry| entry.name == array_name)
    {
        index
    } else {
        events.push(ObjectEntry::new(array_name, ObjectValue::Array(Vec::new())));
        events.len() - 1
    };
    if let ObjectValue::Array(values) = &mut events[array_index].value {
        values.push(ObjectValue::Object(value));
    }
}

fn native_brainprobes_from_bytes(
    bytes: [n_byte; BRAINPROBE_NATIVE_BYTES],
) -> [simulated_ibrain_probe; BRAINCODE_PROBES] {
    std::array::from_fn(|index| {
        let offset = index * 6;
        simulated_ibrain_probe {
            probe_type: bytes[offset],
            position: bytes[offset + 1],
            address: bytes[offset + 2],
            frequency: bytes[offset + 3],
            offset: bytes[offset + 4],
            state: bytes[offset + 5],
        }
    })
}

fn native_immune_from_bytes(bytes: [n_byte; IMMUNE_NATIVE_BYTES]) -> simulated_immune_system {
    let mut immune = simulated_immune_system::default();
    let mut offset = 0;
    immune
        .antigens
        .copy_from_slice(&bytes[offset..offset + IMMUNE_ANTIGENS]);
    offset += IMMUNE_ANTIGENS;
    immune
        .shape_antigen
        .copy_from_slice(&bytes[offset..offset + IMMUNE_ANTIGENS]);
    offset += IMMUNE_ANTIGENS;
    immune
        .antibodies
        .copy_from_slice(&bytes[offset..offset + IMMUNE_POPULATION]);
    offset += IMMUNE_POPULATION;
    immune
        .shape_antibody
        .copy_from_slice(&bytes[offset..offset + IMMUNE_POPULATION]);
    offset += IMMUNE_POPULATION;
    immune.random_seed = [
        n_byte2::from(bytes[offset]) | (n_byte2::from(bytes[offset + 1]) << 8),
        n_byte2::from(bytes[offset + 2]) | (n_byte2::from(bytes[offset + 3]) << 8),
    ];
    immune
}

fn native_immune_object(immune: &simulated_immune_system) -> Vec<ObjectEntry> {
    let mut object = Vec::new();
    object_array_byte(&mut object, "antigens", &immune.antigens);
    object_array_byte(&mut object, "shape_antigen", &immune.shape_antigen);
    object_array_byte(&mut object, "antibodies", &immune.antibodies);
    object_array_byte(&mut object, "shape_antibody", &immune.shape_antibody);
    object_array_byte2(&mut object, "random_seed", &immune.random_seed);
    object
}

fn native_first_byte(
    section: &NativeFileSection,
    token: &[u8; 6],
) -> Result<Option<n_byte>, &'static str> {
    let Some(value) = section
        .field_values(token)
        .and_then(|values| values.first())
    else {
        return Ok(None);
    };
    n_byte::try_from(*value)
        .map(Some)
        .map_err(|_| "native byte value too large")
}

fn native_first_byte2(
    section: &NativeFileSection,
    token: &[u8; 6],
) -> Result<Option<n_byte2>, &'static str> {
    let Some(value) = section
        .field_values(token)
        .and_then(|values| values.first())
    else {
        return Ok(None);
    };
    n_byte2::try_from(*value)
        .map(Some)
        .map_err(|_| "native byte2 value too large")
}

fn native_first_byte4(
    section: &NativeFileSection,
    token: &[u8; 6],
) -> Result<Option<n_byte4>, &'static str> {
    let Some(value) = section
        .field_values(token)
        .and_then(|values| values.first())
    else {
        return Ok(None);
    };
    n_byte4::try_from(*value)
        .map(Some)
        .map_err(|_| "native byte4 value too large")
}

fn native_byte_array<const N: usize>(
    section: &NativeFileSection,
    token: &[u8; 6],
) -> Result<Option<[n_byte; N]>, &'static str> {
    let Some(values) = section.field_values(token) else {
        return Ok(None);
    };
    if values.len() != N {
        return Err("native byte array length mismatch");
    }
    let mut output = [0; N];
    for (slot, value) in output.iter_mut().zip(values.iter().copied()) {
        *slot = n_byte::try_from(value).map_err(|_| "native byte value too large")?;
    }
    Ok(Some(output))
}

fn native_byte2_array<const N: usize>(
    section: &NativeFileSection,
    token: &[u8; 6],
) -> Result<Option<[n_byte2; N]>, &'static str> {
    let Some(values) = section.field_values(token) else {
        return Ok(None);
    };
    if values.len() != N {
        return Err("native byte2 array length mismatch");
    }
    let mut output = [0; N];
    for (slot, value) in output.iter_mut().zip(values.iter().copied()) {
        *slot = n_byte2::try_from(value).map_err(|_| "native byte2 value too large")?;
    }
    Ok(Some(output))
}

fn native_genetics_from_words(words: [n_byte2; CHROMOSOMES * 2]) -> [n_genetics; CHROMOSOMES] {
    std::array::from_fn(|index| {
        n_genetics::from(words[index * 2]) | (n_genetics::from(words[(index * 2) + 1]) << 16)
    })
}

fn optional_field<'a>(entries: &'a [ObjectEntry], name: &str) -> Option<&'a ObjectValue> {
    entries
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| &entry.value)
}

fn optional_object<'a>(
    entries: &'a [ObjectEntry],
    name: &str,
) -> Result<Option<&'a [ObjectEntry]>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Object(entries)) => Ok(Some(entries)),
        Some(_) => Err("json object expected"),
        None => Ok(None),
    }
}

fn field<'a>(entries: &'a [ObjectEntry], name: &str) -> Result<&'a ObjectValue, &'static str> {
    optional_field(entries, name).ok_or("json field missing")
}

fn expect_object<'a>(
    value: &'a ObjectValue,
    error: &'static str,
) -> Result<&'a [ObjectEntry], &'static str> {
    match value {
        ObjectValue::Object(entries) => Ok(entries),
        _ => Err(error),
    }
}

fn field_object<'a>(
    entries: &'a [ObjectEntry],
    name: &str,
) -> Result<&'a [ObjectEntry], &'static str> {
    expect_object(field(entries, name)?, "json object expected")
}

fn field_number(entries: &[ObjectEntry], name: &str) -> Result<i64, &'static str> {
    match field(entries, name)? {
        ObjectValue::Number(number) => Ok(*number),
        _ => Err("json number expected"),
    }
}

fn optional_number_byte2(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<n_byte2>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Number(number)) if (0..=n_byte2::MAX.into()).contains(number) => {
            Ok(Some(*number as n_byte2))
        }
        Some(ObjectValue::Number(_)) => Err("json number outside n_byte2 range"),
        Some(_) => Err("json number expected"),
        None => Ok(None),
    }
}

fn optional_number_byte4(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<n_byte4>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Number(number)) if (0..=n_byte4::MAX.into()).contains(number) => {
            Ok(Some(*number as n_byte4))
        }
        Some(ObjectValue::Number(_)) => Err("json number outside n_byte4 range"),
        Some(_) => Err("json number expected"),
        None => Ok(None),
    }
}

fn optional_number_byte(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<n_byte>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Number(number)) if (0..=n_byte::MAX.into()).contains(number) => {
            Ok(Some(*number as n_byte))
        }
        Some(ObjectValue::Number(_)) => Err("json number outside n_byte range"),
        Some(_) => Err("json number expected"),
        None => Ok(None),
    }
}

fn optional_array_byte2_2(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<[n_byte2; 2]>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Array(values)) if values.len() == 2 => {
            Ok(Some([array_byte2(&values[0])?, array_byte2(&values[1])?]))
        }
        Some(ObjectValue::Array(_)) => Err("array should have two values"),
        Some(_) => Err("json array expected"),
        None => Ok(None),
    }
}

fn optional_array_byte2_4(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<[n_byte2; 4]>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Array(values)) if values.len() == 4 => Ok(Some([
            array_byte2(&values[0])?,
            array_byte2(&values[1])?,
            array_byte2(&values[2])?,
            array_byte2(&values[3])?,
        ])),
        Some(ObjectValue::Array(_)) => Err("array should have four values"),
        Some(_) => Err("json array expected"),
        None => Ok(None),
    }
}

fn optional_array_byte2(
    entries: &[ObjectEntry],
    name: &str,
    expected: usize,
) -> Result<Option<Vec<n_byte2>>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Array(values)) if values.len() == expected => values
            .iter()
            .map(array_byte2)
            .collect::<Result<Vec<_>, _>>()
            .map(Some),
        Some(ObjectValue::Array(_)) => Err("array length mismatch"),
        Some(_) => Err("json array expected"),
        None => Ok(None),
    }
}

fn optional_array_byte(
    entries: &[ObjectEntry],
    name: &str,
    expected: usize,
) -> Result<Option<Vec<n_byte>>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Array(values)) if values.len() == expected => values
            .iter()
            .map(array_byte)
            .collect::<Result<Vec<_>, _>>()
            .map(Some),
        Some(ObjectValue::Array(_)) => Err("array length mismatch"),
        Some(_) => Err("json array expected"),
        None => Ok(None),
    }
}

fn optional_genetics4(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<[n_genetics; CHROMOSOMES]>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Array(values)) if values.len() == CHROMOSOMES => {
            let mut genetics = [0; CHROMOSOMES];
            for (index, value) in values.iter().enumerate() {
                genetics[index] = array_byte4(value)?;
            }
            Ok(Some(genetics))
        }
        Some(ObjectValue::Array(_)) => Err("genetics array should have four values"),
        Some(_) => Err("genetics array expected"),
        None => Ok(None),
    }
}

fn optional_brainprobe_array(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<[simulated_ibrain_probe; BRAINCODE_PROBES]>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Array(values)) if values.len() <= BRAINCODE_PROBES => {
            let mut probes = [simulated_ibrain_probe::default(); BRAINCODE_PROBES];
            for (index, value) in values.iter().enumerate() {
                let ObjectValue::Object(entries) = value else {
                    return Err("brainprobe entry object expected");
                };
                probes[index] = brainprobe_entry_from_object(entries)?;
            }
            Ok(Some(probes))
        }
        Some(ObjectValue::Array(_)) => Err("brainprobe array too long"),
        Some(_) => Err("json array expected"),
        None => Ok(None),
    }
}

fn optional_velocity(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<Option<[n_byte; 10]>, &'static str> {
    match optional_field(entries, name) {
        Some(ObjectValue::Number(number)) if (0..=n_byte::MAX.into()).contains(number) => {
            Ok(Some(speed_history(*number as n_byte)))
        }
        Some(ObjectValue::Number(_)) => Err("json number outside n_byte range"),
        Some(ObjectValue::Array(values)) if values.len() == 10 => {
            let mut velocity = [0; 10];
            for (index, value) in values.iter().enumerate() {
                velocity[index] = array_byte(value)?;
            }
            Ok(Some(velocity))
        }
        Some(ObjectValue::Array(_)) => Err("velocity array should have ten values"),
        Some(_) => Err("velocity number or array expected"),
        None => Ok(None),
    }
}

fn field_string<'a>(entries: &'a [ObjectEntry], name: &str) -> Result<&'a str, &'static str> {
    match field(entries, name)? {
        ObjectValue::String(value) => Ok(value),
        _ => Err("json string expected"),
    }
}

fn field_byte4(entries: &[ObjectEntry], name: &str) -> Result<n_byte4, &'static str> {
    let number = field_number(entries, name)?;
    if (0..=n_byte4::MAX.into()).contains(&number) {
        Ok(number as n_byte4)
    } else {
        Err("json number outside n_byte4 range")
    }
}

fn field_named_byte2(entries: &[ObjectEntry], name: &str) -> Result<n_byte2, &'static str> {
    let number = field_number(entries, name)?;
    if (0..=n_byte2::MAX.into()).contains(&number) {
        Ok(number as n_byte2)
    } else {
        Err("json number outside n_byte2 range")
    }
}

fn field_genetics(entries: &[ObjectEntry]) -> Result<[n_byte2; 2], &'static str> {
    match field(entries, "genetics")? {
        ObjectValue::Array(values) if values.len() == 2 => {
            Ok([array_byte2(&values[0])?, array_byte2(&values[1])?])
        }
        ObjectValue::Array(_) => Err("genetics array should have two values"),
        _ => Err("genetics array expected"),
    }
}

fn field_genetics4(
    entries: &[ObjectEntry],
    name: &str,
) -> Result<[n_genetics; CHROMOSOMES], &'static str> {
    match field(entries, name)? {
        ObjectValue::Array(values) if values.len() == CHROMOSOMES => {
            let mut genetics = [0; CHROMOSOMES];
            for (index, value) in values.iter().enumerate() {
                genetics[index] = array_byte4(value)?;
            }
            Ok(genetics)
        }
        ObjectValue::Array(_) => Err("genetics array should have four values"),
        _ => Err("genetics array expected"),
    }
}

fn array_byte2(value: &ObjectValue) -> Result<n_byte2, &'static str> {
    match value {
        ObjectValue::Number(number) if (0..=n_byte2::MAX.into()).contains(number) => {
            Ok(*number as n_byte2)
        }
        ObjectValue::Number(_) => Err("json number outside n_byte2 range"),
        _ => Err("json number expected"),
    }
}

fn array_byte(value: &ObjectValue) -> Result<n_byte, &'static str> {
    match value {
        ObjectValue::Number(number) if (0..=n_byte::MAX.into()).contains(number) => {
            Ok(*number as n_byte)
        }
        ObjectValue::Number(_) => Err("json number outside n_byte range"),
        _ => Err("json number expected"),
    }
}

fn array_byte4(value: &ObjectValue) -> Result<n_byte4, &'static str> {
    match value {
        ObjectValue::Number(number) if (0..=n_byte4::MAX.into()).contains(number) => {
            Ok(*number as n_byte4)
        }
        ObjectValue::Number(_) => Err("json number outside n_byte4 range"),
        _ => Err("json number expected"),
    }
}

fn social_entry_from_object(entries: &[ObjectEntry]) -> Result<simulated_isocial, &'static str> {
    let mut entry = simulated_isocial::default();
    if let Some(space_time) = optional_object(entries, "space_time")? {
        entry.space_time.date = optional_number_byte4(space_time, "date")?.unwrap_or(0);
        entry.space_time.location =
            optional_array_byte2_2(space_time, "location")?.unwrap_or([0; 2]);
        entry.space_time.time = optional_number_byte4(space_time, "time")?.unwrap_or(0);
    }
    entry.first_name = optional_array_byte2_2(entries, "first_name")?.unwrap_or([0; 2]);
    entry.family_name = optional_array_byte2_2(entries, "family_name")?.unwrap_or([0; 2]);
    entry.attraction = optional_number_byte(entries, "attraction")?.unwrap_or(0);
    entry.friend_foe =
        optional_number_byte(entries, "friend_foe")?.unwrap_or(SOCIAL_RESPECT_NORMAL);
    entry.belief = optional_number_byte2(entries, "belief")?.unwrap_or(0);
    entry.familiarity = optional_number_byte2(entries, "familiarity")?.unwrap_or(0);
    entry.relationship = optional_number_byte(entries, "relationship")?.unwrap_or(0);
    entry.entity_type = optional_number_byte(entries, "entity_type")?.unwrap_or(ENTITY_BEING);
    if let Some(classification) = optional_object(entries, "classification")? {
        entry.classification.feature_number =
            optional_number_byte2(classification, "feature_number")?.unwrap_or(0);
        entry.classification.observations =
            optional_number_byte2(classification, "observations")?.unwrap_or(0);
        if let Some(ObjectValue::Array(features)) = optional_field(classification, "features") {
            let mut loaded = [simulated_feature::default(); MAX_FEATURESET_SIZE];
            for (index, value) in features.iter().take(MAX_FEATURESET_SIZE).enumerate() {
                let ObjectValue::Object(feature_entries) = value else {
                    return Err("feature entry object expected");
                };
                loaded[index] = feature_entry_from_object(feature_entries)?;
            }
            entry.classification.features = loaded;
            entry.classification.feature_number =
                features.len().min(MAX_FEATURESET_SIZE) as n_byte2;
        } else if optional_field(classification, "features").is_some() {
            return Err("features array expected");
        }
    }
    if let Some(braincode) = optional_array_byte(entries, "braincode", BRAINCODE_SIZE)? {
        entry.braincode.copy_from_slice(&braincode);
    }
    Ok(entry)
}

fn social_entry_to_object(entry: &simulated_isocial) -> Vec<ObjectEntry> {
    let mut object = Vec::new();
    let mut space_time = Vec::new();
    object_number(&mut space_time, "date", entry.space_time.date.into());
    object_array_byte2(&mut space_time, "location", &entry.space_time.location);
    object_number(&mut space_time, "time", entry.space_time.time.into());
    object_object(&mut object, "space_time", space_time);
    object_array_byte2(&mut object, "first_name", &entry.first_name);
    object_array_byte2(&mut object, "family_name", &entry.family_name);
    object_number(&mut object, "attraction", entry.attraction.into());
    object_number(&mut object, "friend_foe", entry.friend_foe.into());
    object_number(&mut object, "belief", entry.belief.into());
    object_number(&mut object, "familiarity", entry.familiarity.into());
    object_number(&mut object, "relationship", entry.relationship.into());
    object_number(&mut object, "entity_type", entry.entity_type.into());
    let mut classification = Vec::new();
    object_number(
        &mut classification,
        "feature_number",
        entry.classification.feature_number.into(),
    );
    object_number(
        &mut classification,
        "observations",
        entry.classification.observations.into(),
    );
    if entry.classification.feature_number > 0 {
        object_array(
            &mut classification,
            "features",
            entry
                .classification
                .features
                .iter()
                .take(entry.classification.feature_number as usize)
                .map(|feature| array_object(feature_entry_to_object(feature)))
                .collect(),
        );
    }
    object_object(&mut object, "classification", classification);
    if entry.braincode.iter().any(|byte| *byte != 0) {
        object_array_byte(&mut object, "braincode", &entry.braincode);
    }
    object
}

fn feature_entry_from_object(entries: &[ObjectEntry]) -> Result<simulated_feature, &'static str> {
    Ok(simulated_feature {
        feature_type: optional_number_byte(entries, "type")?
            .or(optional_number_byte(entries, "feature_type")?)
            .unwrap_or(0),
        value: optional_number_byte2(entries, "value")?.unwrap_or(0),
        frequency: optional_number_byte2(entries, "frequency")?.unwrap_or(0),
    })
}

fn feature_entry_to_object(feature: &simulated_feature) -> Vec<ObjectEntry> {
    let mut object = Vec::new();
    object_number(&mut object, "type", feature.feature_type.into());
    object_number(&mut object, "value", feature.value.into());
    object_number(&mut object, "frequency", feature.frequency.into());
    object
}

fn episodic_entry_from_object(
    entries: &[ObjectEntry],
) -> Result<simulated_iepisodic, &'static str> {
    let mut entry = simulated_iepisodic::default();
    entry.affect = EPISODIC_AFFECT_ZERO;
    if let Some(space_time) = optional_object(entries, "space_time")? {
        entry.space_time.date = optional_number_byte4(space_time, "date")?.unwrap_or(0);
        entry.space_time.location =
            optional_array_byte2_2(space_time, "location")?.unwrap_or([0; 2]);
        entry.space_time.time = optional_number_byte4(space_time, "time")?.unwrap_or(0);
    }
    entry.first_name = optional_array_byte2_2(entries, "first_name")?.unwrap_or([0; 2]);
    entry.family_name = optional_array_byte2_2(entries, "family_name")?.unwrap_or([0; 2]);
    entry.event = optional_number_byte(entries, "event")?.unwrap_or(0);
    entry.food = optional_number_byte(entries, "food")?.unwrap_or(0);
    entry.affect = optional_number_byte2(entries, "affect")?.unwrap_or(EPISODIC_AFFECT_ZERO);
    entry.arg = optional_number_byte2(entries, "arg")?.unwrap_or(0);
    Ok(entry)
}

fn episodic_entry_to_object(entry: &simulated_iepisodic) -> Vec<ObjectEntry> {
    let mut object = Vec::new();
    let mut space_time = Vec::new();
    object_number(&mut space_time, "date", entry.space_time.date.into());
    object_array_byte2(&mut space_time, "location", &entry.space_time.location);
    object_number(&mut space_time, "time", entry.space_time.time.into());
    object_object(&mut object, "space_time", space_time);
    object_array_byte2(&mut object, "first_name", &entry.first_name);
    object_array_byte2(&mut object, "family_name", &entry.family_name);
    object_number(&mut object, "event", entry.event.into());
    object_number(&mut object, "food", entry.food.into());
    object_number(&mut object, "affect", entry.affect.into());
    object_number(&mut object, "arg", entry.arg.into());
    object
}

fn brainprobe_entry_from_object(
    entries: &[ObjectEntry],
) -> Result<simulated_ibrain_probe, &'static str> {
    Ok(simulated_ibrain_probe {
        probe_type: optional_number_byte(entries, "probe_type")?.unwrap_or(0),
        position: optional_number_byte(entries, "position")?.unwrap_or(0),
        address: optional_number_byte(entries, "address")?.unwrap_or(0),
        frequency: optional_number_byte(entries, "frequency")?.unwrap_or(0),
        offset: optional_number_byte(entries, "offset")?.unwrap_or(0),
        state: optional_number_byte(entries, "state")?.unwrap_or(0),
    })
}

fn brainprobe_entry_to_object(entry: &simulated_ibrain_probe) -> Vec<ObjectEntry> {
    let mut object = Vec::new();
    object_number(&mut object, "probe_type", entry.probe_type.into());
    object_number(&mut object, "position", entry.position.into());
    object_number(&mut object, "address", entry.address.into());
    object_number(&mut object, "frequency", entry.frequency.into());
    object_number(&mut object, "offset", entry.offset.into());
    object_number(&mut object, "state", entry.state.into());
    object
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{offset_of, size_of};

    #[test]
    fn version_constants_match_c_headers() {
        assert_eq!(SHORT_VERSION_NAME, "Simulated Ape 0.708 ");
        assert_eq!(VERSION_NUMBER, 708);
        assert_eq!(FULL_VERSION_COPYRIGHT, "Copyright Tom Barbalet, 1996-2026.");
        assert_eq!(SIMULATED_APE_SIGNATURE, 0x4e41);
        assert_eq!(SIMULATED_WAR_SIGNATURE, 0x4e57);
    }

    #[test]
    fn map_and_window_constants_match_default_command_line_build() {
        assert_eq!(MAP_BITS, 9);
        assert_eq!(MAP_TILES, 1);
        assert_eq!(MAP_DIMENSION, 512);
        assert_eq!(MAP_AREA, 262_144);
        assert_eq!(MAP_APE_RESOLUTION_SIZE, 32_768);
        assert_eq!(APESPACE_BOUNDS, 32_767);
        assert_eq!(TERRITORY_DIMENSION, 16);
        assert_eq!(TERRITORY_AREA, 256);
        assert_eq!(OFFSCREENSIZE, 17_039_360);
        assert_eq!(LARGE_SIM, 256);
    }

    #[test]
    fn time_constants_match_c_headers() {
        assert_eq!(TIME_DAY_MINUTES, 1_440);
        assert_eq!(TIME_MONTH_MINUTES, 40_320);
        assert_eq!(TIME_YEAR_MINUTES, 524_160);
        assert_eq!(TIME_YEAR_DAYS, 364);
        assert_eq!(TIME_CENTURY_DAYS, 36_400);
        assert_eq!(AGE_OF_MATURITY, 4_368);
    }

    #[test]
    fn braincode_constants_match_c_headers() {
        assert_eq!(BRAINCODE_SIZE, 128);
        assert_eq!(BRAINCODE_PROBES, 16);
        assert_eq!(BRAINCODE_PSPACE_REGISTERS, 3);
        assert_eq!(BRAINCODE_MAX_ADDRESS, 256);
        assert_eq!(BRAINCODE_INSTRUCTIONS, 30);
        assert_eq!(BRAINCODE_ADD, 2);
        assert_eq!(BRAINCODE_ANE, 29);
    }

    #[test]
    fn braincode_decode_wraps_instruction_arguments() {
        let mut code = [0; BRAINCODE_SIZE];
        code[126] = BRAINCODE_ADD | BRAINCODE_CONSTANT0_BIT | BRAINCODE_CONSTANT1_BIT;
        code[127] = 7;
        code[0] = 3;

        assert_eq!(
            braincode_decode(&code, 126),
            BraincodeInstruction {
                opcode: BRAINCODE_ADD,
                constant0: true,
                constant1: true,
                value0: 7,
                value1: 3,
            }
        );
    }

    #[test]
    fn braincode_vm_executes_data_arithmetic_and_register_ops() {
        let mut code = [0; BRAINCODE_SIZE];
        code[0] = BRAINCODE_MOV | BRAINCODE_CONSTANT0_BIT;
        code[1] = 12;
        code[2] = 60;
        code[3] = BRAINCODE_ADD | BRAINCODE_CONSTANT0_BIT;
        code[4] = 5;
        code[5] = 57;
        code[6] = BRAINCODE_SUB | BRAINCODE_CONSTANT0_BIT;
        code[7] = 4;
        code[8] = 54;
        code[9] = BRAINCODE_MUL | BRAINCODE_CONSTANT0_BIT;
        code[10] = 3;
        code[11] = 51;
        code[12] = BRAINCODE_DIV | BRAINCODE_CONSTANT0_BIT;
        code[13] = 1;
        code[14] = 48;
        code[15] = BRAINCODE_MOD | BRAINCODE_CONSTANT0_BIT;
        code[16] = 5;
        code[17] = 45;
        code[18] = BRAINCODE_STP | BRAINCODE_CONSTANT0_BIT | BRAINCODE_CONSTANT1_BIT;
        code[19] = 1;
        code[20] = 77;
        code[21] = BRAINCODE_LTP | BRAINCODE_CONSTANT0_BIT;
        code[22] = 1;
        code[23] = 39;

        let mut vm = BraincodeVm::new(code);
        for _ in 0..6 {
            vm.execute_step();
        }

        assert_eq!(vm.local()[60], 4);
        vm.execute_step();
        vm.execute_step();
        assert_eq!(vm.registers()[1], 77);
        assert_eq!(vm.local()[60], 77);
        assert_eq!(vm.pc(), 24);
    }

    #[test]
    fn braincode_vm_executes_conditionals_and_byte_mutators() {
        let mut code = [0; BRAINCODE_SIZE];
        code[0] = BRAINCODE_SEQ | BRAINCODE_CONSTANT0_BIT;
        code[1] = 42;
        code[2] = 9;
        code[9] = 42;

        let mut vm = BraincodeVm::new(code);
        vm.execute_step();
        assert_eq!(vm.pc(), 6);

        let mut code = [0; BRAINCODE_SIZE];
        code[0] = BRAINCODE_CTR;
        code[1] = 20;
        code[2] = 21;
        code[20] = 200;
        code[21] = 1;
        code[3] = BRAINCODE_INV;
        code[4] = 0;
        code[5] = 18;
        code[21] = 1;
        let mut vm = BraincodeVm::new(code);
        vm.execute_step();
        vm.execute_step();
        assert_eq!(vm.local()[21], 253);
    }

    #[test]
    fn state_and_drive_descriptions_match_c_shapes() {
        assert_eq!(being_state_description(BEING_STATE_ASLEEP), "Sleeping");
        assert_eq!(being_state_description(BEING_STATE_AWAKE), "Awake");
        assert_eq!(
            being_state_description(BEING_STATE_HUNGRY | BEING_STATE_MOVING),
            "Hungry, Moving"
        );
        assert_eq!(drive_description([200, 0, 0, 0]), "Find food");
        assert_eq!(drive_description([0, 200, 0, 0]), "Find friends");
        assert_eq!(drive_description([0, 0, 200, 0]), "Find rest");
        assert_eq!(drive_description([0, 0, 0, 200]), "Find mate");
        assert_eq!(drive_description([100, 80, 0, 0]), "Mixed drives");
        assert_eq!(drive_description([0, 0, 0, 0]), "No Drive");
    }

    #[test]
    fn energy_accessors_clamp_at_dead_and_report_thresholds() {
        let mut being = BeingSummary::new("Tester".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        assert!(being.energy_less_than(BEING_HUNGRY));
        being.energy_delta(5_000);
        assert_eq!(being.energy(), 5_000);
        assert!(!being.energy_less_than(BEING_HUNGRY));
        being.energy_delta(-10_000);
        assert_eq!(being.energy(), BEING_DEAD);
        assert!(being.energy_less_than(BEING_DEAD + 1));
    }

    #[test]
    fn c_version_struct_layout_matches_two_u16_fields() {
        assert_eq!(size_of::<n_version>(), 4);
        let version = n_version {
            signature: SIMULATED_APE_SIGNATURE,
            version: VERSION_NUMBER,
        };
        assert_eq!(version.signature, 0x4e41);
        assert_eq!(version.version, 708);
    }

    #[test]
    fn c_model_struct_layout_matches_command_audit_offsets() {
        assert_eq!(size_of::<simulated_feature>(), 6);
        assert_eq!(size_of::<simulated_featureset>(), 100);
        assert_eq!(size_of::<simulated_isocial>(), 256);
        assert_eq!(offset_of!(simulated_isocial, space_time), 0);
        assert_eq!(offset_of!(simulated_isocial, first_name), 12);
        assert_eq!(offset_of!(simulated_isocial, family_name), 16);
        assert_eq!(offset_of!(simulated_isocial, attraction), 20);
        assert_eq!(offset_of!(simulated_isocial, classification), 28);
        assert_eq!(offset_of!(simulated_isocial, braincode), 128);

        assert_eq!(size_of::<simulated_iepisodic>(), 28);
        assert_eq!(offset_of!(simulated_iepisodic, first_name), 12);
        assert_eq!(offset_of!(simulated_iepisodic, event), 20);
        assert_eq!(offset_of!(simulated_iepisodic, arg), 24);

        assert_eq!(size_of::<simulated_being_delta>(), 52);
        assert_eq!(offset_of!(simulated_being_delta, direction_facing), 4);
        assert_eq!(offset_of!(simulated_being_delta, stored_energy), 16);
        assert_eq!(offset_of!(simulated_being_delta, goal), 34);

        assert_eq!(size_of::<simulated_being_constant>(), 28);
        assert_eq!(size_of::<simulated_being_brain>(), 120);
        assert_eq!(offset_of!(simulated_being_brain, braincode_register), 0);
        assert_eq!(offset_of!(simulated_being_brain, brainprobe), 3);
        assert_eq!(offset_of!(simulated_being_brain, brain_state), 100);
        assert_eq!(offset_of!(simulated_being_brain, attention), 114);

        assert_eq!(size_of::<simulated_being_volatile>(), 72);
        assert_eq!(offset_of!(simulated_being_volatile, drives), 0);
        assert_eq!(offset_of!(simulated_being_volatile, shout), 4);
        assert_eq!(offset_of!(simulated_being_volatile, inventory), 10);
        assert_eq!(offset_of!(simulated_being_volatile, date_of_conception), 40);
        assert_eq!(
            offset_of!(simulated_being_volatile, child_generation_max),
            70
        );

        assert_eq!(size_of::<simulated_immune_system>(), 52);
        assert_eq!(size_of::<simulated_being>(), 4_756);
        assert_eq!(offset_of!(simulated_being, constant), 52);
        assert_eq!(offset_of!(simulated_being, events), 80);
        assert_eq!(offset_of!(simulated_being, braindata), 4_512);
        assert_eq!(offset_of!(simulated_being, changes), 4_632);
        assert_eq!(offset_of!(simulated_being, immune_system), 4_704);
    }

    #[test]
    fn banner_matches_golden_prefix_with_default_date() {
        assert_eq!(
            banner(),
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n"
        );
    }

    #[test]
    fn transfer_sim_obj_matches_c_field_order_and_values() {
        let file = object_top_object(&transfer_sim_obj());
        assert_eq!(
            file.written_data(),
            b"{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"}"
        );
    }

    #[test]
    fn transfer_land_obj_matches_c_field_order_and_values() {
        let land = LandSnapshot::new(0, [6443, 36036], 0);
        let file = object_top_object(&transfer_land_obj(land));
        assert_eq!(
            file.written_data(),
            b"{\"date\":0,\"genetics\":[6443,36036],\"time\":0}"
        );
    }

    #[test]
    fn startup_state_seeds_land_like_c_tile_land_random() {
        let state = SimState::start_up(0x5261_f726);
        assert_eq!(state.kind(), KIND_OF_USE::KIND_START_UP);
        assert_eq!(state.land().date(), 0);
        assert_eq!(state.land().time(), 0);
        assert_eq!(state.land().genetics(), [7633, 53305]);
        assert_eq!(state.land().planet_genetics(), [2144, 24820]);
        assert_eq!(state.random_seed(), [46045, 62452]);
        assert_eq!(
            state.land_snapshot(),
            LandSnapshot::new(0, [7633, 53305], 0)
        );
        assert_eq!(state.population(), 0);
        assert_eq!(state.selected_name(), None);
    }

    #[test]
    fn land_clear_preserves_tile_genetics_and_sets_startup_clock() {
        let mut state = SimState::start_up(0x5261_f726);
        state.prepare_land_for_first_cycle();
        assert_eq!(state.land().date(), AGE_OF_MATURITY);
        assert_eq!(state.land().time(), 5 * TIME_HOUR_MINUTES as n_byte4);
        assert_eq!(state.land().genetics(), [7633, 53305]);
        assert_eq!(state.land().planet_genetics(), [0, 0]);
    }

    #[test]
    fn land_cycle_advances_time_and_rolls_day() {
        let mut land = LandState::from_snapshot(LandSnapshot::new(
            10,
            [7, 8],
            (TIME_DAY_MINUTES - 1) as n_byte4,
        ));
        land.cycle();
        assert_eq!(land.date(), 11);
        assert_eq!(land.time(), 0);
        assert_eq!(land.genetics(), [7, 8]);
    }

    #[test]
    fn land_advance_minutes_rolls_multiple_days() {
        let mut land = LandState::from_snapshot(LandSnapshot::new(2, [7, 8], 30));
        land.advance_minutes((TIME_DAY_MINUTES * 2 + 15) as n_uint);
        assert_eq!(land.date(), 4);
        assert_eq!(land.time(), 45);
        assert_eq!(land.genetics(), [7, 8]);
    }

    #[test]
    fn land_tide_height_slope_and_water_sampling_are_deterministic() {
        let mut land = LandState::from_snapshot(LandSnapshot::new(2, [7633, 53305], 719));
        land.cycle();
        assert_eq!(land.time(), 720);
        assert!((WATER_MAP - TIDE_AMPLITUDE_LUNAR - TIDE_AMPLITUDE_SOLAR
            ..=WATER_MAP + TIDE_AMPLITUDE_LUNAR + TIDE_AMPLITUDE_SOLAR)
            .contains(&n_int::from(land.tide_level())));

        let sample = land.terrain_sample([12_345, 23_456]);
        assert_eq!(sample, land.terrain_sample([12_345, 23_456]));
        assert_eq!(sample.map_position, [192, 366]);
        assert_eq!(sample.water, sample.height < n_int::from(land.tide_level()));
        assert_eq!(sample.intertidal, sample.height <= TIDE_MAX);
    }

    #[test]
    fn land_tiles_hold_primary_and_working_topography_buffers() {
        let land = LandState::from_snapshot(LandSnapshot::new(2, [7633, 53305], 719));
        let repeat = LandState::from_snapshot(LandSnapshot::new(2, [7633, 53305], 719));
        let changed = LandState::from_snapshot(LandSnapshot::new(2, [1, 2], 719));

        let tile = land
            .tile(0)
            .expect("default command line land has one tile");
        assert_eq!(tile.genetics(), [7633, 53305]);
        let primary = land
            .topography_buffer(0, LAND_TOPOGRAPHY_PRIMARY)
            .expect("primary topography buffer");
        let working = land
            .topography_buffer(0, LAND_TOPOGRAPHY_WORKING)
            .expect("working topography buffer");
        assert_eq!(primary.len(), MAP_AREA);
        assert_eq!(working.len(), MAP_AREA);
        assert_eq!(primary, working);
        assert_eq!(
            primary[map_index(192, 366)],
            land.topography_at_map(192, 366)
        );
        assert_eq!(
            land.topography_at_map(192, 366),
            repeat.topography_at_map(192, 366)
        );
        assert_ne!(
            land.topography_at_map(192, 366),
            changed.topography_at_map(192, 366)
        );
    }

    #[test]
    fn high_definition_topography_samples_match_c_bilinear_grid_shape() {
        let land = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 400));
        let map_x = 37;
        let map_y = 91;
        let hires_x = map_x << (HI_RES_MAP_BITS - MAP_BITS);
        let hires_y = map_y << (HI_RES_MAP_BITS - MAP_BITS);

        assert_eq!(
            land.topography_highdef_at(hires_x, hires_y),
            land.topography_at_map(map_x, map_y)
        );
        assert_eq!(
            land.topography_highdef_at(-1, -1),
            land.topography_highdef_at(
                (HI_RES_MAP_DIMENSION - 1) as n_int,
                (HI_RES_MAP_DIMENSION - 1) as n_int
            )
        );
        let tide_height = land.topography_highdef_at(hires_x + 3, hires_y + 5);
        assert_eq!(
            land.highres_tide_at(hires_x + 3, hires_y + 5),
            tide_height > 105 && tide_height < 151
        );
    }

    #[test]
    fn weather_seven_values_follow_native_day_night_and_dawndusk_bands() {
        let day = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 400));
        let night = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 0));
        let dawndusk = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 11 * 32));
        let location = [12_345, 23_456];

        assert!((WEATHER_SEVEN_SUNNY_DAY..=WEATHER_SEVEN_RAINY_DAY)
            .contains(&day.weather_seven_at(location)));
        assert!((WEATHER_SEVEN_CLEAR_NIGHT..=WEATHER_SEVEN_RAINY_NIGHT)
            .contains(&night.weather_seven_at(location)));
        assert_eq!(dawndusk.weather_seven_at(location), WEATHER_SEVEN_DAWN_DUSK);
    }

    #[test]
    fn food_quantity_depletes_local_energy_and_regrows_over_land_time() {
        let mut land = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 400));
        let location = (0..MAP_DIMENSION)
            .step_by(5)
            .flat_map(|map_x| {
                (0..MAP_DIMENSION).step_by(5).map(move |map_y| {
                    [
                        (map_x << APE_TO_MAP_BIT_RATIO) as n_byte2,
                        (map_y << APE_TO_MAP_BIT_RATIO) as n_byte2,
                    ]
                })
            })
            .find(|location| {
                land.height_at(*location) > TIDE_MAX
                    && land.food_source_at(*location).max_energy > BEING_DEAD
            })
            .expect("seeded land should expose edible food quantity");
        let before_source = land.food_source_at(location);
        let before_quantity = land.food_quantity_at(location);

        let depleted_quantity = land.consume_food_at(location, 320);

        assert!(depleted_quantity < before_quantity);
        assert!(land.food_source_at(location).max_energy < before_source.max_energy);
        land.advance_minutes(FOOD_REGROWTH_INTERVAL_MINUTES);
        assert!(land.food_quantity_at(location) > depleted_quantity);
    }

    #[test]
    fn repeated_eating_depletes_then_long_regrowth_restores_food_quantity() {
        let mut land = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 400));
        let location = (0..MAP_DIMENSION)
            .step_by(5)
            .flat_map(|map_x| {
                (0..MAP_DIMENSION).step_by(5).map(move |map_y| {
                    [
                        (map_x << APE_TO_MAP_BIT_RATIO) as n_byte2,
                        (map_y << APE_TO_MAP_BIT_RATIO) as n_byte2,
                    ]
                })
            })
            .find(|location| {
                land.height_at(*location) > TIDE_MAX
                    && land.food_source_at(*location).max_energy > BEING_DEAD
            })
            .expect("seeded land should expose edible food");
        let mut being = BeingSummary::new(
            "Repeater".to_string(),
            512,
            258,
            0,
            [n_genetics::MAX; CHROMOSOMES],
        );
        being.location = location;

        let first = being.eat_food(&mut land);
        let second = being.eat_food(&mut land);

        assert!(first.energy > BEING_DEAD);
        assert!(second.max_energy < first.max_energy);
        assert!(land.food_quantity_at(location) < FOOD_QUANTITY_MAX);
        land.advance_minutes(FOOD_REGROWTH_INTERVAL_MINUTES * n_uint::from(FOOD_QUANTITY_MAX));
        assert_eq!(land.food_quantity_at(location), FOOD_QUANTITY_MAX);
        assert_eq!(land.food_source_at(location).max_energy, first.max_energy);
    }

    #[test]
    fn inventory_food_hooks_cover_native_chew_foods() {
        let mut being = BeingSummary::new(
            "Carrier".to_string(),
            512,
            258,
            0,
            [n_genetics::MAX; CHROMOSOMES],
        );
        for (item, food_type, max_energy) in [
            (INVENTORY_FISH, FOOD_SHELLFISH, ENERGY_FISH),
            (INVENTORY_NUT_CRACKED, FOOD_VEGETABLE, ENERGY_NUT),
            (INVENTORY_BIRD_EGGS, FOOD_BIRD_EGGS, ENERGY_BIRD_EGGS),
            (INVENTORY_LIZARD_EGGS, FOOD_LIZARD_EGGS, ENERGY_LIZARD_EGGS),
            (INVENTORY_GRASS, FOOD_VEGETABLE, ENERGY_GRASS),
        ] {
            being.inventory = [0; INVENTORY_SIZE];
            being.inventory[usize::from(BODY_LEFT_HAND)] = item;
            let food = being
                .consume_inventory_food()
                .expect("native chew food should be edible");
            assert_eq!(food.food_type, food_type);
            assert_eq!(food.max_energy, max_energy);
            assert!(food.energy > BEING_DEAD);
            assert_eq!(being.inventory[usize::from(BODY_LEFT_HAND)], 0);
        }
        assert!(inventory_food_source(INVENTORY_TWIG).is_none());
    }

    #[test]
    fn terrain_food_fixture_probe_matches_checked_values() {
        let land = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 400));
        let probes = [
            land.terrain_food_fixture_sample([12_345, 23_456]),
            land.terrain_food_fixture_sample([3_200, 9_600]),
            land.terrain_food_fixture_sample([28_000, 4_000]),
        ];
        assert_eq!(
            probes,
            [
                TerrainFoodFixtureSample {
                    location: [12_345, 23_456],
                    height: 174,
                    grass: 133,
                    trees: -18,
                    bush: 122,
                    seaweed: 0,
                    rockpool: 0,
                    beach: 0,
                    food_type: 0,
                    max_energy: 50,
                },
                TerrainFoodFixtureSample {
                    location: [3_200, 9_600],
                    height: 207,
                    grass: 123,
                    trees: 86,
                    bush: 109,
                    seaweed: 0,
                    rockpool: 0,
                    beach: 0,
                    food_type: 0,
                    max_energy: 50,
                },
                TerrainFoodFixtureSample {
                    location: [28_000, 4_000],
                    height: 207,
                    grass: 116,
                    trees: 37,
                    bush: 109,
                    seaweed: 0,
                    rockpool: 0,
                    beach: 0,
                    food_type: 0,
                    max_energy: 50,
                },
            ]
        );
    }

    #[test]
    fn terrain_biology_drives_land_and_intertidal_food_sources() {
        let land = LandState::from_snapshot(LandSnapshot::new(5, [7633, 53305], 400));
        let mut saw_land_food = false;
        let mut saw_intertidal_food = false;
        let mut saw_shellfish = false;
        let mut saw_seaweed = false;

        for map_x in (0..MAP_DIMENSION).step_by(7) {
            for map_y in (0..MAP_DIMENSION).step_by(11) {
                let location = [
                    (map_x << APE_TO_MAP_BIT_RATIO) as n_byte2,
                    (map_y << APE_TO_MAP_BIT_RATIO) as n_byte2,
                ];
                let food = land.food_source_at(location);
                let height = land.height_at(location);
                if height > TIDE_MAX && food.max_energy > BEING_DEAD {
                    saw_land_food = true;
                    assert!(matches!(food.food_type, FOOD_VEGETABLE | FOOD_FRUIT));
                }
                if height <= TIDE_MAX {
                    saw_intertidal_food = true;
                    if food.food_type == FOOD_SHELLFISH {
                        saw_shellfish = true;
                        assert_eq!(food.max_energy, ENERGY_SHELLFISH);
                    }
                    if food.food_type == FOOD_SEAWEED {
                        saw_seaweed = true;
                        assert_eq!(food.max_energy, ENERGY_SEAWEED);
                    }
                }
            }
        }

        assert!(saw_land_food);
        assert!(saw_intertidal_food);
        assert!(saw_shellfish);
        assert!(saw_seaweed);
    }

    #[test]
    fn startup_state_transfer_json_uses_seeded_land_snapshot() {
        let state = SimState::start_up(0x5261_f726);
        let file = state.tranfer_startup_out_json();
        assert_eq!(
            file.written_data(),
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":0,\"genetics\":[7633,53305],\"time\":0}}"
        );
        assert_eq!(file.location(), 177);
    }

    #[test]
    fn startup_transfer_loads_from_json_bytes() {
        let startup = startup_transfer_from_json_bytes(
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":27,\"genetics\":[1,65535],\"time\":300}}",
        )
        .unwrap();
        assert_eq!(startup.land, LandSnapshot::new(27, [1, 65_535], 300));
        assert!(startup.beings.is_empty());
    }

    #[test]
    fn native_transfer_sections_parse_version_and_land_blocks() {
        let native = b"/* Simulated Ape 0.708 */\n\
            simul{signa=20033;verio=708;};\n\
            landd{dated=27;timed=300;landg=1,65535;};";
        let sections = native_transfer_sections(native).unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].token_string(), "simul{");
        assert_eq!(sections[0].field_values(b"signa=").unwrap(), &[20_033]);
        assert_eq!(sections[1].token_string(), "landd{");
        assert_eq!(sections[1].field_values(b"landg=").unwrap(), &[1, 65_535]);

        let startup = startup_transfer_from_native_bytes(native).unwrap();
        assert_eq!(startup.land, LandSnapshot::new(27, [1, 65_535], 300));
        assert!(startup.beings.is_empty());
    }

    #[test]
    fn native_transfer_loads_land_and_being_constant_delta_fields() {
        let native = b"simul{signa=20033;verio=708;};\
            landd{dated=9;timed=13;landg=11,12;};\
            being{\
            locat=100,200;facin=64;speed=3;energ=1234;datob=8;rando=5,6;state=1;\
            brast=1,2,3,4,5,6;heigt=2100;masss=120;overr=12;\
            shout=1,2,3,4,5,6;crowd=1;postu=4;inven=0,4096,0,0,0,0,0,0;\
            paras=2;honor=77;conce=99;atten=0,1,2,3,4,5;\
            genet=3,0,10,0,11,0,12,0;fetag=9,0,8,0,7,0,6,0;fathn=5,6;\
            sosim=10,11,12,13;drive=1,2,3,4;goals=0,1,2,3;\
            prefe=1,1,1,1,1,1,1,1,2,3,4,5,6,7;\
            genex=2;genen=1;chigx=4;chign=3;};";
        let state = SimState::load_startup_bytes(native).unwrap();
        let being = &state.beings()[0];

        assert_eq!(state.kind(), KIND_OF_USE::KIND_LOAD_FILE);
        assert_eq!(state.land_snapshot(), LandSnapshot::new(9, [11, 12], 13));
        assert_eq!(state.population(), 1);
        assert_eq!(being.name(), "Ape 001");
        assert_eq!(being.location(), [100, 200]);
        assert_eq!(being.facing(), 64);
        assert_eq!(being.speed(), 3);
        assert_eq!(being.energy(), 1234);
        assert_eq!(being.date_of_birth(), 8);
        assert_eq!(being.random_seed(), [5, 6]);
        assert_eq!(being.macro_state(), 1);
        assert_eq!(being.brain_state(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(being.height(), 2100);
        assert_eq!(being.mass(), 120);
        assert_eq!(being.script_overrides(), 12);
        assert_eq!(being.shout(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(being.inventory()[1], INVENTORY_FISH);
        assert_eq!(being.parasites(), 2);
        assert_eq!(being.honor(), 77);
        assert_eq!(being.attention(), [0, 1, 2, 3, 4, 5]);
        assert_eq!(being.genetics(), [3, 10, 11, 12]);
        assert_eq!(being.social_coord(), [10, 11, 12, 13]);
        assert_eq!(being.drives(), [1, 2, 3, 4]);
        assert_eq!(being.goal(), [0, 1, 2, 3]);
        assert_eq!(
            being.learned_preference(),
            [1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 7]
        );
    }

    #[test]
    fn native_transfer_loads_events_brain_probes_and_immune_bytes() {
        let brpro = (0..BRAINPROBE_NATIVE_BYTES)
            .map(|index| {
                if index < 6 {
                    (index + 1).to_string()
                } else {
                    "0".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        let immun = [
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            vec![8, 7, 6, 5, 4, 3, 2, 1],
            vec![1; IMMUNE_POPULATION],
            vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![55, 0, 66, 0],
        ]
        .concat()
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",");
        let native = format!(
            "simul{{signa=20033;verio=708;}};\
            landd{{dated=0;timed=0;landg=1,2;}};\
            being{{locat=100,200;facin=64;speed=3;energ=1234;datob=8;rando=5,6;state=1;\
            brast=1,2,3,4,5,6;heigt=2100;masss=120;overr=12;\
            shout=1,2,3,4,5,6;crowd=1;postu=4;inven=0,0,0,0,0,0,0,0;\
            paras=2;honor=77;conce=99;atten=0,1,2,3,4,5;\
            genet=3,0,10,0,11,0,12,0;fetag=9,0,8,0,7,0,6,0;fathn=5,6;\
            sosim=10,11,12,13;drive=1,2,3,4;goals=0,1,2,3;\
            prefe=1,1,1,1,1,1,1,1,2,3,4,5,6,7;\
            genex=2;genen=1;chigx=4;chign=3;immun={immun};brreg=65,66,67;brpro={brpro};}};\
            sgcia{{sgloc=10,20;sgtim=30;sgdat=40;sgfin=111;sgfan=222;sgatt=3;sgfof=127;sgbel=9;sgfam=42;sgrel=2;sgtyp=0;}};\
            episo{{eploc=2,3;eptim=4;epdat=1;epfin=512,0;epfan=258,0;epeve=1;epfoo=0;epafe=16434;eparg=50;}};"
        );
        let state = SimState::load_startup_bytes(native.as_bytes()).unwrap();
        let being = &state.beings()[0];
        let social = being.social_memory()[0];
        let episodic = being.episodic_memory()[0];

        assert_eq!(being.braincode_register(), [65, 66, 67]);
        assert_eq!(being.brainprobe()[0].probe_type, 1);
        assert_eq!(being.brainprobe()[0].state, 6);
        assert_eq!(being.immune_antigens(), [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(being.immune_shape_antigen(), [8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(being.immune_antibodies(), [1; IMMUNE_POPULATION]);
        assert_eq!(being.immune_seed(), [55, 66]);
        assert_eq!(social.space_time.location, [10, 20]);
        assert_eq!(social.first_name[BEING_MET], 111);
        assert_eq!(social.familiarity, 42);
        assert_eq!(episodic.event, EVENT_EAT);
        assert_eq!(episodic.arg, 50);
    }

    #[test]
    fn native_transfer_writer_roundtrips_land_population_and_events() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":7,\"genetics\":[11,12],\"time\":13},\"beings\":[{\"name\":\"Expanded Ape\",\"delta\":{\"location\":[100,200],\"direction_facing\":64,\"velocity\":3,\"stored_energy\":3840,\"random_seed\":[5,6],\"macro_state\":1},\"constant\":{\"date_of_birth\":8,\"name\":[512,258],\"genetics\":[2,3,4,5],\"generation_range\":[1,2]},\"changes\":{\"drives\":[1,2,3,4],\"inventory\":[0,4096,0,0,0,0,0,0],\"learned_preference\":[1,1,1,1,1,1,1,1,2,3,4,5,6,7]},\"braindata\":{\"braincode_register\":[65,66,67],\"brain_state\":[1,2,3,4,5,6],\"script_overrides\":12,\"attention\":[0,1,2,3,4,5],\"brainprobe\":[{\"probe_type\":1,\"position\":2,\"address\":3,\"frequency\":4,\"offset\":5,\"state\":6}]},\"immune_system\":{\"antigens\":[1,2,3,4,5,6,7,8],\"shape_antigen\":[8,7,6,5,4,3,2,1],\"antibodies\":[1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2],\"shape_antibody\":[16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1],\"random_seed\":[55,66]},\"events\":{\"episodic\":[{\"space_time\":{\"date\":1,\"location\":[2,3],\"time\":4},\"first_name\":[512,0],\"family_name\":[258,0],\"event\":1,\"food\":0,\"affect\":16434,\"arg\":50}]}}]}",
        )
        .unwrap();
        let native = state.tranfer_startup_out_native();
        let native_text = std::str::from_utf8(native.written_data()).unwrap();
        assert!(native_text.starts_with("simul{signa=20033;verio=708;};"));
        assert!(native_text.contains("landd{dated=7;timed=13;landg=11,12;};"));
        assert!(native_text.contains("being{locat=100,200;"));
        assert!(native_text.contains("episo{"));

        let loaded = SimState::load_startup_bytes(native.written_data()).unwrap();
        let being = &loaded.beings()[0];
        assert_eq!(loaded.land_snapshot(), LandSnapshot::new(7, [11, 12], 13));
        assert_eq!(being.location(), [100, 200]);
        assert_eq!(being.braincode_register(), [65, 66, 67]);
        assert_eq!(being.brainprobe()[0].state, 6);
        assert_eq!(being.immune_seed(), [55, 66]);
        assert_eq!(being.episodic_memory()[0].event, EVENT_EAT);
    }

    #[test]
    fn native_transfer_rejects_unsupported_sections_and_raw_binary() {
        assert!(startup_transfer_from_native_bytes(b"NA\x02\xc4not-native").is_err());
        assert!(startup_transfer_from_native_bytes(
            b"simul{signa=20033;verio=708;};landd{dated=0;timed=0;landg=1,2;};weath{press=1;};"
        )
        .is_err());
        assert!(startup_transfer_from_native_bytes(b"landd{dated=0;timed=0;landg=1,2;};").is_err());
        assert!(startup_transfer_from_native_bytes(b"simul{signa=20033;verio=9999;};").is_err());
        assert!(startup_transfer_from_native_bytes(
            b"simul{signa=20033;verio=708;};sgcia{sgfin=1;};"
        )
        .is_err());
    }

    #[test]
    fn sim_state_load_startup_json_restores_land_snapshot() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":9,\"genetics\":[11,12],\"time\":13}}",
        )
        .unwrap();
        assert_eq!(state.kind(), KIND_OF_USE::KIND_LOAD_FILE);
        assert_eq!(state.random_seed(), [0, 0]);
        assert_eq!(state.land_snapshot(), LandSnapshot::new(9, [11, 12], 13));
        assert_eq!(state.land().planet_genetics(), [11, 12]);
        assert_eq!(state.population(), 0);
    }

    #[test]
    fn sim_state_load_startup_json_restores_being_summaries() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":9,\"genetics\":[11,12],\"time\":13},\"beings\":[{\"name\":\"Ape 001\",\"gender name\":512,\"family name\":258,\"date of birth\":0,\"generation min\":0,\"generation max\":0,\"genetics\":[2,3,4,5]}]}",
        )
        .unwrap();
        assert_eq!(state.population(), 1);
        assert_eq!(state.selected_name(), Some("Ape 001"));
        assert_eq!(state.beings()[0].family_name(), 258);
        assert_eq!(state.beings()[0].genetics(), [2, 3, 4, 5]);
    }

    #[test]
    fn sim_state_load_startup_json_accepts_native_c_being_shape() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":9,\"genetics\":[11,12],\"time\":13},\"beings\":[{\"name\":\"Native Ape\",\"delta\":{\"direction_facing\":64,\"location\":[100,200],\"velocity\":3,\"stored_energy\":1234,\"random_seed\":[5,6],\"macro_state\":1,\"parasites\":2,\"honor\":77,\"crowding\":1,\"height\":2100,\"mass\":120,\"posture\":4,\"goal\":[0,1,2,3],\"social_coord\":[10,11,12,13]},\"constant\":{\"date_of_birth\":8,\"genetics\":[3,10,11,12],\"generation_range\":[1,2]}}]}",
        )
        .unwrap();
        let being = &state.beings()[0];
        assert_eq!(state.population(), 1);
        assert_eq!(state.selected_name(), Some("Native Ape"));
        assert_eq!(being.location(), [100, 200]);
        assert_eq!(being.facing(), 64);
        assert_eq!(being.random_seed(), [5, 6]);
        assert_eq!(being.energy(), 1234);
        assert_eq!(being.speed(), 3);
        assert_eq!(being.macro_state(), 1);
        assert_eq!(being.parasites(), 2);
        assert_eq!(being.honor(), 77);
        assert_eq!(being.crowding(), 1);
        assert_eq!(being.height(), 2100);
        assert_eq!(being.mass(), 120);
        assert_eq!(being.posture(), 4);
        assert_eq!(being.goal(), [0, 1, 2, 3]);
        assert_eq!(being.social_coord(), [10, 11, 12, 13]);
        assert!(being.awake());
        assert!(being.is_female());
    }

    #[test]
    fn native_c_being_shape_loads_full_immune_arrays() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":0},\"beings\":[{\"name\":\"Immune Ape\",\"delta\":{\"stored_energy\":3840},\"constant\":{\"date_of_birth\":0,\"genetics\":[2,3,4,5]},\"immune_system\":{\"antigens\":[1,2,3,4,5,6,7,8],\"shape_antigen\":[8,7,6,5,4,3,2,1],\"antibodies\":[1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2],\"shape_antibody\":[16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1],\"random_seed\":[55,66]}}]}",
        )
        .unwrap();
        let being = &state.beings()[0];
        assert_eq!(being.immune_antigens(), [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(being.immune_shape_antigen(), [8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(
            being.immune_antibodies(),
            [1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2]
        );
        assert_eq!(
            being.immune_shape_antibody(),
            [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        );
        assert_eq!(being.immune_seed(), [55, 66]);
    }

    #[test]
    fn being_summary_projects_to_and_from_native_simulated_being() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        let first = state.beings()[0].clone();
        let native = first.to_simulated_being();
        assert_eq!(native.constant.date_of_birth, first.date_of_birth());
        assert_eq!(
            native.constant.name,
            [first.gender_name(), first.family_name()]
        );
        assert_eq!(native.constant.genetics, first.genetics());
        assert_eq!(native.delta.location, first.location());
        assert_eq!(native.delta.direction_facing, first.facing());
        assert_eq!(native.delta.random_seed, first.random_seed());
        assert_eq!(native.delta.stored_energy, first.energy());
        assert_eq!(native.delta.velocity[0], first.speed());
        assert_eq!(native.changes.drives, first.drives());
        assert_eq!(
            native.braindata.braincode_register,
            first.braincode_register()
        );
        assert_eq!(native.immune_system.antigens, first.immune_antigens());
        assert_eq!(
            native.immune_system.shape_antigen,
            first.immune_shape_antigen()
        );
        assert_eq!(native.immune_system.antibodies, first.immune_antibodies());
        assert_eq!(
            native.immune_system.shape_antibody,
            first.immune_shape_antibody()
        );
        assert_eq!(native.immune_system.random_seed, first.immune_seed());

        let summary = BeingSummary::from_simulated_being(first.name().to_string(), &native);
        assert_eq!(summary, first);
    }

    #[test]
    fn startup_transfer_roundtrips_extended_being_summary_fields() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        let first = state.beings()[0].clone();
        let saved = state.tranfer_startup_out_json();
        let saved_json =
            std::str::from_utf8(saved.written_data()).expect("startup transfer should be utf8");

        assert!(saved_json.contains("\"delta\""));
        assert!(saved_json.contains("\"constant\""));
        assert!(saved_json.contains("\"direction_facing\""));
        assert!(saved_json.contains("\"stored_energy\""));
        assert!(saved_json.contains("\"location\""));
        assert!(saved_json.contains("\"drives\""));
        assert!(saved_json.contains("\"braincode_register\""));
        assert!(saved_json.contains("\"immune_system\""));

        let loaded =
            SimState::load_startup_json(saved.written_data()).expect("saved JSON should reload");
        let loaded_first = &loaded.beings()[0];
        assert_eq!(loaded.population(), INITIAL_POPULATION);
        assert_eq!(loaded.selected_name(), Some("Ape 001"));
        assert_eq!(loaded_first.location(), first.location());
        assert_eq!(loaded_first.facing(), first.facing());
        assert_eq!(loaded_first.random_seed(), first.random_seed());
        assert_eq!(loaded_first.energy(), first.energy());
        assert_eq!(loaded_first.speed(), first.speed());
        assert_eq!(loaded_first.macro_state(), first.macro_state());
        assert_eq!(loaded_first.parasites(), first.parasites());
        assert_eq!(loaded_first.honor(), first.honor());
        assert_eq!(loaded_first.crowding(), first.crowding());
        assert_eq!(loaded_first.height(), first.height());
        assert_eq!(loaded_first.mass(), first.mass());
        assert_eq!(loaded_first.posture(), first.posture());
        assert_eq!(loaded_first.goal(), first.goal());
        assert_eq!(loaded_first.social_coord(), first.social_coord());
        assert_eq!(loaded_first.awake(), first.awake());
        assert_eq!(loaded_first.drives(), first.drives());
        assert_eq!(
            loaded_first.braincode_register(),
            first.braincode_register()
        );
        assert_eq!(loaded_first.immune_antigens(), first.immune_antigens());
        assert_eq!(
            loaded_first.immune_shape_antigen(),
            first.immune_shape_antigen()
        );
        assert_eq!(loaded_first.immune_antibodies(), first.immune_antibodies());
        assert_eq!(
            loaded_first.immune_shape_antibody(),
            first.immune_shape_antibody()
        );
        assert_eq!(loaded_first.immune_seed(), first.immune_seed());
    }

    #[test]
    fn population_age_counts_follow_land_date() {
        let mut state = SimState::start_up(0x5261_f726);
        assert_eq!(state.adult_count(), 0);
        assert_eq!(state.juvenile_count(), 0);

        state.reset_new_simulation_from_land_seed();
        assert_eq!(state.population(), INITIAL_POPULATION);
        assert_eq!(state.adult_count(), 0);
        assert_eq!(state.juvenile_count(), INITIAL_POPULATION);

        state.prepare_land_for_first_cycle();
        assert_eq!(state.adult_count(), INITIAL_POPULATION);
        assert_eq!(state.juvenile_count(), 0);
    }

    #[test]
    fn advance_minutes_cycles_populated_beings_and_land_time() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        let before = state.beings()[0].clone();
        state.advance_minutes(400);
        let after = &state.beings()[0];

        assert_eq!(state.land().time(), 400);
        assert_eq!(state.land().date(), 0);
        assert_eq!(state.kind(), KIND_OF_USE::KIND_NOTHING_TO_RUN);
        assert_ne!(after.location(), before.location());
        assert_ne!(after.facing(), before.facing());
        assert!(after.energy() < before.energy());
        assert_ne!(after.drives(), before.drives());
    }

    #[test]
    fn awake_level_follows_energy_time_and_speed_like_c() {
        let mut being = BeingSummary::new("Sleeper".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        being.energy_delta(i32::from(BEING_FULL));
        assert_eq!(being.awake_level_for_time(0), FULLY_ASLEEP);
        assert_eq!(being.awake_level_for_time(400), FULLY_AWAKE);
        being.set_speed(1);
        assert_eq!(being.awake_level_for_time(0), SLIGHTLY_AWAKE);
        being.set_speed(0);
        being.energy = BEING_HUNGRY;
        assert_eq!(being.awake_level_for_time(0), SLIGHTLY_AWAKE);
        being.energy = BEING_DEAD;
        assert_eq!(being.awake_level_for_time(400), FULLY_ASLEEP);
    }

    #[test]
    fn immune_initialization_and_response_are_seeded_and_persistent() {
        let mut being = BeingSummary::new("Immune".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        being.immune_seed = [123, 456];
        being.init_immune();
        assert_eq!(being.immune_antigens(), [0; IMMUNE_ANTIGENS]);
        assert_eq!(being.immune_antibodies(), [0; IMMUNE_POPULATION]);
        assert_ne!(being.immune_shape_antigen(), [0; IMMUNE_ANTIGENS]);
        assert_ne!(being.immune_shape_antibody(), [0; IMMUNE_POPULATION]);

        being.immune_antigens[0] = 10;
        being.immune_shape_antigen[0] = 200;
        let before_seed = being.immune_seed();
        let energy_cost = being.immune_response();
        assert!(energy_cost >= 0);
        assert_ne!(being.immune_seed(), before_seed);
    }

    #[test]
    fn drive_cycle_updates_hunger_fatigue_social_and_maturity() {
        let mut being = BeingSummary::new("Driven".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        being.energy = BEING_HUNGRY - 1;
        being.set_speed(FATIGUE_SPEED_THRESHOLD + 1);
        being.awake_level = FULLY_AWAKE;
        being.drives = [0; DRIVES];
        being.cycle_drives(AGE_OF_MATURITY + 1);
        assert_eq!(being.drive(DRIVE_HUNGER), 1);
        assert_eq!(being.drive(DRIVE_SOCIAL), 1);
        assert_eq!(being.drive(DRIVE_FATIGUE), 1);
        assert_eq!(being.drive(DRIVE_SEX), 0);
    }

    #[test]
    fn facing_vectors_wander_and_towards_match_toolkit_helpers() {
        let mut being = BeingSummary::new("Facing".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        being.facing = 0;
        assert_eq!(being.facing_vector(1), vect2_direction(0, 32));
        assert_eq!(being.facing_vector(4), vect2_direction(0, 128));

        being.set_facing_towards(n_vect2::new(0, 10));
        assert_eq!(being.facing(), 64);
        being.wander(-65);
        assert_eq!(being.facing(), 255);
        being.wander(2);
        assert_eq!(being.facing(), 1);
    }

    #[test]
    fn speed_history_advance_and_move_energy_follow_native_shape() {
        let land = LandState::from_snapshot(LandSnapshot::new(0, [7633, 53305], 400));
        let mut being = BeingSummary::new("Speedy".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        being.set_speed(5);
        being.speed_advance();
        being.set_speed(7);
        assert_eq!(being.velocity()[0], 7);
        assert_eq!(being.velocity()[1], 5);
        assert_eq!(being.ten_minute_distance(), 12);

        being.set_speed(0);
        let resting = being.move_energy(&land);
        being.set_speed(20);
        assert!(being.move_energy(&land) > resting);
    }

    #[test]
    fn awake_cycle_eats_grows_recalculates_mass_and_sets_state() {
        let mut land = LandState::from_snapshot(LandSnapshot::new(0, [7633, 53305], 400));
        let mut being = BeingSummary::new(
            "Hungry".to_string(),
            512,
            258,
            0,
            [n_genetics::MAX; CHROMOSOMES],
        );
        being.energy = BEING_HUNGRY - 1;
        being.random_seed = [1, 2];
        being.location = (0..MAP_DIMENSION)
            .step_by(5)
            .flat_map(|map_x| {
                (0..MAP_DIMENSION).step_by(5).map(move |map_y| {
                    [
                        (map_x << APE_TO_MAP_BIT_RATIO) as n_byte2,
                        (map_y << APE_TO_MAP_BIT_RATIO) as n_byte2,
                    ]
                })
            })
            .find(|location| {
                land.height_at(*location) > TIDE_MAX
                    && land.food_source_at(*location).max_energy > BEING_DEAD
            })
            .expect("seeded land should expose edible land food");
        let before_energy = being.energy();
        let before_height = being.height();
        let before_food_quantity = land.food_quantity_at(being.location);

        being.cycle_awake(&mut land);

        assert!(being.energy() > before_energy);
        assert!(being.height() > before_height);
        assert!(land.food_quantity_at(being.location) < before_food_quantity);
        assert!(being.macro_state() & BEING_STATE_EATING != 0);
        assert!(being.mass() >= being.calculated_mass());
    }

    #[test]
    fn food_absorption_supports_multiple_food_types_and_ingests_pathogens() {
        let mut being = BeingSummary::new(
            "Forager".to_string(),
            512,
            258,
            0,
            [n_genetics::MAX; CHROMOSOMES],
        );
        assert!(being.food_absorption(ENERGY_GRASS, FOOD_VEGETABLE) > 0);
        assert!(being.food_absorption(ENERGY_FRUIT, FOOD_FRUIT) > 0);
        assert!(being.food_absorption(ENERGY_SHELLFISH, FOOD_SHELLFISH) > 0);
        assert!(being.food_absorption(ENERGY_SEAWEED, FOOD_SEAWEED) > 0);
        assert!(being.food_absorption(ENERGY_BIRD_EGGS, FOOD_BIRD_EGGS) <= 320);
        assert!(being.food_absorption(ENERGY_LIZARD_EGGS, FOOD_LIZARD_EGGS) <= 320);

        let mut infected = None;
        for seed in 0..10_000 {
            let mut candidate = being.clone();
            candidate.immune_seed = [seed, seed ^ 0x55aa];
            let _ = candidate.food_absorption(ENERGY_SHELLFISH, FOOD_SHELLFISH);
            if candidate.immune_antigens().iter().any(|value| *value > 0) {
                infected = Some(candidate);
                break;
            }
        }
        let infected = infected.expect("pathogen environment probability should be reachable");
        assert!(infected
            .immune_shape_antigen()
            .iter()
            .any(|shape| shape & 7 == FOOD_SHELLFISH + PATHOGEN_TRANSMISSION_FOOD_VEGETABLE));
    }

    #[test]
    fn body_and_social_native_state_initializes_and_roundtrips() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        let first = &state.beings()[0];
        assert_eq!(body_inventory_description(BODY_HEAD), "Head");
        assert_eq!(first.body_attention_description(), "Head");
        assert_eq!(first.social_memory()[0].relationship, RELATIONSHIP_SELF);
        assert_eq!(first.social_memory()[0].friend_foe, SOCIAL_RESPECT_NORMAL);
        assert_eq!(first.social_memory()[0].entity_type, ENTITY_BEING);

        let saved = state.tranfer_startup_out_json();
        let saved_json =
            std::str::from_utf8(saved.written_data()).expect("startup transfer should be utf8");
        assert!(saved_json.contains("\"events\":{\"social\""));
        assert!(saved_json.contains("\"attention\""));

        let loaded =
            SimState::load_startup_json(saved.written_data()).expect("saved JSON should reload");
        assert_eq!(loaded.beings()[0].social_memory(), first.social_memory());
        assert_eq!(loaded.beings()[0].attention(), first.attention());
    }

    #[test]
    fn native_social_memory_loads_c_shaped_summary_fields() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":0},\"beings\":[{\"name\":\"Social Ape\",\"delta\":{\"stored_energy\":3840},\"constant\":{\"date_of_birth\":0,\"genetics\":[2,3,4,5]},\"events\":{\"social\":[{\"relationship\":1,\"entity_type\":0,\"friend_foe\":127},{\"first_name\":[0,111],\"family_name\":[0,222],\"friend_foe\":127,\"familiarity\":42,\"relationship\":2,\"entity_type\":0,\"attraction\":1,\"belief\":9,\"classification\":{\"feature_number\":3,\"observations\":4}}]}}]}",
        )
        .unwrap();
        let entry = &state.beings()[0].social_memory()[1];
        assert_eq!(entry.first_name, [0, 111]);
        assert_eq!(entry.family_name, [0, 222]);
        assert_eq!(entry.familiarity, 42);
        assert_eq!(entry.relationship, 2);
        assert_eq!(entry.attraction, 1);
        assert_eq!(entry.belief, 9);
        assert_eq!(entry.classification.feature_number, 3);
        assert_eq!(entry.classification.observations, 4);
    }

    #[test]
    fn social_loops_meet_close_beings_and_record_interactions() {
        let mut first = BeingSummary::new("First".to_string(), 512, 100, 0, [2, 3, 4, 5]);
        let mut second = BeingSummary::new("Second".to_string(), 768, 200, 0, [3, 4, 5, 6]);
        first.location = [100, 100];
        second.location = [120, 120];
        first.energy = BEING_FULL;
        second.energy = BEING_FULL;

        let mut population = PopulationState::from_beings(vec![first, second], 2);
        for _ in 0..20 {
            population.social_initial_loop(AGE_OF_MATURITY + 1, 400);
            population.social_secondary_loop_no_sim();
        }

        let first = &population.beings()[0];
        let second = &population.beings()[1];
        assert_eq!(
            first.social_memory()[1].first_name[BEING_MET],
            second.gender_name()
        );
        assert_eq!(
            second.social_memory()[1].first_name[BEING_MET],
            first.gender_name()
        );
        assert!(first.social_memory()[1].familiarity >= 20);
        assert!(first.social_memory()[1].friend_foe > SOCIAL_RESPECT_NORMAL);
        assert!(first.macro_state() & BEING_STATE_GROOMING != 0);
        assert!(first
            .episodic_memory()
            .iter()
            .any(|entry| entry.event == EVENT_GROOM));
        assert!(second
            .episodic_memory()
            .iter()
            .any(|entry| entry.event == EVENT_GROOMED));
    }

    #[test]
    fn social_meeting_records_feature_classification_and_initial_prejudice() {
        let mut first = BeingSummary::new("Observer".to_string(), 512, 100, 0, [2, 3, 4, 5]);
        let second = BeingSummary::new("Observed".to_string(), 768, 200, 0, [9, 8, 7, 6]);
        let index = first.meet_being(&second, 12, 345);
        let entry = first.social_memory()[index];

        assert_eq!(entry.first_name[BEING_MET], second.gender_name());
        assert_eq!(entry.family_name[BEING_MET], second.family_name());
        assert!(entry.friend_foe > 0);
        assert_eq!(entry.classification.observations, 1);
        assert!(entry.classification.feature_number >= 11);
        assert!(entry
            .classification
            .features
            .iter()
            .take(entry.classification.feature_number as usize)
            .any(|feature| feature.feature_type == FEATURESET_HEIGHT
                && feature.value == second.height()));
    }

    #[test]
    fn social_features_load_and_save_full_feature_arrays() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":0},\"beings\":[{\"name\":\"Feature Ape\",\"delta\":{\"stored_energy\":3840},\"constant\":{\"date_of_birth\":0,\"genetics\":[2,3,4,5]},\"events\":{\"social\":[{\"relationship\":1,\"entity_type\":0,\"friend_foe\":127},{\"first_name\":[0,111],\"family_name\":[0,222],\"friend_foe\":127,\"familiarity\":42,\"relationship\":2,\"entity_type\":0,\"classification\":{\"feature_number\":2,\"observations\":3,\"features\":[{\"type\":0,\"value\":9,\"frequency\":2},{\"type\":2,\"value\":2000,\"frequency\":1}]}}]}}]}",
        )
        .unwrap();
        let entry = &state.beings()[0].social_memory()[1];
        assert_eq!(entry.classification.feature_number, 2);
        assert_eq!(
            entry.classification.features[0].feature_type,
            FEATURESET_PIGMENTATION
        );
        assert_eq!(entry.classification.features[1].value, 2000);

        let saved = state.tranfer_startup_out_json();
        let saved_json = std::str::from_utf8(saved.written_data()).unwrap();
        assert!(saved_json.contains("\"features\":[{\"type\":0,\"value\":9,\"frequency\":2}"));
    }

    #[test]
    fn grooming_marks_body_removes_parasites_and_records_touch_memory() {
        let mut groomer = BeingSummary::new("Groomer".to_string(), 512, 100, 0, [2, 3, 4, 5]);
        let mut groomee = BeingSummary::new("Groomee".to_string(), 768, 200, 0, [3, 4, 5, 6]);
        groomer.learned_preference[PREFERENCE_GROOM_FEMALE] = 255;
        groomer.attention[ATTENTION_BODY] = BODY_LEFT_HAND;
        groomer.random_seed = [1, 2];
        groomee.honor = 255;
        groomee.parasites = 5;
        let groomer_index = groomer.meet_being(&groomee, 1, 1);
        let groomee_index = groomee.meet_being(&groomer, 1, 1);

        assert!(social_groom_native(
            &mut groomer,
            &mut groomee,
            groomer_index,
            groomee_index,
            1,
            16,
            1,
            2,
        ));
        assert!(groomee.inventory()[usize::from(BODY_LEFT_HAND)] & INVENTORY_GROOMED != 0);
        assert_eq!(groomee.parasites(), 3);
        assert!(groomer
            .episodic_memory()
            .iter()
            .any(|entry| entry.event == EVENT_GROOM));
        assert!(groomee
            .episodic_memory()
            .iter()
            .any(|entry| entry.event == EVENT_GROOMED));
    }

    #[test]
    fn squabble_applies_disrespect_wounds_honor_and_fleeing() {
        let mut first = BeingSummary::new("First".to_string(), 512, 100, 0, [2, 3, 4, 5]);
        let mut second = BeingSummary::new("Second".to_string(), 768, 200, 0, [3, 4, 5, 6]);
        first.energy = BEING_FULL;
        second.energy = BEING_HUNGRY;
        first.random_seed = [9, 10];
        let first_index = first.meet_being(&second, 1, 1);
        let second_index = second.meet_being(&first, 1, 1);
        first.social_memory[first_index].friend_foe = SOCIAL_RESPECT_NORMAL - 40;

        assert!(social_squabble_native(
            &mut first,
            &mut second,
            first_index,
            second_index,
            1,
            1,
            2,
        ));
        assert!(first.macro_state() & BEING_STATE_ATTACK != 0);
        assert!(second.speed() == SQUABBLE_FLEE_SPEED || first.speed() == SQUABBLE_FLEE_SPEED);
        assert!(first
            .inventory()
            .iter()
            .chain(second.inventory().iter())
            .any(|item| item & INVENTORY_WOUND != 0));
        assert!(first
            .episodic_memory()
            .iter()
            .chain(second.episodic_memory().iter())
            .any(|entry| entry.event == EVENT_HIT || entry.event == EVENT_HIT_BY));
    }

    #[test]
    fn mate_and_chat_update_attraction_goal_and_anecdotes() {
        let mut first = BeingSummary::new("Male".to_string(), 512, 100, 0, [2, 3, 4, 5]);
        let mut second = BeingSummary::new("Female".to_string(), 768, 200, 0, [3, 4, 5, 7]);
        first.date_of_birth = 0;
        second.date_of_birth = 0;
        first.drives[DRIVE_SEX] = THRESHOLD_SEEK_MATE + 1;
        second.drives[DRIVE_SEX] = THRESHOLD_SEEK_MATE + 1;
        first.random_seed = [1, 2];
        let first_index = first.meet_being(&second, AGE_OF_MATURITY + 1, 400);
        first.social_memory[first_index].attraction = PAIR_BOND_THRESHOLD + 1;
        second.record_episodic_food(ENERGY_FRUIT, FOOD_FRUIT, AGE_OF_MATURITY + 1, 390);

        social_mate_native(
            &mut first,
            &mut second,
            first_index,
            1,
            AGE_OF_MATURITY + 1,
            400,
        );
        assert_eq!(first.goal()[0], GOAL_MATE);
        assert!(first.social_memory()[first_index].attraction > PAIR_BOND_THRESHOLD);

        first.social_memory[first_index].friend_foe = 255;
        social_chat_native(&mut first, &second, first_index, AGE_OF_MATURITY + 1, 401);
        assert!(first.macro_state() & BEING_STATE_SPEAKING != 0);
        assert!(first
            .episodic_memory()
            .iter()
            .any(|entry| entry.event == EVENT_CHAT));
        assert!(first
            .episodic_memory()
            .iter()
            .any(|entry| entry.event == EVENT_EAT && entry.food == FOOD_FRUIT));
    }

    #[test]
    fn episodic_memory_records_fades_and_roundtrips() {
        let mut being = BeingSummary::new("Episode".to_string(), 512, 258, 0, [2, 3, 4, 5]);
        being.energy = BEING_FULL;
        being.record_episodic_food(ENERGY_GRASS, FOOD_VEGETABLE, 2, 30);
        let recorded = being.episodic_memory()[0];
        assert_eq!(recorded.event, EVENT_EAT);
        assert_eq!(recorded.food, FOOD_VEGETABLE);
        assert!(recorded.affect > EPISODIC_AFFECT_ZERO);

        being.cycle_episodic();
        assert!(being.episodic_memory()[0].affect < recorded.affect);

        let state = SimState {
            kind: KIND_OF_USE::KIND_LOAD_FILE,
            land: LandState::from_snapshot(LandSnapshot::new(2, [1, 2], 31)),
            random_seed: [0; 2],
            population: PopulationState::from_beings(vec![being.clone()], 1),
        };
        let saved = state.tranfer_startup_out_json();
        let loaded =
            SimState::load_startup_json(saved.written_data()).expect("episodic JSON should reload");
        assert_eq!(
            loaded.beings()[0].episodic_memory(),
            being.episodic_memory()
        );
    }

    #[test]
    fn native_save_and_load_include_expanded_brain_and_change_fields() {
        let state = SimState::load_startup_json(
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":0},\"beings\":[{\"name\":\"Expanded Ape\",\"delta\":{\"stored_energy\":3840},\"constant\":{\"date_of_birth\":0,\"genetics\":[2,3,4,5]},\"changes\":{\"drives\":[1,2,3,4],\"shout\":[1,2,3,4,5,6],\"inventory\":[10,11,12,13,14,15,16,17],\"learned_preference\":[1,1,1,1,1,1,1,1,2,3,4,5,6,7],\"date_of_conception\":99,\"fetal_genetics\":[9,8,7,6],\"father_name\":[5,6],\"mother_name\":[7,8],\"child_generation_range\":[3,4]},\"braindata\":{\"braincode_register\":[65,66,67],\"brain_state\":[1,2,3,4,5,6],\"script_overrides\":12,\"attention\":[0,1,2,3,4,5],\"brainprobe\":[{\"probe_type\":1,\"position\":2,\"address\":3,\"frequency\":4,\"offset\":5,\"state\":6}]},\"events\":{\"episodic\":[{\"space_time\":{\"date\":1,\"location\":[2,3],\"time\":4},\"first_name\":[512,0],\"family_name\":[258,0],\"event\":1,\"food\":0,\"affect\":16434,\"arg\":50}]}}]}",
        )
        .unwrap();
        let being = &state.beings()[0];
        assert_eq!(being.shout(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(being.inventory(), [10, 11, 12, 13, 14, 15, 16, 17]);
        assert_eq!(being.learned_preference()[PREFERENCE_CHAT], 6);
        assert_eq!(being.brain_state(), [1, 2, 3, 4, 5, 6]);
        assert_eq!(being.script_overrides(), 12);
        assert_eq!(being.brainprobe()[0].probe_type, 1);
        assert_eq!(being.episodic_memory()[0].event, EVENT_EAT);

        let saved = state.tranfer_startup_out_json();
        let saved_json =
            std::str::from_utf8(saved.written_data()).expect("expanded save should be utf8");
        assert!(saved_json.contains("\"episodic\""));
        assert!(saved_json.contains("\"brainprobe\""));
        assert!(saved_json.contains("\"inventory\""));
        assert!(saved_json.contains("\"learned_preference\""));
        let loaded =
            SimState::load_startup_json(saved.written_data()).expect("expanded save should reload");
        assert_eq!(loaded.beings()[0].brainprobe()[0].state, 6);
        assert_eq!(loaded.beings()[0].episodic_memory()[0].affect, 16_434);
    }

    #[test]
    fn reset_new_simulation_derives_seed_from_current_land_genetics() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        assert_eq!(state.kind(), KIND_OF_USE::KIND_NEW_SIMULATION);
        assert_eq!(state.land().date(), 0);
        assert_eq!(state.land().time(), 0);
        assert_eq!(state.land().genetics(), [23809, 53481]);
        assert_eq!(state.land().planet_genetics(), [46774, 42340]);
        assert_eq!(state.random_seed(), [27236, 50571]);
        assert_eq!(state.population(), INITIAL_POPULATION);
        assert_eq!(state.max_population(), LARGE_SIM as usize);
        assert_eq!(state.selected_name(), Some("Ape 001"));
        assert_eq!(state.beings()[0].gender_name() >> 8, SEX_MALE as n_byte2);
        assert_eq!(state.beings()[1].gender_name() >> 8, SEX_FEMALE as n_byte2);
    }

    #[test]
    fn population_selection_moves_and_finds_names() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        assert_eq!(state.selected_name(), Some("Ape 001"));
        state.select_next();
        assert_eq!(state.selected_name(), Some("Ape 002"));
        state.select_previous();
        assert_eq!(state.selected_name(), Some("Ape 001"));
        assert!(state.select_by_name("Ape 010"));
        assert_eq!(state.selected_name(), Some("Ape 010"));
        assert!(!state.select_by_name("Missing Ape"));
        assert_eq!(state.selected_name(), Some("Ape 010"));
    }

    #[test]
    fn step_empty_advances_save_visible_land_time() {
        let mut state = SimState::start_up(0x5261_f726);
        state.step_empty();
        assert_eq!(state.kind(), KIND_OF_USE::KIND_NOTHING_TO_RUN);
        assert_eq!(
            state.land_snapshot(),
            LandSnapshot::new(0, [7633, 53305], 1)
        );
    }

    #[test]
    fn step_empty_by_advances_save_visible_land_time_by_interval() {
        let mut state = SimState::start_up(0x5261_f726);
        state.step_empty_by(TIME_DAY_MINUTES as n_uint);
        assert_eq!(state.kind(), KIND_OF_USE::KIND_NOTHING_TO_RUN);
        assert_eq!(
            state.land_snapshot(),
            LandSnapshot::new(1, [7633, 53305], 0)
        );
    }

    #[test]
    fn startup_transfer_load_rejects_bad_signature_and_bad_genetics() {
        assert_eq!(
            startup_transfer_from_json_bytes(
                b"{\"information\":{\"signature\":0,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":0}}",
            )
            .unwrap_err(),
            "not a simulated ape json"
        );
        assert_eq!(
            startup_transfer_from_json_bytes(
                b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1],\"time\":0}}",
            )
            .unwrap_err(),
            "genetics array should have two values"
        );
    }

    #[test]
    fn startup_transfer_json_matches_empty_c_save_shape() {
        let startup = StartupTransfer::empty(LandSnapshot::new(0, [6443, 36036], 0));
        let file = tranfer_startup_out_json(&startup);
        assert_eq!(
            file.written_data(),
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":0,\"genetics\":[6443,36036],\"time\":0}}"
        );
        assert_eq!(file.location(), 177);
        assert_eq!(file.size(), 4096);
    }

    #[test]
    fn startup_transfer_omits_beings_for_empty_group() {
        let startup = StartupTransfer::empty(LandSnapshot::new(0, [1, 2], 3));
        let object = transfer_startup_obj(&startup);
        assert_eq!(object.len(), 2);
        assert_eq!(object[0].name, "information");
        assert_eq!(object[1].name, "land");
    }

    #[test]
    fn startup_transfer_serializes_supplied_being_objects() {
        let mut being = Vec::new();
        object_string(&mut being, "name", "Ape One");

        let startup = StartupTransfer {
            land: LandSnapshot::new(0, [1, 2], 3),
            beings: vec![being],
        };
        let file = tranfer_startup_out_json(&startup);
        assert_eq!(
            file.written_data(),
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":3},\"beings\":[{\"name\":\"Ape One\"}]}"
        );
    }

    #[test]
    fn sim_state_transfer_round_trips_initial_being_summaries() {
        let mut state = SimState::start_up(0x5261_f726);
        state.reset_new_simulation_from_land_seed();
        let json = state.tranfer_startup_out_json();
        let loaded = SimState::load_startup_json(json.written_data()).unwrap();
        assert_eq!(loaded.population(), INITIAL_POPULATION);
        assert_eq!(loaded.selected_name(), Some("Ape 001"));
        assert_eq!(loaded.beings()[0], state.beings()[0]);
        assert_eq!(loaded.land_snapshot(), state.land_snapshot());
    }
}
