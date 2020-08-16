extern crate rusoto_core;
extern crate rusoto_ecr;
extern crate clap;

use clap::{Arg, App, AppSettings};
use tokio;

mod utils;

#[tokio::main]
async fn main() {
    let matches = App::new("GetECR")
        .version("1.0.1")
        .author("Colin MacRae <me@cmac4603.dev>")
        .about("Fetches a single AWS ECR image & corresponding sha.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("repo")
            .required(true)
            .index(1)
            .help("Name of the ECR image"))
        .arg(Arg::with_name("tag")
            .required(true)
            .index(2)
            .help("Image tag to search for"))
        .get_matches();

    let image_output: utils::writer::CliOutput = utils::get_tag(
        matches.value_of("repo").unwrap(),
        matches.value_of("tag").unwrap()
    ).await;

    utils::writer::write(image_output);
}
