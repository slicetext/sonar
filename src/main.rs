use std::{fs::File, io::{self, BufReader}, panic::{self, AssertUnwindSafe}, time::Duration};

use clap::Parser;
use rodio::{Decoder, OutputStream, source::Source, Sink};

fn seconds_to_minute_seconds(seconds: u64)->String{
    let minutes=((seconds/60) as f32).round();
    let seconds=seconds as f32 -(minutes*60.0);
    return format!("{minutes}:{seconds}");
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short,long)]
    file: String,
}
fn main() {
    let help = 
        "
        This is a command line music player. Run with the --file=\"filename\" parameter to play a file.
        While playing, use commands to control the player. Command list:
        - q: quit
        - h: print this help
        - i: info
        - p: play/pause
        ";
    let args= Args::parse();
    println!("Playing {}",args.file);
    let filename=args.file.clone();
    let (_stream, stream_handle) = OutputStream::try_default()
        .unwrap();
    let file = BufReader::new(File::open(args.file)
        .unwrap()
    );
    let sink = Sink::try_new(&stream_handle)
        .unwrap();
    let source = Decoder::new(file).unwrap();
    let result = panic::catch_unwind(AssertUnwindSafe(|| source.total_duration().unwrap()));
    let length = match  result{
        Ok(d)  => d,
        Err(_) => Duration::ZERO,
    };

    sink.append(source);

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command
            .trim();
        match command.as_ref() {
            "q" => break,
            "h" => println!("{help}"),
            "i" => {
                println!("Playing {}, {} long audio file",filename,seconds_to_minute_seconds(length.as_secs()));
            },
            "p" => {
                if sink.is_paused() {
                    sink.play();
                } else {
                    sink.pause();
                }
            }
             _   => continue,
        }
        if sink.empty()  {
            println!("Finished playing.");
            return;
        }
    }
}
