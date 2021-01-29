#![allow(dead_code)]
#![allow(unused_variables)]
use crate::cmd::run;

mod cmd;
mod parse;
mod section;

fn main() -> std::io::Result<()> {
    run()
}
