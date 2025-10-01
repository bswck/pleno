use clap::Parser;

#[derive(Parser)]
/// A toy ELF linker for fun!
struct Cli {
    /// Object files
    objects: Vec<String>,
    #[arg(long, required = false, default_value = "a.out")]
    /// The output ELF file
    out_file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    println!("Linking to {}", args.out_file);

    for object in args.objects {
        println!("Linking file {}", object);
    }

    Ok(())
}
