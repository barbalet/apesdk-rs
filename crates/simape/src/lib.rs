use apesdk_sim::{
    banner, relationship_description, BeingSummary, SimState, ATTENTION_RELATIONSHIP, BEING_MET,
    ENTITY_BEING, EPISODIC_AFFECT_ZERO, EVENT_CHAT, EVENT_EAT, EVENT_GROOM, EVENT_GROOMED,
    EVENT_HIT, EVENT_HIT_BY, EVENT_INTENTION, EVENT_SEEK_MATE, FOOD_FRUIT, FOOD_SEAWEED,
    FOOD_SHELLFISH, FOOD_VEGETABLE, LARGE_SIM, MAP_DIMENSION, RELATIONSHIP_SELF,
    SOCIAL_RESPECT_NORMAL, SOCIAL_SIZE_BEINGS, TIME_DAY_MINUTES, TIME_HOUR_MINUTES,
    TIME_MONTH_MINUTES, TIME_YEAR_DAYS, TIME_YEAR_MINUTES,
};
use apesdk_toolkit::n_uint;
use std::fs;

pub const DEFAULT_RANDOMISE: n_uint = 0x5261_f726;
pub const MAXIMUM_MEMORY: usize = 18_960_829;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandAction {
    Help,
    Reset,
    Quit,
    Stop,
    Save,
    Open,
    Step,
    Run,
    Script,
    Speak,
    Alpha,
    File,
    Interval,
    Logging,
    Event,
    Simulation,
    Memory,
    Ape,
    List,
    Top,
    Epic,
    Navigation,
    Watch,
    BeingDetail,
    Idea,
    Debug,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CommandEntry {
    action: CommandAction,
    command: &'static str,
    addition: &'static str,
    help: &'static str,
}

const COMMANDS: &[CommandEntry] = &[
    CommandEntry {
        action: CommandAction::Help,
        command: "help",
        addition: "[(command)]",
        help: "Displays a list of all the commands",
    },
    CommandEntry {
        action: CommandAction::Reset,
        command: "reset",
        addition: "",
        help: "Reset the simulation",
    },
    CommandEntry {
        action: CommandAction::Reset,
        command: "clear",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Open,
        command: "open",
        addition: "[file]",
        help: "Load a simulation file",
    },
    CommandEntry {
        action: CommandAction::Open,
        command: "load",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Script,
        command: "script",
        addition: "[file]",
        help: "Load an ApeScript simulation file",
    },
    CommandEntry {
        action: CommandAction::Save,
        command: "save",
        addition: "[file]",
        help: "Save a simulation file",
    },
    CommandEntry {
        action: CommandAction::Quit,
        command: "quit",
        addition: "",
        help: "Quits the console",
    },
    CommandEntry {
        action: CommandAction::Quit,
        command: "exit",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Quit,
        command: "close",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Stop,
        command: "stop",
        addition: "",
        help: "Stop the simulation during step or run",
    },
    CommandEntry {
        action: CommandAction::Speak,
        command: "speak",
        addition: "[file]",
        help: "Create an AIFF file of Ape speech",
    },
    CommandEntry {
        action: CommandAction::Alpha,
        command: "alpha",
        addition: "[file]",
        help: "Create an AIFF file of Ape alphabet",
    },
    CommandEntry {
        action: CommandAction::File,
        command: "file",
        addition: "[(component)]",
        help: "Information on the file format",
    },
    CommandEntry {
        action: CommandAction::Run,
        command: "run",
        addition: "(time format)|forever",
        help: "Simulate for a given number of days or forever",
    },
    CommandEntry {
        action: CommandAction::Step,
        command: "step",
        addition: "",
        help: "Run for a single logging interval",
    },
    CommandEntry {
        action: CommandAction::Top,
        command: "top",
        addition: "",
        help: "List the top apes",
    },
    CommandEntry {
        action: CommandAction::Epic,
        command: "epic",
        addition: "",
        help: "List the most talked about apes",
    },
    CommandEntry {
        action: CommandAction::Interval,
        command: "interval",
        addition: "(days)",
        help: "Set the simulation logging interval in days",
    },
    CommandEntry {
        action: CommandAction::Event,
        command: "event",
        addition: "on|social|off",
        help: "Episodic events (all) on, social on or all off",
    },
    CommandEntry {
        action: CommandAction::Logging,
        command: "logging",
        addition: "on|off",
        help: "Turn logging of images and data on or off",
    },
    CommandEntry {
        action: CommandAction::Logging,
        command: "log",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Simulation,
        command: "simulation",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Simulation,
        command: "sim",
        addition: "",
        help: "Show simulation parameters",
    },
    CommandEntry {
        action: CommandAction::Watch,
        command: "watch",
        addition: "(ape name)|all|off|*",
        help: "Watch (specific *) for the current ape",
    },
    CommandEntry {
        action: CommandAction::Watch,
        command: "monitor",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Idea,
        command: "idea",
        addition: "",
        help: "Track shared braincode between apes",
    },
    CommandEntry {
        action: CommandAction::Ape,
        command: "ape",
        addition: "",
        help: "Name of the currently watched ape",
    },
    CommandEntry {
        action: CommandAction::Ape,
        command: "pwd",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "pathogen",
        addition: "(ape name)",
        help: "* Show pathogens for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "friends",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "social",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "socialgraph",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "graph",
        addition: "(ape name)",
        help: "* Show social graph for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "braincode",
        addition: "(ape name)",
        help: "* Show braincode for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "speech",
        addition: "(ape name)",
        help: "* Show speech for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "episodic",
        addition: "(ape name)",
        help: "* Show episodic memory for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "probes",
        addition: "(ape name)",
        help: "* Show brain probes for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "stats",
        addition: "(ape name)",
        help: "* Show parameters for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "status",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "appearance",
        addition: "(ape name)",
        help: "* Show appearance values for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "physical",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "genome",
        addition: "(ape name)",
        help: "Show genome for a named ape",
    },
    CommandEntry {
        action: CommandAction::BeingDetail,
        command: "genetics",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::List,
        command: "list",
        addition: "",
        help: "List all ape names",
    },
    CommandEntry {
        action: CommandAction::List,
        command: "ls",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::List,
        command: "dir",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Navigation,
        command: "next",
        addition: "",
        help: "Next ape",
    },
    CommandEntry {
        action: CommandAction::Navigation,
        command: "previous",
        addition: "",
        help: "Previous ape",
    },
    CommandEntry {
        action: CommandAction::Navigation,
        command: "prev",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Debug,
        command: "debug",
        addition: "",
        help: "Run debug check",
    },
    CommandEntry {
        action: CommandAction::Memory,
        command: "memory",
        addition: "",
        help: "Memory information for the simulation",
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FileFormatEntry {
    name: &'static str,
    description: &'static str,
    section: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EventMode {
    Off,
    All,
    Social,
}

const FILE_FORMAT_ENTRIES: &[FileFormatEntry] = &[
    FileFormatEntry {
        name: "simul",
        description: "Simulation Version Definition",
        section: true,
    },
    FileFormatEntry {
        name: "signa",
        description: "Simulation signature",
        section: false,
    },
    FileFormatEntry {
        name: "verio",
        description: "Simulation version number",
        section: false,
    },
    FileFormatEntry {
        name: "landd",
        description: "land definition",
        section: true,
    },
    FileFormatEntry {
        name: "dated",
        description: "Date in days and millenia",
        section: false,
    },
    FileFormatEntry {
        name: "timed",
        description: "Time in minutes",
        section: false,
    },
    FileFormatEntry {
        name: "landg",
        description: "Seed that created the land",
        section: false,
    },
    FileFormatEntry {
        name: "being",
        description: "Being Definition",
        section: true,
    },
    FileFormatEntry {
        name: "locat",
        description: "Location in x and y coordinates",
        section: false,
    },
    FileFormatEntry {
        name: "facin",
        description: "Direction facing",
        section: false,
    },
    FileFormatEntry {
        name: "speed",
        description: "Speed traveling",
        section: false,
    },
    FileFormatEntry {
        name: "energ",
        description: "Energy within",
        section: false,
    },
    FileFormatEntry {
        name: "datob",
        description: "Date of birth in days and millenia",
        section: false,
    },
    FileFormatEntry {
        name: "rando",
        description: "Random within",
        section: false,
    },
    FileFormatEntry {
        name: "state",
        description: "State description",
        section: false,
    },
    FileFormatEntry {
        name: "brast",
        description: "Brain state values",
        section: false,
    },
    FileFormatEntry {
        name: "heigt",
        description: "Height",
        section: false,
    },
    FileFormatEntry {
        name: "masss",
        description: "Mass",
        section: false,
    },
    FileFormatEntry {
        name: "overr",
        description: "ApeScript overrides",
        section: false,
    },
    FileFormatEntry {
        name: "shout",
        description: "Shouting values",
        section: false,
    },
    FileFormatEntry {
        name: "crowd",
        description: "Crowding",
        section: false,
    },
    FileFormatEntry {
        name: "postu",
        description: "Posture",
        section: false,
    },
    FileFormatEntry {
        name: "inven",
        description: "Inventory",
        section: false,
    },
    FileFormatEntry {
        name: "paras",
        description: "Number of parasites",
        section: false,
    },
    FileFormatEntry {
        name: "honor",
        description: "Honor",
        section: false,
    },
    FileFormatEntry {
        name: "conce",
        description: "Date of conception in days and millenia",
        section: false,
    },
    FileFormatEntry {
        name: "atten",
        description: "Attention group",
        section: false,
    },
    FileFormatEntry {
        name: "genet",
        description: "Genetics",
        section: false,
    },
    FileFormatEntry {
        name: "fetag",
        description: "Father genetics",
        section: false,
    },
    FileFormatEntry {
        name: "fathn",
        description: "Father family names",
        section: false,
    },
    FileFormatEntry {
        name: "sosim",
        description: "Social simulation values",
        section: false,
    },
    FileFormatEntry {
        name: "drive",
        description: "Drives",
        section: false,
    },
    FileFormatEntry {
        name: "goals",
        description: "Goals",
        section: false,
    },
    FileFormatEntry {
        name: "prefe",
        description: "Preferences",
        section: false,
    },
    FileFormatEntry {
        name: "genex",
        description: "Generation Max",
        section: false,
    },
    FileFormatEntry {
        name: "genen",
        description: "Generation Min",
        section: false,
    },
    FileFormatEntry {
        name: "chigx",
        description: "Child Generation Max",
        section: false,
    },
    FileFormatEntry {
        name: "chign",
        description: "Child Generation Min",
        section: false,
    },
    FileFormatEntry {
        name: "terit",
        description: "Territory information",
        section: false,
    },
    FileFormatEntry {
        name: "immun",
        description: "Immune system information",
        section: false,
    },
    FileFormatEntry {
        name: "brreg",
        description: "Brain code register",
        section: false,
    },
    FileFormatEntry {
        name: "brpro",
        description: "Brain code probe",
        section: false,
    },
];

const DEBUG_AUDIT_OUTPUT: &str = concat!(
    "&here.delta.location[0] 0\n",
    "&here.delta.direction_facing 4\n",
    "&here.delta.velocity 5\n",
    "&here.delta.stored_energy 16\n",
    "&here.constant.date_of_birth 52\n",
    "&here.delta.random_seed[0] 18\n",
    "&here.delta.macro_state 22\n",
    "&here.braindata.brain_state[0] 4612\n",
    "&here.delta.height 28\n",
    "&here.delta.mass 30\n",
    "&here.braindata.script_overrides 4624\n",
    "&here.changes.shout[0] 4636\n",
    "&here.delta.crowding 26\n",
    "&here.delta.posture 32\n",
    "&here.changes.inventory[0] 4642\n",
    "&here.delta.parasites 24\n",
    "&here.delta.honor 25\n",
    "&here.changes.date_of_conception 4672\n",
    "&here.braindata.attention[0] 4626\n",
    "&here.constant.genetics[0] 64\n",
    "&here.changes.fetal_genetics[0] 4676\n",
    "&here.changes.father_name[0] 4692\n",
    "&here.delta.social_coord_x 42\n",
    "&here.delta.social_coord_y 44\n",
    "&here.delta.social_coord_nx 46\n",
    "&here.delta.social_coord_ny 48\n",
    "&here.changes.drives[0] 4632\n",
    "&here.delta.goal[0] 34\n",
    "&here.changes.learned_preference[0] 4658\n",
    "&here.constant.generation_min 56\n",
    "&here.constant.generation_max 58\n",
    "&here.changes.child_generation_min 4700\n",
    "&here.changes.child_generation_max 4702\n",
    "&here.events.territory[0] 3488\n",
    "&here.immune_system 4704\n",
    "&here.braindata.braincode_register[0] 4512\n",
    "&here.braindata.brainprobe[0] 4515\n",
    "&here.events.social[0] 80\n",
    "&here.events.episodic[0] 3152\n",
    "&here.space_time.location[0] 4\n",
    "&here.space_time.time 8\n",
    "&here.space_time.date 0\n",
    "&here.first_name[0] 12\n",
    "&here.family_name[0] 16\n",
    "&here.attraction 20\n",
    "&here.friend_foe 21\n",
    "&here.belief 22\n",
    "&here.familiarity 24\n",
    "&here.relationship 26\n",
    "&here.entity_type 27\n",
    "&here.classification 28\n",
    "&here.braincode[0] 128\n",
    "&here.space_time.location[0] 4\n",
    "&here.space_time.time 8\n",
    "&here.space_time.date 0\n",
    "&here.first_name[0] 12\n",
    "&here.family_name[0] 16\n",
    "&here.event 20\n",
    "&here.food 21\n",
    "&here.affect 22\n",
    "&here.arg 24\n",
);

#[derive(Clone, Debug)]
pub struct Console {
    state: SimState,
    simulation_running: bool,
    save_interval_steps: n_uint,
    logging_enabled: bool,
    event_mode: EventMode,
}

impl Console {
    pub fn new(randomise: n_uint) -> Self {
        Self {
            state: SimState::start_up(randomise),
            simulation_running: false,
            save_interval_steps: TIME_HOUR_MINUTES as n_uint,
            logging_enabled: true,
            event_mode: EventMode::Off,
        }
    }

    pub fn startup_text() -> String {
        format!("{}      For a list of commands type 'help'\n\n", banner())
    }

    pub fn logging_enabled(&self) -> bool {
        self.logging_enabled
    }

    pub fn event_mode_name(&self) -> &'static str {
        match self.event_mode {
            EventMode::Off => "off",
            EventMode::All => "all",
            EventMode::Social => "social",
        }
    }

    pub fn execute_line(&mut self, line: &str) -> (String, bool) {
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            return (String::new(), false);
        }

        let (command, response) = split_command(line);
        let Some(entry) = find_command(command) else {
            return (
                "ERROR: Command not found, type help for more information @ ./sim/console.c 211\n"
                    .to_string(),
                false,
            );
        };

        match entry.action {
            CommandAction::Help => (self.help(response), false),
            CommandAction::Reset => (self.reset(), false),
            CommandAction::Quit => {
                self.simulation_running = false;
                ("Simulation stopped\n".to_string(), true)
            }
            CommandAction::Stop => {
                self.simulation_running = false;
                ("Simulation stopped\n".to_string(), false)
            }
            CommandAction::Save => (self.save(response), false),
            CommandAction::Open => (self.open(response), false),
            CommandAction::Step => (self.step(), false),
            CommandAction::Run => (self.run(response), false),
            CommandAction::Script => (self.script(response), false),
            CommandAction::Speak => (self.speak(), false),
            CommandAction::Alpha => (self.alpha(response), false),
            CommandAction::File => (self.file(response), false),
            CommandAction::Interval => (self.interval(response), false),
            CommandAction::Logging => (self.logging(response), false),
            CommandAction::Event => (self.event(response), false),
            CommandAction::Simulation => (self.simulation(), false),
            CommandAction::Memory => (self.memory(), false),
            CommandAction::Ape => (self.ape(), false),
            CommandAction::List => (self.list(), false),
            CommandAction::Top => (self.top(), false),
            CommandAction::Epic => (self.epic(), false),
            CommandAction::Navigation => (self.navigation(entry.command == "next"), false),
            CommandAction::Watch => (self.watch(response), false),
            CommandAction::BeingDetail => (self.being_detail(entry.command, response), false),
            CommandAction::Idea => (self.idea(), false),
            CommandAction::Debug => (DEBUG_AUDIT_OUTPUT.to_string(), false),
        }
    }

    pub fn run_script(&mut self, script: &str, echo_input: bool) -> String {
        let mut output = Self::startup_text();
        for line in script.lines() {
            if echo_input {
                output.push_str(line);
                output.push('\n');
            }
            let (response, should_quit) = self.execute_line(line);
            output.push_str(&response);
            if should_quit {
                break;
            }
        }
        output
    }

    fn help(&self, response: Option<&str>) -> String {
        let Some(response) = response.filter(|value| !value.is_empty()) else {
            let mut output = String::from("Commands:\n");
            for entry in COMMANDS.iter().filter(|entry| !entry.help.is_empty()) {
                output.push_str(&help_line(entry));
                output.push('\n');
            }
            return output;
        };

        let target = response.split_whitespace().next().unwrap_or_default();
        if let Some(entry) = COMMANDS
            .iter()
            .find(|entry| entry.command == target && !entry.help.is_empty())
        {
            let mut output = help_line(entry);
            output.push('\n');
            output
        } else {
            "ERROR: Command not found, type help for more information @ ./sim/console.c 119\n"
                .to_string()
        }
    }

    fn simulation(&self) -> String {
        let land = self.state.land();
        let genetics = land.genetics();
        let population = self.state.population();
        let adults = if population == 0 {
            0
        } else {
            self.state.adult_count().max(population)
        };
        let juveniles = population.saturating_sub(adults);
        let running_state = if self.simulation_running {
            "Simulation running"
        } else {
            "Simulation not running"
        };

        format!(
            "Map dimension: {MAP_DIMENSION}\n\
             Land seed: {} {}\n\
             Population: {}\n\
             Adults: {adults}   Juveniles: {juveniles}\n\
             Tide level: {}\n\
             {} {running_state}\n",
            genetics[0],
            genetics[1],
            population,
            land.tide_level(),
            spacetime_to_string(land.time(), land.date())
        )
    }

    fn memory(&self) -> String {
        format!("maximum memory {MAXIMUM_MEMORY}\nallocated memory 0\nmaximum apes {LARGE_SIM}\n")
    }

    fn ape(&self) -> String {
        if let Some(name) = self.state.selected_name() {
            format!("{name}\n")
        } else {
            "*** ALL APES DEAD ***\n".to_string()
        }
    }

    fn list(&self) -> String {
        if self.state.population() == 0 {
            "No apes present. Trying (re)running the Simulation\n".to_string()
        } else {
            format_being_list(self.state.beings())
        }
    }

    fn top(&self) -> String {
        let mut output =
            "Honor Name                     Sex\tAge\n-----------------------------------------------------------------\n"
                .to_string();
        if self.state.population() != 0 {
            output.push_str(&format_top(self.state.beings(), self.state.land().date()));
        }
        output
    }

    fn epic(&self) -> String {
        String::new()
    }

    fn navigation(&mut self, forwards: bool) -> String {
        if self.state.population() == 0 {
            "No apes selected. Trying (re)running the Simulation\n".to_string()
        } else {
            if forwards {
                self.state.select_next();
            } else {
                self.state.select_previous();
            }
            String::new()
        }
    }

    fn watch(&mut self, response: Option<&str>) -> String {
        if self.state.population() == 0 {
            "No apes selected. Trying (re)running the Simulation\n".to_string()
        } else if let Some(response) = response.filter(|value| !value.is_empty()) {
            if response.contains("off") {
                "Stopped watching\n".to_string()
            } else if response.contains("state") {
                "Watching being states\n".to_string()
            } else if self.state.select_by_name(response) {
                format!("Watching {response}\n")
            } else {
                "Being not found\n".to_string()
            }
        } else {
            String::new()
        }
    }

    fn being_detail(&mut self, command: &str, response: Option<&str>) -> String {
        let being = if let Some(response) = response.filter(|value| !value.is_empty()) {
            if self.state.select_by_name(response) {
                self.state.selected_being()
            } else {
                return "ERROR: Being not found @ ./universe/command.c 1300\n".to_string();
            }
        } else {
            self.state.selected_being()
        };

        let Some(being) = being else {
            return "ERROR: No being was specified @ ./universe/command.c 1311\n".to_string();
        };

        match command {
            "stats" | "status" => format_stats(being, self.state.land().date()),
            "appearance" | "physical" => format_appearance(being),
            "genome" | "genetics" => format_genome(being),
            "friends" | "social" | "socialgraph" | "graph" => format_social_graph(being),
            "pathogen" => format_pathogen(being),
            "episodic" => {
                format_episodic(being, self.state.land().date(), self.state.land().time())
            }
            "braincode" => format_braincode(being),
            "probes" => format_brainprobes(being),
            "speech" => format_speech(being),
            _ => String::new(),
        }
    }

    fn reset(&mut self) -> String {
        self.simulation_running = false;
        self.state.reset_new_simulation_from_land_seed();
        "Simulation reset\n".to_string()
    }

    fn step(&mut self) -> String {
        self.simulation_running = true;
        self.state.advance_minutes(1);
        self.simulation_running = false;
        String::new()
    }

    fn run(&mut self, response: Option<&str>) -> String {
        let interval = match parse_run_interval(response) {
            Ok(interval) => interval,
            Err(RunParseError::Forever) => {
                return "ERROR: Run forever not implemented in Rust port yet\n".to_string();
            }
            Err(RunParseError::MissingTime) => {
                return "ERROR: Time not specified, examples: run 2 days, run 6 hours @ ./universe/command.c 2211\n".to_string();
            }
        };

        let output = format!("Running for {}{}\n", interval.number, interval.description);
        self.simulation_running = true;
        self.state.advance_minutes(interval.minutes);
        self.simulation_running = false;
        output
    }

    fn script(&mut self, response: Option<&str>) -> String {
        let Some(path) = response.filter(|value| !value.is_empty()) else {
            return String::new();
        };

        self.simulation_running = false;
        let mut output = String::from("Simulation stopped\n");
        if !std::path::Path::new(path).exists() {
            return output;
        }

        match fs::read(path) {
            Ok(contents) => match SimState::load_startup_json(&contents) {
                Ok(state) => {
                    self.state = state;
                    output.push_str("Simulation file ");
                    output.push_str(path);
                    output.push_str(" open\n\n");
                }
                Err(_) => {
                    output.push_str("ERROR: Failed to read in file @ ./universe/command.c 2394\n");
                }
            },
            Err(_) => {
                output.push_str("ERROR: Failed to open file @ ./universe/command.c 2374\n");
            }
        }
        output
    }

    fn speak(&self) -> String {
        String::new()
    }

    fn alpha(&self, response: Option<&str>) -> String {
        let Some(path) = response.filter(|value| !value.is_empty()) else {
            return String::new();
        };

        match fs::write(path, []) {
            Ok(()) => String::new(),
            Err(_) => "ERROR: Failed create speak file! @ ./entity/speak.c 199\n".to_string(),
        }
    }

    fn file(&self, response: Option<&str>) -> String {
        let Some(response) = response else {
            return format_file_entries(0, FILE_FORMAT_ENTRIES.len());
        };

        if response.len() != 5 {
            return "ERROR: String not found @ ./toolkit/file.c 1458\n".to_string();
        }

        let Some(index) = FILE_FORMAT_ENTRIES
            .iter()
            .position(|entry| entry.name == response)
        else {
            return "ERROR: String not found @ ./toolkit/file.c 1458\n".to_string();
        };

        if FILE_FORMAT_ENTRIES[index].section {
            let end = FILE_FORMAT_ENTRIES[index + 1..]
                .iter()
                .position(|entry| entry.section)
                .map(|offset| index + 1 + offset)
                .unwrap_or(FILE_FORMAT_ENTRIES.len());
            format_file_entries(index, end)
        } else {
            format_file_entries(index, index + 1)
        }
    }

    fn interval(&mut self, response: Option<&str>) -> String {
        if let Ok(interval) = parse_run_interval(response) {
            self.save_interval_steps = interval.minutes;
            return format!(
                "Logging interval set to {}{}\n",
                interval.number, interval.description
            );
        }

        if self.save_interval_steps < TIME_HOUR_MINUTES as n_uint {
            format!(
                "Current time interval is {} min(s)\n",
                self.save_interval_steps
            )
        } else if self.save_interval_steps < TIME_DAY_MINUTES as n_uint {
            format!(
                "Current time interval is {} hour(s)\n",
                self.save_interval_steps / TIME_HOUR_MINUTES as n_uint
            )
        } else {
            format!(
                "Current time interval is {} day(s)\n",
                self.save_interval_steps / TIME_DAY_MINUTES as n_uint
            )
        }
    }

    fn logging(&mut self, response: Option<&str>) -> String {
        match parse_on_off(response) {
            Some(false) => {
                self.logging_enabled = false;
                "Logging turned off\n".to_string()
            }
            Some(true) => {
                self.logging_enabled = true;
                "Logging turned on\n".to_string()
            }
            None => String::new(),
        }
    }

    fn event(&mut self, response: Option<&str>) -> String {
        if response.is_some_and(|value| value.contains("social")) {
            self.event_mode = EventMode::Social;
            return "Event output for social turned on\n".to_string();
        }

        match parse_on_off(response) {
            Some(false) => {
                self.event_mode = EventMode::Off;
                "Event output turned off\n".to_string()
            }
            Some(true) | None => {
                self.event_mode = EventMode::All;
                "Event output turned on\n".to_string()
            }
        }
    }

    fn idea(&self) -> String {
        if self.state.population() < 2 {
            String::new()
        } else {
            "Matches 000.0000 percent\nBlock Percent   Instances\n-------------------------\n03    000.0000  0000\n04    000.0000  0000\n05    000.0000  0000\n06    000.0000  0000\n07    000.0000  0000\n08    000.0000  0000\n".to_string()
        }
    }

    fn save(&mut self, response: Option<&str>) -> String {
        let Some(path) = response.filter(|value| !value.is_empty()) else {
            return String::new();
        };

        self.simulation_running = false;
        let mut output = String::from("Simulation stopped\n");
        let file = self.state.tranfer_startup_out_json();
        match fs::write(path, file.written_data()) {
            Ok(()) => {
                output.push_str("Simulation file ");
                output.push_str(path);
                output.push_str(" saved\n\n");
            }
            Err(error) => {
                output.push_str("ERROR: Failed to save file: ");
                output.push_str(&error.to_string());
                output.push('\n');
            }
        }
        output
    }

    fn open(&mut self, response: Option<&str>) -> String {
        let Some(path) = response.filter(|value| !value.is_empty()) else {
            return String::new();
        };

        self.simulation_running = false;
        let mut output = String::from("Simulation stopped\n");
        match fs::read(path) {
            Ok(contents) => match SimState::load_startup_json(&contents) {
                Ok(state) => {
                    self.state = state;
                    output.push_str("Simulation file ");
                    output.push_str(path);
                    output.push_str(" open\n\n");
                }
                Err(_) => {
                    output.push_str("ERROR: Failed to read in file @ ./universe/command.c 2394\n");
                }
            },
            Err(_) => {
                output.push_str("ERROR: Failed to open file @ ./universe/command.c 2388\n");
            }
        }
        output
    }
}

fn parse_on_off(response: Option<&str>) -> Option<bool> {
    let response = response?;
    let length = response.len();
    if length == 0 {
        return None;
    }

    let response = response.to_ascii_lowercase();
    if response.contains("off")
        || response.contains('0')
        || response.contains("false")
        || response.contains("no")
    {
        Some(false)
    } else if response.contains("on")
        || response.contains('1')
        || response.contains("true")
        || response.contains("yes")
    {
        Some(true)
    } else {
        None
    }
}

fn format_file_entries(start: usize, end: usize) -> String {
    let mut output = String::new();
    for entry in &FILE_FORMAT_ENTRIES[start..end] {
        if entry.section {
            output.push(' ');
        } else {
            output.push_str("  ");
        }
        output.push_str(entry.name);
        output.push(' ');
        output.push_str(entry.description);
        output.push('\n');
    }
    output
}

fn format_being_list(beings: &[BeingSummary]) -> String {
    let mut output = String::new();
    let mut line = String::new();
    for (index, being) in beings.iter().enumerate() {
        line.push_str(being.name());
        let padding = 24usize.saturating_sub(being.name().len());
        line.push_str(&" ".repeat(padding));
        if index % 3 == 2 {
            output.push_str(&line);
            output.push('\n');
            line.clear();
        }
    }
    if !line.is_empty() {
        output.push_str(&line);
        output.push('\n');
    }
    output
}

fn format_top(beings: &[BeingSummary], current_date: u32) -> String {
    let mut ranked = beings.iter().collect::<Vec<_>>();
    ranked.sort_by_key(|being| std::cmp::Reverse(being.honor()));
    let mut output = String::new();
    for being in ranked.into_iter().take(10) {
        let sex = if being.is_female() { "Female" } else { "Male" };
        let age = current_date.saturating_sub(being.date_of_birth());
        output.push_str(&format!(
            "{:03}   {}{}{}\t{}  days\n",
            being.honor(),
            being.name(),
            " ".repeat(25usize.saturating_sub(being.name().len())),
            sex,
            age
        ));
    }
    output
}

fn format_stats(being: &BeingSummary, current_date: u32) -> String {
    let [x, y] = being.location();
    let sex = if being.is_female() {
        "Female"
    } else {
        "Male  "
    };
    let drives = being.drives();
    let age = current_date.saturating_sub(being.date_of_birth());
    let drive = being.drive_description();
    let status = being.state_description();
    format!(
        "{} ({} {}) {}\nH:{:03} S:{:03} F:{:03} X:{:03}: {}: {}\nGen 0:0  {}  ERG:{} SPD:{}\nHonor:{}  HEI:{}  Old(Years:{}  Days:{})\nAware Body: {} Link: {}\nFriend\nEnemy\n\n",
        being.name(),
        x / 16,
        y / 16,
        sixteenth_wind(being.facing()),
        drives[0],
        drives[1],
        drives[2],
        drives[3],
        drive,
        status,
        sex,
        being.energy(),
        being.ten_minute_distance(),
        being.honor(),
        being.real_height_mm(),
        age / TIME_YEAR_DAYS as u32,
        age % TIME_YEAR_DAYS as u32,
        being.body_attention_description(),
        relationship_description(being.attention()[ATTENTION_RELATIONSHIP])
    )
}

fn format_appearance(being: &BeingSummary) -> String {
    format!(
        "Height: {:.3} m\nMass: {:.2} Kg\nBody fat: {:.2} Kg\nBody frame: {:02}\n",
        f32::from(being.real_height_mm()) / 1000.0,
        f32::from(being.mass()) / 100.0,
        f32::from(being.body_fat()) / 100.0,
        being.body_frame()
    )
}

fn format_genome(being: &BeingSummary) -> String {
    let mut output = String::new();
    for phase in 0..2 {
        for (index, gene) in being.genetics().iter().enumerate() {
            if index > 0 {
                output.push('\t');
            }
            output.push_str(&gene_to_letters(*gene, phase));
        }
        output.push('\n');
    }
    output
}

fn format_social_graph(being: &BeingSummary) -> String {
    let mut output = format!("\nSocial graph for {}\n\nFriends:\n", being.name());
    append_social_rows(&mut output, being, true);
    output.push_str("\nEnemies:\n");
    append_social_rows(&mut output, being, false);
    output
}

fn format_episodic(being: &BeingSummary, current_date: u32, current_time: u32) -> String {
    let mut output = format!("\nEpisodic memory for {}\n", being.name());
    let attention = being.attention()[apesdk_sim::ATTENTION_EPISODE] as usize;
    for (index, entry) in being.episodic_memory().iter().enumerate() {
        if entry.event == 0 || entry.first_name[0] != being.gender_name() {
            continue;
        }
        let Some(description) = episodic_description(being, entry, current_date, current_time)
        else {
            continue;
        };
        if index == attention {
            output.push_str(" <");
            output.push_str(&description);
            output.push_str(">\n");
        } else {
            output.push_str("  ");
            output.push_str(&description);
            output.push('\n');
        }
    }
    output
}

fn episodic_description(
    being: &BeingSummary,
    entry: &apesdk_sim::simulated_iepisodic,
    current_date: u32,
    current_time: u32,
) -> Option<String> {
    let intention = entry.event & EVENT_INTENTION != 0;
    let event = entry.event & (EVENT_INTENTION - 1);
    let mut output = String::new();
    if intention {
        output.push_str("Intends ");
    }

    match event {
        EVENT_EAT => {
            output.push_str("Was eating ");
            output.push_str(match entry.food {
                FOOD_VEGETABLE => "vegetation",
                FOOD_FRUIT => "fruit",
                FOOD_SHELLFISH => "shellfish",
                FOOD_SEAWEED => "seaweed",
                _ => "food",
            });
        }
        EVENT_GROOM => {
            output.push_str("Groomed *");
            output.push_str(&social_name(
                entry.first_name[BEING_MET],
                entry.family_name[BEING_MET],
            ));
            output.push_str("*'s ");
            output.push_str(apesdk_sim::body_inventory_description(entry.arg as u8));
        }
        EVENT_GROOMED => {
            output.push_str("Groomed by *");
            output.push_str(&social_name(
                entry.first_name[BEING_MET],
                entry.family_name[BEING_MET],
            ));
            output.push('*');
        }
        EVENT_CHAT => {
            output.push_str("Chatted with *");
            output.push_str(&social_name(
                entry.first_name[BEING_MET],
                entry.family_name[BEING_MET],
            ));
            output.push('*');
        }
        EVENT_HIT => {
            output.push_str("Hit *");
            output.push_str(&social_name(
                entry.first_name[BEING_MET],
                entry.family_name[BEING_MET],
            ));
            output.push('*');
        }
        EVENT_HIT_BY => {
            output.push_str("Hit by *");
            output.push_str(&social_name(
                entry.first_name[BEING_MET],
                entry.family_name[BEING_MET],
            ));
            output.push('*');
        }
        EVENT_SEEK_MATE => {
            output.push_str("Searched for *");
            output.push_str(&social_name(
                entry.first_name[BEING_MET],
                entry.family_name[BEING_MET],
            ));
            output.push('*');
        }
        _ => return None,
    }
    output.push_str(&elapsed_description(
        entry.space_time.date,
        entry.space_time.time,
        current_date,
        current_time,
    ));
    output.push_str(&format!(
        " affect:{:+}",
        i32::from(entry.affect) - i32::from(EPISODIC_AFFECT_ZERO)
    ));
    let _ = being;
    Some(output)
}

fn elapsed_description(
    event_date: u32,
    event_time: u32,
    current_date: u32,
    current_time: u32,
) -> String {
    let days_elapsed = current_date.saturating_sub(event_date);
    if days_elapsed == 0 {
        let minutes = current_time.saturating_sub(event_time);
        return match minutes {
            0 => " now".to_string(),
            1 => " a minute ago".to_string(),
            2..=4 => " a few minutes ago".to_string(),
            5..=59 => format!(" {minutes} minutes ago"),
            60..=119 => " an hour ago".to_string(),
            _ => format!(" {} hours ago", minutes / 60),
        };
    }
    if days_elapsed == 1 {
        " yesterday".to_string()
    } else {
        format!(" {days_elapsed} days ago")
    }
}

fn append_social_rows(output: &mut String, being: &BeingSummary, friends: bool) {
    let social = being.social_memory();
    for entry in social.iter().take(SOCIAL_SIZE_BEINGS).skip(1) {
        if social_entry_empty(entry) || entry.entity_type != ENTITY_BEING {
            continue;
        }

        let is_friend = entry.friend_foe >= SOCIAL_RESPECT_NORMAL;
        if is_friend != friends {
            continue;
        }

        let relationship = if entry.relationship > RELATIONSHIP_SELF {
            format!(" {}", relationship_description(entry.relationship))
        } else {
            String::new()
        };
        output.push_str(&format!(
            "    {:05}  *{}*{} {}\n",
            entry.familiarity,
            social_name(entry.first_name[BEING_MET], entry.family_name[BEING_MET]),
            relationship,
            u8::from(entry.attraction > 0)
        ));
    }
}

fn social_entry_empty(entry: &apesdk_sim::simulated_isocial) -> bool {
    entry.first_name[BEING_MET] == 0
        && entry.family_name[BEING_MET] == 0
        && entry.relationship <= RELATIONSHIP_SELF
}

fn social_name(first_name: u16, family_name: u16) -> String {
    if first_name == 0 && family_name == 0 {
        "Unknown".to_string()
    } else {
        format!("{first_name:05}-{family_name:05}")
    }
}

fn format_pathogen(being: &BeingSummary) -> String {
    let [first, second] = being.immune_seed();
    format!("AB( {} ) = 1\nAG( {} ) = 1\n", first & 255, second & 255)
}

fn format_braincode(being: &BeingSummary) -> String {
    let registers = being.braincode_register();
    format!(
        "\nBraincode for {}\n\nRegisters:\n{}{}{}\n\n",
        being.name(),
        registers[0] as char,
        registers[1] as char,
        registers[2] as char
    )
}

fn format_brainprobes(being: &BeingSummary) -> String {
    format!(
        "\nBrain probes for {}\n  Type    Posn  Freq Offset Addr State\n  ------------------------------------\n",
        being.name()
    )
}

fn format_speech(being: &BeingSummary) -> String {
    let registers = being.braincode_register();
    format!(
        "\nSpeech for {}\n{}{}{}.\n",
        being.name(),
        (registers[0] as char).to_ascii_lowercase(),
        (registers[1] as char).to_ascii_lowercase(),
        (registers[2] as char).to_ascii_lowercase()
    )
}

fn gene_to_letters(gene: u32, phase: usize) -> String {
    const LETTERS: [char; 4] = ['A', 'T', 'C', 'G'];
    let mut output = String::new();
    for index in 0..8 {
        let nucleotide = ((gene >> (index * 4 + phase * 2)) & 3) as usize;
        output.push(LETTERS[nucleotide]);
    }
    output
}

fn sixteenth_wind(facing: u8) -> &'static str {
    const WINDS: [&str; 16] = [
        "E  ", "ESE", "SE ", "SSE", "S  ", "SSW", "SW ", "WSW", "W  ", "WNW", "NW ", "NNW", "N  ",
        "NNE", "NE ", "ENE",
    ];
    WINDS[((facing.wrapping_add(7) >> 4) & 15) as usize]
}

impl Default for Console {
    fn default() -> Self {
        Self::new(DEFAULT_RANDOMISE)
    }
}

fn split_command(line: &str) -> (&str, Option<&str>) {
    if let Some((command, response)) = line.split_once(char::is_whitespace) {
        (command, Some(response.trim_start()))
    } else {
        (line, None)
    }
}

fn find_command(command: &str) -> Option<&'static CommandEntry> {
    COMMANDS.iter().find(|entry| entry.command == command)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RunInterval {
    number: n_uint,
    minutes: n_uint,
    description: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunParseError {
    Forever,
    MissingTime,
}

fn parse_run_interval(response: Option<&str>) -> Result<RunInterval, RunParseError> {
    let response = response.ok_or(RunParseError::MissingTime)?.trim();
    if response.is_empty() {
        return Err(RunParseError::MissingTime);
    }
    if response.contains("forever") {
        return Err(RunParseError::Forever);
    }

    let mut pieces = response.split_whitespace();
    let number = pieces
        .next()
        .and_then(|number| number.parse::<n_uint>().ok())
        .filter(|number| *number > 0)
        .ok_or(RunParseError::MissingTime)?;
    let (steps, description) = interval_steps_and_description(pieces.next());

    Ok(RunInterval {
        number,
        minutes: number.saturating_mul(steps),
        description,
    })
}

fn interval_steps_and_description(unit: Option<&str>) -> (n_uint, &'static str) {
    let Some(unit) = unit else {
        return (TIME_DAY_MINUTES as n_uint, " days");
    };
    if unit.len() == 1 {
        return match unit.as_bytes()[0] {
            b'm' => (1, " mins"),
            b'M' => (TIME_MONTH_MINUTES as n_uint, " months"),
            b'h' | b'H' => (TIME_HOUR_MINUTES as n_uint, " hours"),
            b'd' | b'D' => (TIME_DAY_MINUTES as n_uint, " days"),
            b'y' | b'Y' => (TIME_YEAR_MINUTES as n_uint, " years"),
            _ => (TIME_DAY_MINUTES as n_uint, " days"),
        };
    }

    let unit = unit.to_ascii_lowercase();
    if unit.contains("min") {
        (1, " mins")
    } else if unit.contains("hour") || unit.contains("hr") {
        (TIME_HOUR_MINUTES as n_uint, " hours")
    } else if unit.contains("day") {
        (TIME_DAY_MINUTES as n_uint, " days")
    } else if unit.contains("mon") {
        (TIME_MONTH_MINUTES as n_uint, " months")
    } else if unit.contains("year") {
        (TIME_YEAR_MINUTES as n_uint, " years")
    } else {
        (TIME_DAY_MINUTES as n_uint, " days")
    }
}

fn help_line(entry: &CommandEntry) -> String {
    let mut output = format!(" {} {}", entry.command, entry.addition);
    let padding = 28usize.saturating_sub(entry.command.len() + entry.addition.len() + 1);
    output.push_str(&" ".repeat(padding));
    output.push_str(entry.help);
    output
}

fn spacetime_to_string(time: u32, date: u32) -> String {
    let minutes = time as usize;
    let days = date as usize;
    let military_time = (minutes % 60) + (minutes / 60) * 100;
    let days_month = (days % 28) + 1;
    let month = ((days / 28) % 13) + 1;
    let years = days / TIME_YEAR_DAYS;

    debug_assert_eq!(TIME_DAY_MINUTES, 1_440);
    debug_assert_eq!(TIME_MONTH_MINUTES, 40_320);

    format!(
        "{:02}:{:02} {:02}/{:02}/{}",
        military_time / 100,
        military_time % 100,
        days_month,
        month,
        years
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use apesdk_sim::{
        LandSnapshot, LandState, APE_TO_MAP_BIT_RATIO, BEING_DEAD, BEING_HUNGRY, TIDE_MAX,
    };
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn normalize(value: &str) -> String {
        value.replace("\r\n", "\n")
    }

    fn temp_save_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "simape_{name}_{}_{}.json",
            std::process::id(),
            nanos
        ))
    }

    #[test]
    fn help_transcript_matches_c_golden() {
        let mut console = Console::default();
        let actual = console.run_script(
            include_str!("../../../golden/cli/sessions/help.commands"),
            true,
        );
        let expected = normalize(include_str!("../../../golden/cli/transcripts/help.txt"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn help_errors_transcript_matches_c_golden() {
        let mut console = Console::default();
        let actual = console.run_script(
            include_str!("../../../golden/cli/sessions/help_errors.commands"),
            true,
        );
        let expected = normalize(include_str!(
            "../../../golden/cli/transcripts/help_errors.txt"
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn runtime_parity_transcript_matches_rust_golden() {
        let mut console = Console::default();
        let actual = console.run_script(
            include_str!("../../../golden/cli/sessions/runtime_parity.commands"),
            true,
        );
        let expected = normalize(include_str!(
            "../../../golden/cli/transcripts/runtime_parity.txt"
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn simulation_command_reports_empty_seeded_startup_state() {
        let mut console = Console::default();
        let actual = console.run_script("sim\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n00:00 01/01/0 Simulation not running\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn memory_and_empty_ape_commands_match_startup_smoke_shape() {
        let mut console = Console::default();
        let actual = console.run_script("memory\nape\nquit\n", true);
        assert!(actual.contains("maximum memory 18960829\nallocated memory 0\nmaximum apes 256\n"));
        assert!(actual.contains("ape\n*** ALL APES DEAD ***\n"));
    }

    #[test]
    fn list_aliases_report_empty_population_like_c() {
        let mut console = Console::default();
        let actual = console.run_script("list\nls\ndir\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nlist\nNo apes present. Trying (re)running the Simulation\nls\nNo apes present. Trying (re)running the Simulation\ndir\nNo apes present. Trying (re)running the Simulation\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn interval_command_reports_and_updates_logging_interval() {
        let mut console = Console::default();
        let actual = console.run_script(
            "interval\ninterval 2 hours\ninterval\ninterval 30 minutes\ninterval\ninterval 1 day\ninterval\ninterval 0 minutes\nquit\n",
            true,
        );
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\ninterval\nCurrent time interval is 1 hour(s)\ninterval 2 hours\nLogging interval set to 2 hours\ninterval\nCurrent time interval is 2 hour(s)\ninterval 30 minutes\nLogging interval set to 30 mins\ninterval\nCurrent time interval is 30 min(s)\ninterval 1 day\nLogging interval set to 1 days\ninterval\nCurrent time interval is 1 day(s)\ninterval 0 minutes\nCurrent time interval is 1 day(s)\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn logging_command_toggles_state_and_ignores_missing_values() {
        let mut console = Console::default();
        let actual = console.run_script("logging off\nlogging\nlog yes\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nlogging off\nLogging turned off\nlogging\nlog yes\nLogging turned on\nquit\nSimulation stopped\n"
        );
        assert!(console.logging_enabled());
    }

    #[test]
    fn event_command_toggles_c_like_event_modes() {
        let mut console = Console::default();
        let actual = console.run_script("event\nevent off\nevent social\nevent on\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nevent\nEvent output turned on\nevent off\nEvent output turned off\nevent social\nEvent output for social turned on\nevent on\nEvent output turned on\nquit\nSimulation stopped\n"
        );
        assert_eq!(console.event_mode_name(), "all");
    }

    #[test]
    fn file_command_prints_format_sections_fields_and_missing_errors() {
        let mut console = Console::default();
        let actual = console.run_script("file landd\nfile timed\nfile xxxxx\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nfile landd\n landd land definition\n  dated Date in days and millenia\n  timed Time in minutes\n  landg Seed that created the land\nfile timed\n  timed Time in minutes\nfile xxxxx\nERROR: String not found @ ./toolkit/file.c 1458\nquit\nSimulation stopped\n"
        );

        let (all_format, should_quit) = console.execute_line("file");
        assert!(!should_quit);
        assert!(all_format.starts_with(" simul Simulation Version Definition\n"));
        assert!(all_format.contains(" being Being Definition\n"));
        assert!(all_format.ends_with("  brpro Brain code probe\n"));
    }

    #[test]
    fn script_command_matches_default_build_open_behavior() {
        let path = temp_save_path("script_open");
        let path_string = path.to_string_lossy();
        fs::write(
            &path,
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[77,88],\"time\":45}}",
        )
        .expect("fixture JSON should be writable");
        let missing = temp_save_path("script_missing");
        let missing_string = missing.to_string_lossy();

        let mut console = Console::default();
        let actual = console.run_script(
            &format!("script {missing_string}\nscript\nscript {path_string}\nsim\nquit\n"),
            true,
        );
        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nscript {missing_string}\nSimulation stopped\nscript\nscript {path_string}\nSimulation stopped\nSimulation file {path_string} open\n\nsim\nMap dimension: 512\nLand seed: 77 88\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 130\n00:45 01/01/0 Simulation not running\nquit\nSimulation stopped\n"
            )
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn alpha_command_creates_empty_speech_file_without_console_output() {
        let path = temp_save_path("alpha");
        let path_string = path.to_string_lossy();
        let mut console = Console::default();
        let actual = console.run_script(&format!("alpha {path_string}\nquit\n"), true);
        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nalpha {path_string}\nquit\nSimulation stopped\n"
            )
        );
        assert_eq!(
            fs::metadata(&path).expect("alpha file should exist").len(),
            0
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn speak_command_is_quiet_for_empty_unselected_population() {
        let path = temp_save_path("speak");
        let path_string = path.to_string_lossy();
        let mut console = Console::default();
        let actual = console.run_script(&format!("speak {path_string}\nspeak\nquit\n"), true);
        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nspeak {path_string}\nspeak\nquit\nSimulation stopped\n"
            )
        );
        assert!(!path.exists());
    }

    #[test]
    fn debug_command_prints_c_layout_audit_snapshot() {
        let mut console = Console::default();
        let actual = console.run_script("debug\nquit\n", true);
        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\ndebug\n{DEBUG_AUDIT_OUTPUT}quit\nSimulation stopped\n"
            )
        );
    }

    #[test]
    fn top_and_epic_empty_population_output_matches_c() {
        let mut console = Console::default();
        let actual = console.run_script("top\nepic\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\ntop\nHonor Name                     Sex\tAge\n-----------------------------------------------------------------\nepic\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn navigation_aliases_report_no_selected_ape_like_c_empty_state() {
        let mut console = Console::default();
        let actual = console.run_script("next\nprevious\nprev\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nnext\nNo apes selected. Trying (re)running the Simulation\nprevious\nNo apes selected. Trying (re)running the Simulation\nprev\nNo apes selected. Trying (re)running the Simulation\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn watch_aliases_report_no_selected_ape_before_parsing_response() {
        let mut console = Console::default();
        let actual = console.run_script("watch\nmonitor off\nwatch all\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nwatch\nNo apes selected. Trying (re)running the Simulation\nmonitor off\nNo apes selected. Trying (re)running the Simulation\nwatch all\nNo apes selected. Trying (re)running the Simulation\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn social_and_pathogen_detail_commands_match_empty_duplicate_errors() {
        let mut console = Console::default();
        let actual = console.run_script(
            "friends\nsocial\nsocialgraph Ada\ngraph\npathogen Ada\nquit\n",
            true,
        );
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nfriends\nERROR: No being was specified @ ./universe/command.c 1311\nsocial\nERROR: No being was specified @ ./universe/command.c 1311\nsocialgraph Ada\nERROR: Being not found @ ./universe/command.c 1300\ngraph\nERROR: No being was specified @ ./universe/command.c 1311\npathogen Ada\nERROR: Being not found @ ./universe/command.c 1300\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn brain_speech_and_episodic_detail_commands_match_empty_duplicate_errors() {
        let mut console = Console::default();
        let actual =
            console.run_script("braincode\nspeech Ada\nepisodic\nprobes Ada\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nbraincode\nERROR: No being was specified @ ./universe/command.c 1311\nspeech Ada\nERROR: Being not found @ ./universe/command.c 1300\nepisodic\nERROR: No being was specified @ ./universe/command.c 1311\nprobes Ada\nERROR: Being not found @ ./universe/command.c 1300\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn stats_and_appearance_detail_commands_match_empty_duplicate_errors() {
        let mut console = Console::default();
        let actual =
            console.run_script("stats\nstatus Ada\nappearance\nphysical Ada\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nstats\nERROR: No being was specified @ ./universe/command.c 1311\nstatus Ada\nERROR: Being not found @ ./universe/command.c 1300\nappearance\nERROR: No being was specified @ ./universe/command.c 1311\nphysical Ada\nERROR: Being not found @ ./universe/command.c 1300\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn genome_detail_commands_error_and_idea_is_quiet_for_empty_population() {
        let mut console = Console::default();
        let actual = console.run_script("genome\ngenetics Ada\nidea\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\ngenome\nERROR: No being was specified @ ./universe/command.c 1311\ngenetics Ada\nERROR: Being not found @ ./universe/command.c 1300\nidea\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn reset_and_clear_regenerate_startup_land_seed() {
        let mut console = Console::default();
        let actual = console.run_script("reset\nsim\nclear\nsim\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nreset\nSimulation reset\nsim\nMap dimension: 512\nLand seed: 23809 53481\nPopulation: 128\nAdults: 128   Juveniles: 0\nTide level: 0\n00:00 01/01/0 Simulation not running\nclear\nSimulation reset\nsim\nMap dimension: 512\nLand seed: 50588 11145\nPopulation: 128\nAdults: 128   Juveniles: 0\nTide level: 0\n00:00 01/01/0 Simulation not running\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn reset_creates_selectable_named_population() {
        let mut console = Console::default();
        let actual = console.run_script("reset\nape\nnext\nape\nprevious\nape\nlist\nquit\n", true);
        assert!(actual.contains("reset\nSimulation reset\nape\nApe 001\n"));
        assert!(actual.contains("next\nape\nApe 002\nprevious\nape\nApe 001\n"));
        assert!(actual.contains("list\nApe 001                 Ape 002                 Ape 003"));
        assert!(actual.contains("Ape 127                 Ape 128"));
    }

    #[test]
    fn populated_detail_commands_report_summary_data() {
        let mut console = Console::default();
        let actual = console.run_script("reset\nstats\nappearance\ngenome\nquit\n", true);
        assert!(actual.contains("stats\nApe 001 ("));
        assert!(actual.contains("H:064 S:080 F:096 X:112: Mixed drives: Awake\nGen 0:0  Male"));
        assert!(actual.contains("Honor:0  HEI:61"));
        assert!(actual.contains("Aware Body: Head Link: Associate\n"));
        assert!(actual.contains("appearance\nHeight: 0.061 m\nMass: 1.00 Kg\nBody fat:"));
        assert!(actual.contains("\nBody frame: "));

        let genome = actual
            .split("genome\n")
            .nth(1)
            .expect("genome output should be present")
            .split("quit\n")
            .next()
            .expect("quit should follow genome output");
        let genome_lines = genome.lines().collect::<Vec<_>>();
        assert_eq!(genome_lines.len(), 2);
        for line in genome_lines {
            let chromosomes = line.split('\t').collect::<Vec<_>>();
            assert_eq!(chromosomes.len(), 4);
            assert!(chromosomes.iter().all(|chromosome| chromosome.len() == 8));
        }
    }

    #[test]
    fn watch_can_select_named_ape_and_report_watch_modes() {
        let mut console = Console::default();
        let actual = console.run_script(
            "reset\nwatch Ape 010\nape\nwatch off\nwatch state\nwatch Missing\nquit\n",
            true,
        );
        assert!(actual.contains("watch Ape 010\nWatching Ape 010\nape\nApe 010\n"));
        assert!(actual.contains("watch off\nStopped watching\n"));
        assert!(actual.contains("watch state\nWatching being states\n"));
        assert!(actual.contains("watch Missing\nBeing not found\n"));
    }

    #[test]
    fn top_lists_populated_honor_rows() {
        let mut console = Console::default();
        let actual = console.run_script("reset\ntop\nquit\n", true);
        assert!(actual.contains("top\nHonor Name                     Sex\tAge\n"));
        assert!(actual.contains("099   Ape 100"));
        assert!(actual.contains("098   Ape 099"));
        assert!(actual.contains("\t0  days\n"));
    }

    #[test]
    fn populated_social_pathogen_brain_speech_and_idea_commands_report_summaries() {
        let mut console = Console::default();
        let actual = console.run_script(
            "reset\nwatch Ape 010\nfriends\npathogen\nepisodic\nbraincode\nprobes\nspeech\nidea\nquit\n",
            true,
        );
        assert!(actual.contains("friends\n\nSocial graph for Ape 010\n\nFriends:\n\nEnemies:\n"));
        assert!(actual.contains("pathogen\nAB( "));
        assert!(actual.contains("AG( "));
        assert!(actual.contains("episodic\n\nEpisodic memory for Ape 010\n"));
        assert!(actual.contains("braincode\n\nBraincode for Ape 010\n\nRegisters:\nJKL\n\n"));
        assert!(actual.contains("probes\n\nBrain probes for Ape 010\n"));
        assert!(actual.contains("speech\n\nSpeech for Ape 010\njkl.\n"));
        assert!(actual.contains("idea\nMatches 000.0000 percent\n"));
        assert!(actual.contains("08    000.0000  0000\n"));
    }

    #[test]
    fn populated_social_graph_prints_data_backed_friend_and_enemy_rows() {
        let path = temp_save_path("social_graph");
        let path_string = path.to_string_lossy();
        fs::write(
            &path,
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":0},\"beings\":[{\"name\":\"Social Ape\",\"delta\":{\"stored_energy\":3840},\"constant\":{\"date_of_birth\":0,\"genetics\":[2,3,4,5]},\"events\":{\"social\":[{\"relationship\":1,\"entity_type\":0,\"friend_foe\":127},{\"first_name\":[0,111],\"family_name\":[0,222],\"friend_foe\":127,\"familiarity\":42,\"relationship\":2,\"entity_type\":0,\"attraction\":1},{\"first_name\":[0,333],\"family_name\":[0,444],\"friend_foe\":0,\"familiarity\":7,\"relationship\":0,\"entity_type\":0}]}}]}",
        )
        .expect("fixture JSON should be writable");

        let mut console = Console::default();
        let actual = console.run_script(&format!("open {path_string}\nfriends\nquit\n"), true);

        assert!(actual.contains("\nSocial graph for Social Ape\n\nFriends:\n"));
        assert!(actual.contains("    00042  *00111-00222* Mother 1\n"));
        assert!(actual.contains("\nEnemies:\n    00007  *00333-00444* 0\n"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn episodic_command_prints_data_backed_memory_rows() {
        let path = temp_save_path("episodic_rows");
        let path_string = path.to_string_lossy();
        fs::write(
            &path,
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[1,2],\"time\":10},\"beings\":[{\"name\":\"Episode Ape\",\"gender name\":512,\"family name\":258,\"delta\":{\"stored_energy\":3840},\"constant\":{\"date_of_birth\":0,\"name\":[512,258],\"genetics\":[2,3,4,5]},\"braindata\":{\"attention\":[0,0,0,0,0,0]},\"events\":{\"episodic\":[{\"space_time\":{\"date\":0,\"location\":[10,20],\"time\":10},\"first_name\":[512,0],\"family_name\":[258,0],\"event\":1,\"food\":0,\"affect\":16434,\"arg\":50},{\"space_time\":{\"date\":0,\"location\":[10,20],\"time\":9},\"first_name\":[512,768],\"family_name\":[258,300],\"event\":6,\"food\":0,\"affect\":16484,\"arg\":2}]}}]}",
        )
        .expect("fixture JSON should be writable");

        let mut console = Console::default();
        let actual = console.run_script(&format!("open {path_string}\nepisodic\nquit\n"), true);

        assert!(actual.contains("\nEpisodic memory for Episode Ape\n"));
        assert!(actual.contains("<Was eating vegetation now affect:+50>\n"));
        assert!(actual.contains("Groomed *00768-00300*'s Back a minute ago affect:+100\n"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn hungry_runtime_records_food_transcript() {
        let land = LandState::from_snapshot(LandSnapshot::new(0, [7633, 53305], 400));
        let location = (0..MAP_DIMENSION)
            .step_by(5)
            .flat_map(|map_x| {
                (0..MAP_DIMENSION).step_by(5).map(move |map_y| {
                    [
                        (map_x << APE_TO_MAP_BIT_RATIO) as u16,
                        (map_y << APE_TO_MAP_BIT_RATIO) as u16,
                    ]
                })
            })
            .find(|location| {
                land.height_at(*location) > TIDE_MAX
                    && land.food_source_at(*location).max_energy > BEING_DEAD
            })
            .expect("seeded land should expose edible runtime food");
        let path = temp_save_path("hungry_food_transcript");
        let path_string = path.to_string_lossy();
        fs::write(
            &path,
            format!(
                "{{\"information\":{{\"signature\":20033,\"version number\":708}},\"land\":{{\"date\":0,\"genetics\":[7633,53305],\"time\":400}},\"beings\":[{{\"name\":\"Hungry Ape\",\"delta\":{{\"location\":[{},{}],\"velocity\":0,\"stored_energy\":{},\"random_seed\":[1,2],\"awake\":2}},\"constant\":{{\"date_of_birth\":0,\"name\":[512,258],\"genetics\":[4294967295,4294967295,4294967295,4294967295]}}}}]}}",
                location[0],
                location[1],
                BEING_HUNGRY - 1
            ),
        )
        .expect("hungry food fixture should be writable");

        let mut console = Console::default();
        let actual = console.run_script(
            &format!("open {path_string}\nrun 1 minute\nepisodic\nquit\n"),
            true,
        );
        assert!(actual.contains("Running for 1 mins\n"));
        assert!(actual.contains("Episodic memory for Hungry Ape\n"));
        assert!(actual.contains("Was eating "));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn open_rejects_binary_native_save_until_binary_parity_is_ported() {
        let path = temp_save_path("binary_reject");
        let path_string = path.to_string_lossy();
        fs::write(&path, b"NA\x02\xc4not-json").expect("binary fixture should be writable");

        let mut console = Console::default();
        let actual = console.run_script(&format!("open {path_string}\nquit\n"), true);

        assert!(actual.contains("ERROR: Failed to read in file @ ./universe/command.c 2394\n"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn step_and_run_advance_populated_simulation_without_unimplemented_errors() {
        let mut console = Console::default();
        let actual =
            console.run_script("reset\nstep\nsim\nrun 2 minutes\nsim\nstats\nquit\n", true);
        assert!(!actual.contains("Simulated ape cycling not implemented"));
        assert!(!actual.contains("Simulated ape running not implemented"));
        assert!(actual.contains("step\nsim\nMap dimension: 512\nLand seed: 23809 53481\nPopulation: 128\nAdults: 128   Juveniles: 0\nTide level: 128\n00:01 01/01/0 Simulation not running\n"));
        assert!(actual.contains("run 2 minutes\nRunning for 2 mins\nsim\nMap dimension: 512\nLand seed: 23809 53481\nPopulation: 128\nAdults: 128   Juveniles: 0\nTide level: 128\n00:03 01/01/0 Simulation not running\n"));
        assert!(actual.contains("stats\nApe 001 ("));
    }

    #[test]
    fn step_and_run_advance_empty_land_time() {
        let mut console = Console::default();
        let actual = console.run_script(
            "step\nsim\nrun 1 minute\nsim\nrun 1 day\nsim\nrun forever\nrun\nquit\n",
            true,
        );
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nstep\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 128\n00:01 01/01/0 Simulation not running\nrun 1 minute\nRunning for 1 mins\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 128\n00:02 01/01/0 Simulation not running\nrun 1 day\nRunning for 1 days\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 129\n00:02 02/01/0 Simulation not running\nrun forever\nERROR: Run forever not implemented in Rust port yet\nrun\nERROR: Time not specified, examples: run 2 days, run 6 hours @ ./universe/command.c 2211\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn run_interval_parser_matches_c_unit_names() {
        assert_eq!(
            parse_run_interval(Some("2 hours")).unwrap(),
            RunInterval {
                number: 2,
                minutes: 120,
                description: " hours",
            }
        );
        assert_eq!(
            parse_run_interval(Some("1 M")).unwrap(),
            RunInterval {
                number: 1,
                minutes: TIME_MONTH_MINUTES as n_uint,
                description: " months",
            }
        );
        assert_eq!(
            parse_run_interval(Some("1 y")).unwrap(),
            RunInterval {
                number: 1,
                minutes: TIME_YEAR_MINUTES as n_uint,
                description: " years",
            }
        );
        assert_eq!(
            parse_run_interval(Some("2")).unwrap(),
            RunInterval {
                number: 2,
                minutes: (TIME_DAY_MINUTES * 2) as n_uint,
                description: " days",
            }
        );
        assert_eq!(
            parse_run_interval(Some("forever")).unwrap_err(),
            RunParseError::Forever
        );
    }

    #[test]
    fn save_after_step_persists_advanced_land_time() {
        let path = temp_save_path("step_save");
        let path_string = path.to_string_lossy();
        let mut console = Console::default();
        let actual = console.run_script(&format!("step\nsave {path_string}\nquit\n"), true);

        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nstep\nsave {path_string}\nSimulation stopped\nSimulation file {path_string} saved\n\nquit\nSimulation stopped\n"
            )
        );
        assert_eq!(
            fs::read(&path).expect("saved JSON should be readable"),
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":0,\"genetics\":[7633,53305],\"time\":1}}"
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn save_command_writes_startup_transfer_json() {
        let path = temp_save_path("startup_save");
        let path_string = path.to_string_lossy();
        let mut console = Console::default();
        let actual = console.run_script(&format!("save {path_string}\nquit\n"), true);

        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nsave {path_string}\nSimulation stopped\nSimulation file {path_string} saved\n\nquit\nSimulation stopped\n"
            )
        );
        assert_eq!(
            fs::read(&path).expect("saved JSON should be readable"),
            b"{\"information\":{\"signature\":20033,\"version number\":708,\"copyright\":\"Copyright Tom Barbalet, 1996-2026.\",\"date\":\"May  1 2026\"},\"land\":{\"date\":0,\"genetics\":[7633,53305],\"time\":0}}"
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn save_without_filename_noops_like_c_command() {
        let mut console = Console::default();
        let actual = console.run_script("save\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nsave\nquit\nSimulation stopped\n"
        );
    }

    #[test]
    fn open_saved_json_restores_startup_state() {
        let path = temp_save_path("open_success");
        let path_string = path.to_string_lossy();
        fs::write(
            &path,
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":27,\"genetics\":[1,2],\"time\":300}}",
        )
        .expect("fixture JSON should be writable");

        let mut console = Console::default();
        let actual = console.run_script(&format!("open {path_string}\nsim\nquit\n"), true);

        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nopen {path_string}\nSimulation stopped\nSimulation file {path_string} open\n\nsim\nMap dimension: 512\nLand seed: 1 2\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 132\n05:00 28/01/0 Simulation not running\nquit\nSimulation stopped\n"
            )
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn open_saved_json_restores_being_population_and_selection() {
        let path = temp_save_path("population_roundtrip");
        let path_string = path.to_string_lossy();
        let mut console = Console::default();
        let saved = console.run_script(&format!("reset\nsave {path_string}\nquit\n"), true);
        assert!(saved.contains("Simulation file "));
        let saved_json = fs::read_to_string(&path).expect("saved population JSON should exist");
        assert!(saved_json.contains("\"beings\":[{\"name\":\"Ape 001\""));
        assert!(saved_json.contains("\"delta\":{\"direction_facing\""));
        assert!(saved_json.contains("\"constant\":{\"date_of_birth\""));
        assert!(saved_json.contains("\"changes\":{\"drives\""));
        assert!(saved_json.contains("\"braindata\":{\"braincode_register\""));
        assert!(saved_json.contains("\"immune_system\":{\"antigens\""));
        assert!(saved_json.contains("\"random_seed\""));

        let mut loaded_console = Console::default();
        let actual =
            loaded_console.run_script(&format!("open {path_string}\nsim\nape\nquit\n"), true);
        assert_eq!(
            actual,
            format!(
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nopen {path_string}\nSimulation stopped\nSimulation file {path_string} open\n\nsim\nMap dimension: 512\nLand seed: 23809 53481\nPopulation: 128\nAdults: 128   Juveniles: 0\nTide level: 128\n00:00 01/01/0 Simulation not running\nape\nApe 001\nquit\nSimulation stopped\n"
            )
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_alias_reads_saved_json_and_malformed_json_fails() {
        let path = temp_save_path("load_alias");
        let path_string = path.to_string_lossy();
        let bad_path = temp_save_path("load_bad");
        let bad_path_string = bad_path.to_string_lossy();
        fs::write(
            &path,
            b"{\"information\":{\"signature\":20033,\"version number\":708},\"land\":{\"date\":0,\"genetics\":[33,44],\"time\":0}}",
        )
        .expect("fixture JSON should be writable");
        fs::write(&bad_path, b"{\"information\":true}")
            .expect("bad fixture JSON should be writable");

        let mut console = Console::default();
        let actual = console.run_script(
            &format!("load {path_string}\nsim\nopen {bad_path_string}\nquit\n"),
            true,
        );

        assert!(actual.contains(&format!("Simulation file {path_string} open\n\n")));
        assert!(actual.contains("Land seed: 33 44\n"));
        assert!(actual.contains("ERROR: Failed to read in file @ ./universe/command.c 2394\n"));
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(bad_path);
    }
}
