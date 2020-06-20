use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

pub const THREAD_AMOUNT: usize = 20;

fn is_prime_i(num: usize, first: usize, last: usize) -> bool {
    (first..last).into_iter().all(|i| num % i != 0)
}
pub fn worker<'a>(
    tx: &SyncSender<bool>,
    value: usize,
    v: &mut Vec<thread::JoinHandle<()>>,
    i: usize,
) {
    let tx = tx.clone();
    v.push(thread::spawn(move || loop {
        let temp = is_prime_i(
            value,
            (&value / (THREAD_AMOUNT - i + 1)) - (&value / THREAD_AMOUNT + 1),
            &value / (THREAD_AMOUNT - i + 1),
        );
        if temp == false {
            tx.send(temp)
                .unwrap_or_else(|x| eprintln!("THREAD {} ERROR:{}", i, x));
        } else if i == THREAD_AMOUNT {
            tx.send(temp)
                .unwrap_or_else(|x| eprintln!("THREAD {} ERROR:{}", i, x));
        }
    }));
}
pub fn producer(value: usize) -> Receiver<bool> {
    let (tx, rx): (SyncSender<bool>, Receiver<bool>) = sync_channel(1024);
    let mut v: Vec<thread::JoinHandle<()>> = vec![];
    for i in 1..=THREAD_AMOUNT {
        worker(&tx, value, &mut v, i);
    }
    rx
}
pub fn is_prime(value: usize) -> bool {
    let rx = producer(value);
    match rx.recv() {
        Ok(o) => o,
        Err(e) => panic!(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_true() {
        assert!(is_prime(2));
    }
    #[test]
    fn test_prime_false() {
        assert!(!is_prime(24));
    }
    #[test]
    #[ignore]
    fn expensive_prime_true() {
        assert!(is_prime(546754754801 * 546754754771))
    }
}
