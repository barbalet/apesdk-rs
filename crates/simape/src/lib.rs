use apesdk_sim::{
    banner, SimState, LARGE_SIM, MAP_DIMENSION, TIME_DAY_MINUTES, TIME_HOUR_MINUTES,
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
    Unsupported,
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
        action: CommandAction::Unsupported,
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
        action: CommandAction::Unsupported,
        command: "speak",
        addition: "[file]",
        help: "Create an AIFF file of Ape speech",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "alpha",
        addition: "[file]",
        help: "Create an AIFF file of Ape alphabet",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
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
        action: CommandAction::Unsupported,
        command: "watch",
        addition: "(ape name)|all|off|*",
        help: "Watch (specific *) for the current ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "monitor",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
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
        action: CommandAction::Unsupported,
        command: "pathogen",
        addition: "(ape name)",
        help: "* Show pathogens for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "friends",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "social",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "socialgraph",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "graph",
        addition: "(ape name)",
        help: "* Show social graph for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "braincode",
        addition: "(ape name)",
        help: "* Show braincode for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "speech",
        addition: "(ape name)",
        help: "* Show speech for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "episodic",
        addition: "(ape name)",
        help: "* Show episodic memory for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "probes",
        addition: "(ape name)",
        help: "* Show brain probes for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "stats",
        addition: "(ape name)",
        help: "* Show parameters for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "status",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "appearance",
        addition: "(ape name)",
        help: "* Show appearance values for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "physical",
        addition: "",
        help: "",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
        command: "genome",
        addition: "(ape name)",
        help: "Show genome for a named ape",
    },
    CommandEntry {
        action: CommandAction::Unsupported,
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
        action: CommandAction::Unsupported,
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

#[derive(Clone, Debug)]
pub struct Console {
    state: SimState,
    population: usize,
    simulation_running: bool,
    save_interval_steps: n_uint,
    logging_enabled: bool,
}

impl Console {
    pub fn new(randomise: n_uint) -> Self {
        Self {
            state: SimState::start_up(randomise),
            population: 0,
            simulation_running: false,
            save_interval_steps: TIME_HOUR_MINUTES as n_uint,
            logging_enabled: true,
        }
    }

    pub fn startup_text() -> String {
        format!("{}      For a list of commands type 'help'\n\n", banner())
    }

    pub fn logging_enabled(&self) -> bool {
        self.logging_enabled
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
            CommandAction::Interval => (self.interval(response), false),
            CommandAction::Logging => (self.logging(response), false),
            CommandAction::Event => ("Episodic not supported in this build\n".to_string(), false),
            CommandAction::Simulation => (self.simulation(), false),
            CommandAction::Memory => (self.memory(), false),
            CommandAction::Ape => ("*** ALL APES DEAD ***\n".to_string(), false),
            CommandAction::List => (self.list(), false),
            CommandAction::Top => (self.top(), false),
            CommandAction::Epic => (self.epic(), false),
            CommandAction::Navigation => (self.navigation(), false),
            CommandAction::Unsupported => (
                "ERROR: Command not implemented in Rust port yet\n".to_string(),
                false,
            ),
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
        let adults = self.population;
        let juveniles = 0;
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
            self.population,
            land.tide_level(),
            spacetime_to_string(land.time(), land.date())
        )
    }

    fn memory(&self) -> String {
        format!("maximum memory {MAXIMUM_MEMORY}\nallocated memory 0\nmaximum apes {LARGE_SIM}\n")
    }

    fn list(&self) -> String {
        if self.population == 0 {
            "No apes present. Trying (re)running the Simulation\n".to_string()
        } else {
            "ERROR: Ape listing not implemented in Rust port yet\n".to_string()
        }
    }

    fn top(&self) -> String {
        if self.population == 0 {
            "Honor Name                     Sex\tAge\n-----------------------------------------------------------------\n"
                .to_string()
        } else {
            "ERROR: Ape ranking not implemented in Rust port yet\n".to_string()
        }
    }

    fn epic(&self) -> String {
        if self.population == 0 {
            String::new()
        } else {
            "ERROR: Ape episodic ranking not implemented in Rust port yet\n".to_string()
        }
    }

    fn navigation(&self) -> String {
        if self.population == 0 {
            "No apes selected. Trying (re)running the Simulation\n".to_string()
        } else {
            "No apes selected.\n".to_string()
        }
    }

    fn reset(&mut self) -> String {
        self.simulation_running = false;
        self.population = 0;
        self.state.reset_new_simulation_from_land_seed();
        "Simulation reset\n".to_string()
    }

    fn step(&mut self) -> String {
        self.simulation_running = true;
        if self.population == 0 {
            self.state.step_empty();
            self.simulation_running = false;
            String::new()
        } else {
            "ERROR: Simulated ape cycling not implemented in Rust port yet\n".to_string()
        }
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

        let mut output = format!("Running for {}{}\n", interval.number, interval.description);
        self.simulation_running = true;
        if self.population == 0 {
            self.state.step_empty_by(interval.minutes);
            self.simulation_running = false;
        } else {
            output.push_str("ERROR: Simulated ape running not implemented in Rust port yet\n");
        }
        output
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
    fn event_command_reports_episodic_unavailable_for_current_build() {
        let mut console = Console::default();
        let actual = console.run_script("event\nevent social\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nevent\nEpisodic not supported in this build\nevent social\nEpisodic not supported in this build\nquit\nSimulation stopped\n"
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
    fn reset_and_clear_regenerate_startup_land_seed() {
        let mut console = Console::default();
        let actual = console.run_script("reset\nsim\nclear\nsim\nquit\n", true);
        assert_eq!(
            actual,
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nreset\nSimulation reset\nsim\nMap dimension: 512\nLand seed: 23809 53481\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n00:00 01/01/0 Simulation not running\nclear\nSimulation reset\nsim\nMap dimension: 512\nLand seed: 50588 11145\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n00:00 01/01/0 Simulation not running\nquit\nSimulation stopped\n"
        );
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
            "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nstep\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n00:01 01/01/0 Simulation not running\nrun 1 minute\nRunning for 1 mins\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n00:02 01/01/0 Simulation not running\nrun 1 day\nRunning for 1 days\nsim\nMap dimension: 512\nLand seed: 7633 53305\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n00:02 02/01/0 Simulation not running\nrun forever\nERROR: Run forever not implemented in Rust port yet\nrun\nERROR: Time not specified, examples: run 2 days, run 6 hours @ ./universe/command.c 2211\nquit\nSimulation stopped\n"
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
                "\n *** Simulated Ape 0.708 Console, May  1 2026 ***\n      For a list of commands type 'help'\n\nopen {path_string}\nSimulation stopped\nSimulation file {path_string} open\n\nsim\nMap dimension: 512\nLand seed: 1 2\nPopulation: 0\nAdults: 0   Juveniles: 0\nTide level: 0\n05:00 28/01/0 Simulation not running\nquit\nSimulation stopped\n"
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
