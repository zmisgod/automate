use std::process::Command;
use std::{thread, time};
fn main() {
    loop {
       let output = Command::new("cmd").
       args(&["/C", "adb shell input swipe 300 700 300 1000"]).
       output().
       expect("cmd error");
       let outstr = String::from_utf8_lossy(&output.stdout);
       println!("{}", outstr);
       thread::sleep(time::Duration::from_millis(600));
    }
}
