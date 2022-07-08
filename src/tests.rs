use std::{net::TcpStream, io::Write};

#[cfg(test)]
mod tests {
    #[test]
    fn strings() {
        let mut buf = [0; 255];

        let msg = String::from("up");

        let x = msg.as_bytes();

        for i in 0..msg.len() {
            buf[i] = x[i]
        }

        let h = String::from_utf8_lossy(&buf).to_string();

        let v = h.chars().collect::<Vec<char>>();

        let mut counter = 0;

        for i in 0..h.len() {
            if h.as_bytes()[i] != 0 {
                counter +=1
            }
        }

        println!("{}", counter);

        let mut msg = String::new();

        for i in 0..counter {
            msg.push(v[i])
        }

        match msg.as_str() {
            "up" => println!("up"),
            "down" => println!("down"),
            _ => println!("none"),
        }
    }
}