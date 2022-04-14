use bevy::prelude::*;

use dotfiles::argparse;

#[derive(Debug, Default)]
struct Args {
    window_descriptor: WindowDescriptor,
}

#[derive(Debug)]
enum Error {
    Argparse(argparse::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl From<argparse::Error> for Error {
    fn from(e: argparse::Error) -> Self {
        Self::Argparse(e)
    }
}

fn parse_args(args: &[String]) -> Result<Args> {
    let mut parsed_args = Args::default();
    let mut parser = argparse::Parser::new(&mut parsed_args);

    parser
        .add_argument(
            argparse::Argument::default()
                .with_identifiers(&["-W", "--width"])
                .with_callback_and_args(1, move |args: &mut Args, parameters| {
                    args.window_descriptor.width = parameters[0].parse().unwrap();
                }),
        )?
        .add_argument(
            argparse::Argument::default()
                .with_identifiers(&["-H", "--height"])
                .with_callback_and_args(1, move |args: &mut Args, parameters| {
                    args.window_descriptor.height = parameters[0].parse().unwrap();
                }),
        )?;

    parser.parse(args)?;

    Ok(parsed_args)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = parse_args(&args[1..]).unwrap();

    App::new()
        .insert_resource(parsed_args.window_descriptor)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .run();
}
