#![allow(non_camel_case_types)]

//! Simulator constants and C-compatible public types for the ApeSDK Rust port.

use apesdk_toolkit::{
    array_add, array_number, math_random, math_random3, n_byte, n_byte2, n_byte4, n_spacetime,
    n_uint, object_array, object_number, object_object, object_parse_json, object_string,
    object_top_object, NFile, ObjectEntry, ObjectValue,
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
pub const NUMBER_OF_BODIES: usize = 10;
pub const INVENTORY_SIZE: usize = 8;
pub const PREFERENCES: usize = 14;
pub const ATTENTION_SIZE: usize = 6;
pub const SHOUT_BYTES: usize = 6;
pub const DRIVES: usize = 4;

pub const MAP_BITS: usize = 9;
pub const MAP_TILES: usize = 1;
pub const MAP_DIMENSION: usize = 1 << MAP_BITS;
pub const MAP_AREA: usize = 1 << (2 * MAP_BITS);
pub const APE_TO_MAP_BIT_RATIO: usize = 6;
pub const MAP_TO_TERRITORY_RATIO: usize = 5;
pub const MAP_APE_RESOLUTION_SIZE: n_byte2 = (MAP_DIMENSION << APE_TO_MAP_BIT_RATIO) as n_byte2;
pub const APESPACE_BOUNDS: n_byte2 = MAP_APE_RESOLUTION_SIZE - 1;
pub const TERRITORY_DIMENSION: usize = MAP_DIMENSION >> MAP_TO_TERRITORY_RATIO;
pub const TERRITORY_AREA: usize = TERRITORY_DIMENSION * TERRITORY_DIMENSION;

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
pub const FATIGUE_SPEED_THRESHOLD: n_byte = 8;
pub const THRESHOLD_SEEK_MATE: n_byte = 100;
pub const IMMUNE_FIT: n_byte = 5;
pub const MIN_ANTIBODIES: n_byte = 16;
pub const PATHOGEN_MUTATION_PROB: n_byte2 = 100;
pub const ANTIBODY_DEPLETION_PROB: n_byte2 = 100;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LandState {
    date: n_byte4,
    time: n_byte4,
    tile_genetics: [[n_byte2; 2]; MAP_TILES],
    genetics: [n_byte2; 2],
    tide_level: n_byte,
}

impl LandState {
    pub const fn new() -> Self {
        Self {
            date: 0,
            time: 0,
            tile_genetics: [[0; 2]; MAP_TILES],
            genetics: [0; 2],
            tide_level: 0,
        }
    }

    pub fn from_snapshot(snapshot: LandSnapshot) -> Self {
        let mut state = Self::new();
        state.date = snapshot.date;
        state.time = snapshot.time;
        state.tile_genetics[0] = snapshot.genetics;
        state.genetics = snapshot.genetics;
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

    pub const fn genetics(&self) -> [n_byte2; 2] {
        self.tile_genetics[0]
    }

    pub const fn planet_genetics(&self) -> [n_byte2; 2] {
        self.genetics
    }

    pub const fn snapshot(&self) -> LandSnapshot {
        LandSnapshot::new(self.date, self.genetics(), self.time)
    }

    pub fn clear(&mut self, kind: KIND_OF_USE, start: n_byte4) {
        let tile_genetics = self.tile_genetics;
        *self = Self::new();
        self.tile_genetics = tile_genetics;
        if kind != KIND_OF_USE::KIND_LOAD_FILE {
            self.time = (5 * TIME_HOUR_MINUTES) as n_byte4;
            self.date = start;
        }
    }

    pub fn cycle(&mut self) {
        self.time += 1;
        if self.time == TIME_DAY_MINUTES as n_byte4 {
            self.time = 0;
            self.date += 1;
        }
    }

    pub fn advance_minutes(&mut self, minutes: n_uint) {
        let day_minutes = TIME_DAY_MINUTES as n_uint;
        let total_minutes = n_uint::from(self.time) + minutes;
        self.date = self
            .date
            .wrapping_add((total_minutes / day_minutes) as n_byte4);
        self.time = (total_minutes % day_minutes) as n_byte4;
    }

    pub fn seed_genetics(&mut self, random: &mut [n_byte2; 2]) {
        for tile in &mut self.tile_genetics {
            tile[0] = random_byte2(random);
            tile[1] = random_byte2(random);
            math_random3(random);
        }
        self.genetics[0] = random_byte2(random);
        self.genetics[1] = random_byte2(random);
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
    speed: n_byte,
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
        Self {
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
            speed: 0,
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
            immune_antigens: [0; IMMUNE_ANTIGENS],
            immune_shape_antigen: [0; IMMUNE_ANTIGENS],
            immune_antibodies: [0; IMMUNE_POPULATION],
            immune_shape_antibody: [0; IMMUNE_POPULATION],
            immune_seed: [0; 2],
        }
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
        being.speed = (index % 16) as n_byte;
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
            speed: optional_number_byte(entries, "speed")?.unwrap_or(0),
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
        let speed = optional_number_byte(delta, "velocity")?.unwrap_or(0);
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
            speed,
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
        }
        if let Some(brain) = optional_object(entries, "braindata")? {
            if let Some(registers) =
                optional_array_byte(brain, "braincode_register", BRAINCODE_PSPACE_REGISTERS)?
            {
                being.braincode_register.copy_from_slice(&registers);
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

        Ok(being)
    }

    fn native_transfer_object(&self) -> Vec<ObjectEntry> {
        let mut object = Vec::new();
        object_string(&mut object, "name", &self.name);
        object_object(&mut object, "delta", self.native_delta_object());
        object_object(&mut object, "constant", self.native_constant_object());
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
        object_number(&mut object, "speed", self.speed.into());
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
        object_number(&mut delta, "velocity", self.speed.into());
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
        changes
    }

    fn native_brain_object(&self) -> Vec<ObjectEntry> {
        let mut brain = Vec::new();
        object_array_byte(&mut brain, "braincode_register", &self.braincode_register);
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

    pub fn to_simulated_being(&self) -> simulated_being {
        let mut being = simulated_being::default();
        being.constant.date_of_birth = self.date_of_birth;
        being.constant.generation_min = self.generation_min;
        being.constant.generation_max = self.generation_max;
        being.constant.name = [self.gender_name, self.family_name];
        being.constant.genetics = self.genetics;
        being.delta.location = self.location;
        being.delta.direction_facing = self.facing;
        being.delta.velocity[0] = self.speed;
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
        being.changes.drives = self.drives;
        being.braindata.braincode_register = self.braincode_register;
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
            speed: native.delta.velocity[0],
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
        self.speed
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

    fn advance_minute(&mut self, land_date: n_byte4, land_time: n_byte4) {
        self.awake_level = self.awake_level_for_time(land_time);
        self.awake = self.awake_level != FULLY_ASLEEP;
        self.cycle_universal();

        if self.energy == BEING_DEAD || self.awake_level == FULLY_ASLEEP {
            self.speed = 0;
            return;
        }

        math_random3(&mut self.random_seed);
        let turn = ((math_random(&mut self.random_seed) & 7) as i16) - 3;
        self.facing = self.facing.wrapping_add(turn as n_byte);

        let pace = 1 + (math_random(&mut self.random_seed) & 7) as n_byte;
        self.speed = pace.min(39);
        let [dx, dy] = facing_delta(self.facing);
        self.location[0] = wrap_apespace(self.location[0] as i32 + dx * i32::from(self.speed));
        self.location[1] = wrap_apespace(self.location[1] as i32 + dy * i32::from(self.speed));

        let energy_cost = 1 + n_byte2::from(self.speed >> 2);
        self.energy_delta(-i32::from(energy_cost));
        self.macro_state = BEING_STATE_AWAKE;
        if self.energy_less_than(BEING_HUNGRY + 1) {
            self.macro_state |= BEING_STATE_HUNGRY;
        }
        if self.speed > 0 {
            self.macro_state |= BEING_STATE_MOVING;
        }

        self.cycle_drives(land_date);

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
        if self.energy_less_than(BEING_HUNGRY + 1) || self.speed > 0 {
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
            self.speed = 0;
        }
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

        if self.speed > FATIGUE_SPEED_THRESHOLD {
            self.inc_drive(DRIVE_FATIGUE);
            if self.macro_state & BEING_STATE_SWIMMING != 0 {
                self.inc_drive(DRIVE_FATIGUE);
            }
            self.dec_drive(DRIVE_SEX);
        } else {
            self.dec_drive(DRIVE_FATIGUE);
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

fn wrap_apespace(value: i32) -> n_byte2 {
    let bounds = i32::from(APESPACE_BOUNDS);
    value.rem_euclid(bounds + 1) as n_byte2
}

fn facing_delta(facing: n_byte) -> [i32; 2] {
    const DELTAS: [[i32; 2]; 16] = [
        [1, 0],
        [1, 1],
        [1, 1],
        [0, 1],
        [0, 1],
        [-1, 1],
        [-1, 1],
        [-1, 0],
        [-1, 0],
        [-1, -1],
        [-1, -1],
        [0, -1],
        [0, -1],
        [1, -1],
        [1, -1],
        [1, 0],
    ];
    DELTAS[((facing.wrapping_add(7) >> 4) & 15) as usize]
}

pub fn is_night(time: n_byte4) -> bool {
    let hourish = time >> 5;
    !(11..=36).contains(&hourish)
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

fn pathogen_severity(pathogen: n_byte) -> n_byte {
    ((u16::from(pathogen) * u16::from(pathogen)) >> 11) as n_byte
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

    fn advance_minute(&mut self, land_date: n_byte4, land_time: n_byte4) {
        for being in &mut self.beings {
            being.advance_minute(land_date, land_time);
        }
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
            self.population
                .advance_minute(self.land.date(), self.land.time());
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

    pub const fn land_snapshot(&self) -> LandSnapshot {
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
        being.speed = 1;
        assert_eq!(being.awake_level_for_time(0), SLIGHTLY_AWAKE);
        being.speed = 0;
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
        being.speed = FATIGUE_SPEED_THRESHOLD + 1;
        being.awake_level = FULLY_AWAKE;
        being.drives = [0; DRIVES];
        being.cycle_drives(AGE_OF_MATURITY + 1);
        assert_eq!(being.drive(DRIVE_HUNGER), 1);
        assert_eq!(being.drive(DRIVE_SOCIAL), 1);
        assert_eq!(being.drive(DRIVE_FATIGUE), 1);
        assert_eq!(being.drive(DRIVE_SEX), 0);
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
