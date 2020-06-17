/*
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
*/
use std::io;

mod lib;
use lib::*;

/*
const THREAD_AMOUNT: u32 = 20;

fn worker<'a>(tx: &SyncSender<bool>, value: u32, v: &mut Vec<thread::JoinHandle<()>>) {
    let mut thread_number = 0;
    for _i in 1..=THREAD_AMOUNT {
        let tx = tx.clone();
        v.push(thread::spawn(move || loop {
            thread_number += 1;
            tx.send(is_prime(
                &value / (THREAD_AMOUNT - thread_number + 1),
                (&value / (THREAD_AMOUNT - thread_number + 1)) - (&value / THREAD_AMOUNT + 1),
            ))
            .unwrap_or_else(|x| println!("THREAD {} ERROR:{}", thread_number, x));
        }));
    }
}
fn producer(value: u32) -> Receiver<bool> {
    let (tx, rx): (SyncSender<bool>, Receiver<bool>) = sync_channel(1024);
    let mut v: Vec<thread::JoinHandle<()>> = vec![];
    worker(&tx, value, &mut v);
    rx
}


fn main() {
    let rx = producer(25676);
    loop {
        match rx.try_recv() {
            Ok(o) => println!("{}", o),
            Err(_) => {}
        }
    }
}
*/

fn main() -> io::Result<()> {
    println!("Enter a number:");
    let mut num = String::new();
    io::stdin().read_line(&mut num)?;
    let num: u128 = match num.trim().parse() {
        Ok(o) => o,
        Err(e) => panic!("Enter a number! ERROR: {}", e),
    };
    match is_prime(num, 2){
        true =>    println!("{} is prime", num),
        false =>    println!("{} is not prime", num),
    }

    Ok(())
}
