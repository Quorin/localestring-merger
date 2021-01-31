use crate::cmd::run;

mod cmd;
mod convert;
mod find;
mod parse;
mod section;

fn main() -> std::io::Result<()> {
    run()
}
