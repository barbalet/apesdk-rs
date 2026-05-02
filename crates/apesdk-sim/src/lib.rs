#![allow(non_camel_case_types)]

//! Simulator constants and C-compatible public types for the ApeSDK Rust port.

use apesdk_toolkit::{
    array_add, array_number, math_random, math_random3, n_byte, n_byte2, n_byte4, n_uint,
    object_array, object_number, object_object, object_parse_json, object_string,
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

pub const MAP_BITS: usize = 9;
pub const MAP_TILES: usize = 1;
pub const MAP_DIMENSION: usize = 1 << MAP_BITS;
pub const MAP_AREA: usize = 1 << (2 * MAP_BITS);
pub const APE_TO_MAP_BIT_RATIO: usize = 6;
pub const MAP_TO_TERRITORY_RATIO: usize = 5;
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

pub const LARGE_SIM: n_uint = 256;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct n_version {
    pub signature: n_byte2,
    pub version: n_byte2,
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
pub struct SimState {
    kind: KIND_OF_USE,
    land: LandState,
    random_seed: [n_byte2; 2],
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
        }
    }

    pub fn start_up(randomise: n_uint) -> Self {
        Self::init(KIND_OF_USE::KIND_START_UP, randomise)
    }

    pub fn from_startup_transfer(startup: &StartupTransfer) -> Self {
        Self {
            kind: KIND_OF_USE::KIND_LOAD_FILE,
            land: LandState::from_snapshot(startup.land),
            random_seed: [0; 2],
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
    }

    pub fn step_empty(&mut self) {
        self.land.cycle();
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

    pub const fn land_snapshot(&self) -> LandSnapshot {
        self.land.snapshot()
    }

    pub fn startup_transfer(&self) -> StartupTransfer {
        StartupTransfer::empty(self.land_snapshot())
    }

    pub fn tranfer_startup_out_json(&self) -> NFile {
        tranfer_startup_out_json(&self.startup_transfer())
    }

    pub fn prepare_land_for_first_cycle(&mut self) {
        self.land.clear(self.kind, AGE_OF_MATURITY);
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

fn field_byte4(entries: &[ObjectEntry], name: &str) -> Result<n_byte4, &'static str> {
    let number = field_number(entries, name)?;
    if (0..=n_byte4::MAX.into()).contains(&number) {
        Ok(number as n_byte4)
    } else {
        Err("json number outside n_byte4 range")
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

fn array_byte2(value: &ObjectValue) -> Result<n_byte2, &'static str> {
    match value {
        ObjectValue::Number(number) if (0..=n_byte2::MAX.into()).contains(number) => {
            Ok(*number as n_byte2)
        }
        ObjectValue::Number(_) => Err("json number outside n_byte2 range"),
        _ => Err("json number expected"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

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
}
