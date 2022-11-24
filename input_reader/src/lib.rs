use dotenv::dotenv;
use error_chain::error_chain;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(day: String) -> io::Result<Vec<String>>
//io::Result<io::Lines<io::BufReader<File>>>
{
    let path = format!("inputs/{day}.txt");
    let file = File::open(path)?;
    let mut output_lines: Vec<String> = Vec::new();
    let lines = io::BufReader::new(file).lines();

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        match line {
            Ok(line) => {
                output_lines.push(line);
            }
            _ => (),
        }
    }
    Ok(output_lines)
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn url_reader(year: i32, day: i32, filename: &String, overwrite: bool) -> Result<()> {
    let path = format!("inputs/{filename}.txt");
    let file_exists = Path::new(&path).exists();

    if file_exists && !overwrite {
        return Ok(());
    }

    println!("Getting file from website");

    dotenv().ok();
    let session = dotenv::var("SESSION").unwrap();

    let client = reqwest::blocking::Client::new();
    let mut res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("Cookie", format!("session={session}"))
        .send()?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    save_file(&body, &path);

    Ok(())
}

fn save_file(body: &str, path: &String) {
    let path = Path::new(path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(body.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
