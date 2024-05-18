#![allow(unused_variables)]
#![allow(dead_code)]
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    path: String,
    data: Vec<u8>,
    state: FileState,
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, ({})>", self.path, self.state)
    }
}

impl File {
    fn new(path: &str) -> File {
        File {
            path: String::from(path),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(path: &str, data: &Vec<u8>) -> File {
        File {
            path: String::from(path),
            data: data.clone(),
            state: FileState::Closed,
        }
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let f1 = File {
        path: String::from("../alice_in_wonderland.txt"),
        data: Vec::new(),
        state: FileState::Closed,
    };
    println!("{:?}", f1);

    let f2 = File {
        path: String::from("../test.txt"),
        data: vec![114, 117, 115, 116, 33],
        state: FileState::Closed,
    };
    let mut buffer: Vec<u8> = vec![];
    //open(&mut f2);
    let f2_len = read(&f2, &mut buffer);
    //close(&mut f2);

    let f2_text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f2);
    println!("{}", f2_text);

    let mut f3 = File::new_with_data("../test_2.txt", &vec![114, 117, 115, 116, 33]);
    f3 = open(f3).unwrap();
    let f4_len = f3.read(&mut buffer).unwrap();
    f3 = close(f3).unwrap();

    println!("{:?}", f3);
}
