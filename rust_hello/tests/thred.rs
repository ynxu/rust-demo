use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn test_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

#[test]
fn test_thread_move(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move||{
        println!("vec {:?}",v);
    });
    handle.join().unwrap();
}

#[test]
fn test_thread_channel(){
    let (tx,rx) = mpsc::channel();

    thread::spawn(move||{
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("got : {}",received);
}