extern crate chrono;
use tai64::Tai64N;
use chrono::offset::{Utc, Local};
use chrono::{DateTime, SecondsFormat};
use clap::{Parser, Subcommand};
use log::debug;
use hex::FromHex;


#[derive(Parser)]
#[command(name = "tai64n")]
#[command(author = "Yonas Yanfa <yonas@mail.lan>")]
#[command(version = "0.0.1")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    /// Enable verbose mode
    #[arg(long)]
    verbose: bool,

    /// Use local timezone
    #[arg(long, default_value_t = false)]
    local: bool,

    /// Read in as hexadecimal
    #[arg(long)]
    from_hex: Option<String>,

    /// Display as hexadecimal
    #[arg(long)]
    to_hex: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
  let cli = Cli::parse();
  if cli.debug == true {
    std::env::set_var("RUST_LOG", "debug");
  }

  env_logger::init();

  // Get date
  let time;
  //let mut vec: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  //let mut x = core::str::from_utf8(&vec).expect("Invalid buffer length");

  if let Some(from_hex) = cli.from_hex.as_deref() {
    // Date from hexidecimal
    debug!("from_hex: {:#?}", from_hex);
    time = Tai64N::from_slice(&<[u8; 12]>::from_hex(from_hex).unwrap()).unwrap();
  } else {
    // Date from current system time
    time = Tai64N::now();
  }

  // Print date
  if cli.to_hex == true {
    println!("{}", hex::encode(time.to_bytes()));
  } else {
    if cli.local == true {
      let d: DateTime<Local> = time.to_system_time().into();
      println!("{}", d.to_rfc3339_opts(SecondsFormat::Nanos, true));
    } else {
      let d: DateTime<Utc> = time.to_system_time().into();
      println!("{}", d.to_rfc3339_opts(SecondsFormat::Nanos, true));
    }
  }
}
