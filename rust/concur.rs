use std::thread::spawn;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel::<isize>();
    let (strtx, strrx) =  channel();

    spawn(move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();
        strtx.send(format!("{} + {} + {} = {}" , x , y , z , x + y + z)).unwrap();
    });

    let my_in = tx.clone();
    spawn(move || { my_in.send(2 * 10).unwrap() });
    let my_in = tx.clone();
    spawn(move || { my_in.send(2 * 20).unwrap() });
    let my_in = tx.clone();
    spawn(move || { my_in.send(30 + 40).unwrap() });
    match strrx.recv() {
        Ok(v)  => println!("{}", v),
        Err(e) => println!("Error receiving {:?}", e),
    }
}
