use std::str::FromStr;
use std::convert::From;
use getopts;
use std::env;

#[derive(Debug)]
pub enum Input {
    Stdin,
    Random(usize),
    File(String),
}

#[derive(Debug)]
pub enum Heuristic {
    Manhattan,
    Error,
}

impl FromStr for Heuristic {
	type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    	match &s.to_lowercase()[..] {
    		"manhattan"	=> Ok(Heuristic::Manhattan),
    		_			=> Err("No such heuristic."),
    	}
    }
}

#[derive(Debug)]
pub struct Options
{
    input:		Input,
    help:		bool,
    heuristic:	Heuristic,
    usage:		String,

    /// True if some args of the cli has not been parsed
    has_free:		bool,
}

impl Options
{
	fn cli_collect(args: &Vec<String>) -> getopts::Options {
	    let mut opts = getopts::Options::new();
	    opts.optopt("f", "file", "set intput file name", "NAME");
	    opts.optopt("r", "random",
	    		"create random npuzzle of size SIZE", "SIZE");
	    opts.optflag("h", "help", "print this help menu");
	    opts.optopt("e", "heuristic", "heuristic to use with A*", "NAME");
	    opts
	}

	fn extract_input(cli_matches: &getopts::Matches) -> Input {
		if cli_matches.opt_present("f") {
			let file_name = cli_matches.opt_str("f").unwrap();
			return Input::File(file_name);
		}
		if cli_matches.opt_present("r") {
			let size_str = cli_matches.opt_str("r").unwrap();
			let size_res = size_str.parse::<usize>();
			match size_res {
				Ok(m) => return Input::Random(m),
				Err(err) => panic!(
						"Cannot parse the --random option to usigned int"),
			}
		}
		Input::Stdin
	}

	fn extract_heuri(cli_matches: &getopts::Matches) -> Heuristic {
		if !cli_matches.opt_present("e") {
			return Heuristic::Manhattan;
		}
		cli_matches.opt_str("e").unwrap().parse::<Heuristic>().unwrap()
	}

	/// Return the options deduced from the command line args.
    fn generate_usage(program: &String, opts: &getopts::Options) -> String {
    	let brief = format!("Usage: {} [options]", program);
	    format!("{}", opts.usage(&brief))
    }

    pub fn new() -> Options
    {
    	let args: Vec<String> = env::args().collect();
	    let program = args[0].clone();
    	let opts = Options::cli_collect(&args[1..].to_vec());
	    let cli_matches = match opts.parse(&args[1..]) {
	        Ok(m) => m,
	        Err(f) => panic!(f.to_string()),
	    };
        Options {
            input:		Options::extract_input(&cli_matches),
            help:		cli_matches.opt_present("h"),
            has_free:	!cli_matches.free.is_empty(),
            heuristic:	Options::extract_heuri(&cli_matches),
            usage:		Options::generate_usage(&program, &opts),
        }
    }

    /// Return true if there is an error in the parsing of the command line
    pub fn check_options(&self) -> bool {
    	if self.help || self.has_free {
    		print!("{}", self.usage);
    		return true;
    	}
    	false
    }
}
