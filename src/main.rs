use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_num: u32 = 10;
    let file_size: u32 = 1000000;

    for idx in 1..=file_num {
        let mut file = File::create(format!("./target/data{}.csv", idx))?;

        for line_i in 1..file_size {
            file.write_all(format!("{:0>6},AAA,100\n", line_i).as_bytes())?;
        }
        file.write_all(format!("{:0>6},AAA,100", file_size).as_bytes())?;

        
        println!("Data has been written to data{}.csv 🚀", idx);
    }

    Ok(())

}