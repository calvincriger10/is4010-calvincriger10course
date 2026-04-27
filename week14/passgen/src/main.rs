use clap::{Parser, Subcommand};
use passgen::{
    calculate_entropy, check_common_patterns, generate_passphrase, generate_pin, generate_random,
    validate_strength,
};

#[derive(Parser)]
#[command(name = "passgen")]
#[command(about = "A secure password generator and validator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Random {
        #[arg(short, long, default_value_t = 16)]
        length: usize,

        #[arg(short, long)]
        symbols: bool,
    },

    Passphrase {
        #[arg(short, long, default_value_t = 4)]
        words: usize,

        #[arg(short, long, default_value = "-")]
        separator: char,
    },

    Pin {
        #[arg(short, long, default_value_t = 4)]
        length: usize,
    },

    Validate {
        password: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random { length, symbols } => {
            println!("Generated password: {}", generate_random(length, symbols));
        }
        Commands::Passphrase { words, separator } => {
            println!(
                "Generated passphrase: {}",
                generate_passphrase(words, separator)
            );
        }
        Commands::Pin { length } => {
            println!("Generated PIN: {}", generate_pin(length));
        }
        Commands::Validate { password } => {
            println!("Password strength: {:?}", validate_strength(&password));
            println!("Common pattern: {}", check_common_patterns(&password));
            println!("Entropy: {:.2}", calculate_entropy(&password));
        }
    }
}
