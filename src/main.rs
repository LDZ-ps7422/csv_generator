use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn main() -> std::io::Result<()> {
    let file_num: u32 = 5;
    let file_size: u32 = 100000;

    let mut handles = vec![];

    for idx in 1..=file_num {

        let handle = thread::spawn(move || {

            let mut file = File::create(format!("./target/data{}.csv", idx))
                .expect("ERROR: fail to create a file!");

            for line_i in 1..file_size {
                file.write_all(format!("{:0>6},AAA,100\n", line_i).as_bytes())
                    .expect("ERROR: fail to write data!");
            }
            file.write_all(format!("{:0>6},AAA,100", file_size).as_bytes())
                .expect("ERROR: fail to write data!");
    
            
            println!("Data has been written to data{}.csv 🚀", idx);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Error joining thread");
    }

    Ok(())

}