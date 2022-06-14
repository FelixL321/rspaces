use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{create_template, Repository, Space, Template, Tuple};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum MessageType {
    Get,
    Getp,
    Getall,
    Query,
    Queryp,
    Queryall,
    Put,
}
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub action: MessageType,
    pub source: u32,
    pub tuple: Tuple,
    pub template: Template,
}

pub struct Gate {
    pub handle: Mutex<Sender<()>>,
    repo: Arc<Repository>,
    connections: Mutex<Vec<Sender<()>>>,
}

impl Gate {
    pub fn new_gate(addr: SocketAddr, repo: Arc<Repository>) -> std::io::Result<Arc<Gate>> {
        let (tx, rx) = mpsc::channel();
        match TcpListener::bind(addr) {
            Ok(listener) => {
                let gate = Arc::new(Gate {
                    handle: Mutex::new(tx),
                    repo,
                    connections: Mutex::new(Vec::new()),
                });
                let clone = Arc::clone(&gate);
                Gate::start(clone, listener, rx);
                Ok(gate)
            }
            Err(e) => Err(e),
        }
    }
    fn start(gate: Arc<Gate>, listener: TcpListener, rx: Receiver<()>) {
        thread::spawn(move || loop {
            listener
                .set_nonblocking(true)
                .expect("Cannot set nonblocking");
            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => {
                        let (tx, rx) = mpsc::channel();
                        let mut buffer = [0; 1024];
                        let space_string: String;
                        match s.read(&mut buffer) {
                            Ok(n) => {
                                let inc_string = String::from_utf8_lossy(&buffer[..n]);
                                space_string = inc_string.to_string();
                            }
                            Err(_) => todo!(),
                        }
                        let space = match gate.repo.get_space(space_string) {
                            Some(space) => {
                                s.write("ok".as_bytes()).unwrap();
                                space
                            }
                            None => todo!(),
                        };
                        let mut c = Connection {
                            signal: rx,
                            stream: s,
                            space: space,
                        };
                        let mut cons = gate.connections.lock().unwrap();
                        cons.push(tx);
                        thread::spawn(move || c.handle_connection());
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        match rx.recv_timeout(Duration::from_millis(10)) {
                            Ok(_) => break,
                            Err(_) => {
                                thread::sleep(Duration::from_millis(2000));
                                continue;
                            }
                        }
                    }
                    Err(e) => panic!("encountered IO error: {}", e),
                }
            }
            let mut cons = gate.connections.lock().unwrap();
            for con in cons.iter() {
                con.send(());
            }
        });
    }
}

struct Connection {
    signal: Receiver<()>,
    stream: TcpStream,
    space: Arc<Space>,
}

impl Connection {
    fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];
        self.stream
            .set_read_timeout(Some(Duration::from_millis(5000)))
            .expect("could not set timeout");
        loop {
            match self.signal.recv_timeout(Duration::from_millis(10)) {
                Ok(_) => break,
                Err(_) => {}
            }
            match self.stream.read(&mut buffer) {
                Ok(n) => {
                    let inc_string = String::from_utf8_lossy(&buffer[..n]);
                    let response = self.handle_message(inc_string.to_string());
                    let r_json = serde_json::to_string(&response).unwrap();
                    self.stream.write(r_json.as_bytes()).unwrap();
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => todo!(),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => todo!(),
                Err(e) => todo!(),
            }
        }
    }

    fn handle_message(&mut self, inc: String) -> Message {
        let message = match serde_json::from_str::<Message>(&inc) {
            Ok(m) => m,
            Err(e) => panic!("Malformed message, panicking with message: {}", e),
        };
        match message.action {
            MessageType::Get => self.handle_get(message),
            MessageType::Getp => todo!(),
            MessageType::Getall => todo!(),
            MessageType::Query => todo!(),
            MessageType::Queryp => todo!(),
            MessageType::Queryall => todo!(),
            MessageType::Put => todo!(),
        }
    }

    fn handle_get(&mut self, message: Message) -> Message {
        let tuple = self.space.get(&message.template);
        Message {
            action: MessageType::Get,
            source: 0,
            tuple,
            template: create_template!(),
        }
    }
}
