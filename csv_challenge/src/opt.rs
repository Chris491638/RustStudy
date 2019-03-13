use structopt_derive::*;

#[derive(Debug, StructOpt)]
// name: The binary name displayed in help messages. Defaults to the crate name given by Cargo.
// version: Defaults to the crate version given by Cargo.
// author: Defaults to the crate author name given by Cargo.
// about: Defaults to the crate description given by Cargo.
#[structopt(name = "csv_challenge", about = "Usage")]
pub struct Opt {
    #[structopt(help = "Input file")]
    pub input: String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Replacement Column Name")]
    pub replacement: String,
   #[structopt(help = "Output file, stdout if not present")]
    pub output: Option<String>,
}