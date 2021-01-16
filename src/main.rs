use clap::{crate_version, Clap};
use lazy_static::lazy_static;
use notify_rust::Notification;
use std::{
    collections::HashMap,
    io::{self, Write},
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use warp::{http::Response, Filter};

#[derive(Clap)]
#[clap(version = crate_version!())]
struct Opts {
    #[clap(short, long, default_value = "4829")]
    port: u16,
}

#[allow(dead_code)]
enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

use LogLevel::*;

lazy_static! {
    static ref STDERR: BufferWriter = BufferWriter::stderr(ColorChoice::Always);
    static ref STDOUT: BufferWriter = BufferWriter::stdout(ColorChoice::Always);
}

fn write_log(level: LogLevel, message: String) -> io::Result<()> {
    let mut buffer = STDERR.buffer();
    write!(&mut buffer, "[")?;
    buffer.set_color(ColorSpec::new().set_fg(Some(match level {
        DEBUG => Color::White,
        INFO => Color::Blue,
        WARN => Color::Yellow,
        ERROR => Color::Red,
    })))?;
    write!(
        &mut buffer,
        "{}",
        match level {
            DEBUG => "DEBUG",
            INFO => "INFO",
            WARN => "WARN",
            ERROR => "ERROR",
        }
    )?;
    buffer.reset()?;
    write!(&mut buffer, "] ")?;
    writeln!(&mut buffer, "{}", message)?;
    STDERR.print(&buffer)?;
    Ok(())
}

fn log(level: LogLevel, message: String) {
    write_log(level, message).unwrap();
}

fn send_message(msg: &String) {
    Notification::new()
        .summary("I Say")
        .body(msg)
        .show()
        .unwrap();
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    let port = opts.port;
    log(INFO, format!("Starting web server at :{}", port));

    let hello = warp::any()
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("q") {
            Some(s) => {
                send_message(s);
                Response::builder().body("good")
            }
            None => Response::builder().status(400).body("bad"),
        });

    warp::serve(hello).run(([0, 0, 0, 0], port)).await;
}
