use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::thread;

fn main() -> std::io::Result<()> {
    let file_num: u32 = 20;
    let file_size: u32 = 100000000;

    let mut handles = vec![];

    for idx in 1..=file_num {
        let handle = thread::spawn(move || {
            let file = File::create(format!("./target/data{}.csv", idx))
                .expect("ERROR: fail to create a file!");

            let mut writer = BufWriter::new(file);

            for line_i in 1..=file_size {
                if line_i == file_size {
                    writeln!(writer, "{:0>8},AAA,100", line_i)
                        .expect("ERROR: fail to write data!");
                } else {
                    writeln!(writer, "{:0>8},AAA,100", line_i)
                        .expect("ERROR: fail to write data!");
                }
            }

            writer.flush().expect("ERROR: fail to flush buffer!");

            println!("Data has been written to data{}.csv 🚀", idx);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Error joining thread");
    }

    Ok(())
}