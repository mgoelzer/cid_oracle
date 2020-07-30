use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_toml::StructOptToml;
use std::path::PathBuf;

pub const CONFIG_DIR : &str  = ".cid-oracle";  // Under $HOME
const CONFIG_FILENAME : &str = "config.toml";  // Optional config file

#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[serde(default)]
#[structopt(name = "cid_oracle")]
struct Opt {
    /// url of lotus api
    #[structopt(default_value = "http://127.0.0.1:1234/rpc/v0", long, short="e")]
    endpoint: String,
    
    /// start scan at height
    #[structopt(default_value = "0", long)]
    min: u64,

    /// end scan at height (0=never)
    #[structopt(default_value = "18446744073709551615", long)]
    max: u64,
}

/// Returns the path to the config file on this systme, which is 
/// the user's home directory path + ".cid_oracle/config.toml"
/// 
/// # Arguments
///
/// (none)
///
/// # Examples
///
/// ```
/// println!("look for config file at {}",config_file_path);
/// ```
fn config_file_path() -> String {
    let mut homedir : PathBuf = match dirs::home_dir() {
        Some(path) => { path },
        None       => { std::path::PathBuf::from(".") },
    };
    homedir.push(CONFIG_DIR);
    homedir.push(CONFIG_FILENAME);
    match homedir.as_path().to_str() {
        Some(s) => { s.to_string() },
        None => { "".to_string() },
    }
}

#[derive(Debug,Clone)]
pub struct ExecutionParameters {
    pub min: u64,
    pub max: u64,
    pub endpoint: String,
}

impl ExecutionParameters {
    pub fn new(min:u64,max:u64,endpoint:&str) -> ExecutionParameters {
        ExecutionParameters{min:min, max:max, endpoint:endpoint.to_string(),}
    }
}

pub struct ExecutionParametersBuilder {
    execparams : ExecutionParameters,
    min_assigned : bool,
    max_assigned : bool,
    endpoint_assigned : bool,
}

impl ExecutionParametersBuilder {
    /// Construct the builder
    pub fn new() -> ExecutionParametersBuilder {
        ExecutionParametersBuilder{
            execparams : ExecutionParameters::new(0,0,""),
            min_assigned : false,
            max_assigned : false,
            endpoint_assigned : false,
        }
    }

    /// Provide min
    pub fn min_fld<'a>(&'a mut self, min: u64) -> &'a mut ExecutionParametersBuilder {
        self.execparams.min = min;
        self.min_assigned = true;
        self
    }
    /// Provide max
    pub fn max_fld<'a>(&'a mut self, max: u64) -> &'a mut ExecutionParametersBuilder {
        self.execparams.max = max;
        self.max_assigned = true;
        self
    } 
    /// Provide endpoint
    pub fn endpoint_fld<'a>(&'a mut self, endpoint: String) -> &'a mut ExecutionParametersBuilder {
        self.execparams.endpoint = endpoint;
        self.endpoint_assigned = true;
        self
    }
    /// Release the inner ExecutionParameters struct
    pub fn release(&mut self) -> ExecutionParameters {
        assert!(self.min_assigned && self.max_assigned && self.endpoint_assigned, 
            "ExecutionParametersBuilder: release: all parameters must be assigned before release call");
        let mut other_execparams = ExecutionParameters::new(0,0,"");
        std::mem::swap(&mut self.execparams, &mut other_execparams);
        other_execparams
    }
}

/// Parses configuration to return the execution parameters the
/// program will use.  Configuration comes from hard-coded 
/// defaults in the program, which is overridable by 
/// $HOME/.cid-oracle/config.toml (if it exists), which is overridable
/// by command line arguments.
/// 
/// These are the possible arguments:
/// 
///     --min=NNN                 # where NNN is a positive number
///     --max=NNN                 # where NNN is a postive number
///     --endpoint=http://...     # URL of your running Lotus instance
///     --help, -h                # print usage and exit
/// 
/// # Arguments
///
/// (none)
///
/// # Examples
///
/// ```
/// let settings = parse_configuration();
/// println!("endpoint url: {}",settings.endpoint;
/// ```
pub fn parse_configuration() -> ExecutionParameters {
    let opt;

    let config_toml_contents : String;
    if let Ok(contents) = std::fs::read_to_string(config_file_path()) {
        config_toml_contents = contents;
        opt = Opt::from_args_with_toml(&config_toml_contents).expect("toml parsing failed");
    } else {
        log::info!("cli::print_args: file does not exist or is unreadable: '{}'",config_file_path());
        opt = Opt::from_args();
    }

    ExecutionParametersBuilder::new()
        .max_fld(opt.max)
        .min_fld(opt.min)
        .endpoint_fld(opt.endpoint)
        .release()
}
