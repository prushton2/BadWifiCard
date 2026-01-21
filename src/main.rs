use std::process::exit;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum State {
    Starting,
    ConnectionNotFound(u32),
    Connected,
    LostConnection(u32),
}

fn main() {
    let mut state = State::Starting;
    loop {
        println!("{:?}", state);
        match state {
            State::Starting => {
                if check_internet() {
                    state = State::Connected
                } else {
                    state = State::ConnectionNotFound(1);
                }
            },
            State::ConnectionNotFound(d) => {
                if d >= 4 {
                    reboot()
                }
                match check_internet() {
                    true => state = State::Connected,
                    false => state = State::ConnectionNotFound(d + 1)
                }
            }
            State::Connected => {
                if !check_internet() {
                    state = State::LostConnection(1)
                }
            },
            State::LostConnection(d) => {
                if d >= 2 {
                    reboot();
                }
                if !check_internet() {
                    state = State::LostConnection(d + 1)
                }
            }
        }
        std::thread::sleep(Duration::from_secs(5));
    }
}

fn reboot() {
    println!("Rebooting");
    exit(0);
}

fn check_internet() -> bool {
    let client = reqwest::blocking::Client::builder()
        .user_agent("aaaa")
        .build()
        .unwrap();

    let urls: [&str; 4] = ["https://google.com", "https://gnu.org", "https://kernel.org", "https://cloudflare.com"];

    for url in urls {
        let res = client.head(url).send();
        
        match res {
            Err(d) if d.is_connect() => {},
            _ => {
                return true
            }
        }
    }

    return false
}