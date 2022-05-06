#![feature(thread_id_value)]

use std::io::Write;
use humantime;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn do_work() {
    log::debug!("do_work:+");

    let sender_thread;
    let receiver_thread;
    {
        log::debug!("do_work: inner-scope-");

        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        // Send between functions
        tx.send(1).unwrap();

        // Send from a thread
        let tx1 = tx.clone();
        sender_thread = std::thread::spawn(move || {
            log::debug!("Sender thread:+");
            for i in [100, 101, 102] {
                tx1.send(i).unwrap();
            }
            log::debug!("Sender thread:-");
        });

        receiver_thread = std::thread::spawn(move || {
            log::debug!("Reciever thread:+");
            loop {
                match rx.recv() {
                    Ok(v) => log::debug!("v={:?}", v),
                    Err(e) => {
                        log::debug!("err={:?}", e);
                        break;
                    }
                }
            }
            log::debug!("Reciever thread:-");
        });

        // No guarantees, but this will probably be the second item received
        tx.send(i32::MAX).unwrap();

        log::debug!("do_work: send(132::MAX)-;inner-scope-");
    }

    // Wait for the threads to finish
    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();


    log::debug!("do_work:-");
}

fn env_logger_init(default_level: &str) {
    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env.default_filter_or(default_level)).format(|buf, record| {
        let time = std::time::SystemTime::now();
        writeln!(buf, "[{} {:5} {} {:>4} {:2}] {}",
            humantime::format_rfc3339_nanos(time),
            record.level(),
            if let Some(s) = record.module_path_static() { s } else { "" },
            if let Some(v) = record.line() { v } else { 0 },
            std::thread::current().id().as_u64(),
            record.args())
    }).init();
}

fn main() {
    env_logger_init("info");
    let start = std::time::SystemTime::now();
    log::info!("main:+");

    do_work();

    log::info!("main:- {}ns", start.elapsed().unwrap().as_nanos());
}
