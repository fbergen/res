// use std::process::Command;
// use std::sync::mpsc::channel;

use psutil::process::Process;
use std::fmt;
use tokio::process::Command;
// use tokio::sync::mpsc::channel;
use clap::{App, AppSettings};
use std::{thread, time};
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;

#[derive(Debug)]
struct Res {
    timestamp: f32,
    cpu: f32,
    rss: u64,
    virt: u64,
}

const PREFIXES: [&str; 8] = ["", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"];

fn human_readable(num: u64) -> String {
    let mut n = num;
    for i in 0..PREFIXES.len() - 1 {
        if n < 1024 {
            return format!("{}{}", n, PREFIXES[i]);
        }
        n = n / 1024;
    }
    return format!("{}{}", n, PREFIXES[PREFIXES.len() - 1]);
}

impl fmt::Display for Res {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:.02} CPU% {:.02} RSS {} VIRT {} ",
            self.timestamp,
            self.cpu,
            human_readable(self.rss),
            human_readable(self.virt)
        )
    }
}

fn sleep(ms: u64) {
    thread::sleep(time::Duration::from_millis(ms));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = App::new("test")
        .setting(AppSettings::TrailingVarArg)
        .args_from_usage(
            "-v         'unused verbosity'
          <utility>...       'utility to run'",
        )
        .get_matches();

    let mut u = m.values_of("utility").unwrap();

    // Safe because "utility" is required by clap
    let mut cmd = Command::new(u.next().unwrap());
    for a in u {
        cmd.arg(a);
    }

    let child = cmd.spawn().expect("failed to spawn command");

    let (tx, mut rx) = oneshot::channel();

    let mut proc = Process::new(child.id()).expect("Count not acquire process");

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    let guard = tokio::task::spawn(async {
        let status = child.await.expect("child process encountered an error");
        tx.send(true).unwrap();
    });

    loop {
        match rx.try_recv() {
            Err(TryRecvError::Empty) => (),
            _ => break,
        }

        let percent_cpu = proc.cpu_percent().unwrap();
        let mem = proc.memory_info().unwrap();
        println!(
            "{}",
            Res {
                timestamp: 0.0,
                cpu: percent_cpu,
                rss: mem.rss(),
                virt: mem.vms()
            }
        );

        sleep(1000);
    }

    guard.await?;

    Ok(())
}
