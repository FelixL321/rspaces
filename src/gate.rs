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

use crate::{create_template, space::Space, Repository, Template, Tuple};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum MessageType {
    Get,
    Getp,
    Getall,
    Query,
    Queryp,
    Queryall,
    Put,
    Error,
    Ok,
}
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub action: MessageType,
    pub tuple: Vec<Tuple>,
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
                            Err(_) => {
                                s.write("f".as_bytes()).unwrap();
                                continue;
                            }
                        }
                        let space = match gate.repo.get_space(space_string) {
                            Some(space) => {
                                s.write("t".as_bytes()).unwrap();
                                space
                            }
                            None => {
                                s.write("f".as_bytes()).unwrap();
                                continue;
                            }
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
            let cons = gate.connections.lock().unwrap();
            for con in cons.iter() {
                con.send(()).expect("Couldnt shut down threadpool");
            }
        });
    }
}

struct Connection {
    signal: Receiver<()>,
    stream: TcpStream,
    space: Arc<dyn Space>,
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
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => panic!("{}", e),
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
            MessageType::Getp => self.handle_getp(message),
            MessageType::Getall => self.handle_getall(message),
            MessageType::Query => self.handle_query(message),
            MessageType::Queryp => self.handle_queryp(message),
            MessageType::Queryall => self.handle_queryall(message),
            MessageType::Put => self.handle_put(message),
            m => self.handle_echo(m),
        }
    }

    fn handle_get(&mut self, message: Message) -> Message {
        let mut tuple = Vec::new();
        tuple.push(self.space.get(message.template).unwrap());
        Message {
            action: MessageType::Ok,
            tuple,
            template: create_template!(),
        }
    }
    fn handle_getp(&mut self, message: Message) -> Message {
        let mut action = MessageType::Ok;
        let mut tuple = Vec::new();
        match self.space.getp(message.template) {
            Ok(t) => tuple.push(t),
            Err(_) => {
                action = MessageType::Error;
            }
        };
        Message {
            action,
            tuple,
            template: create_template!(),
        }
    }
    fn handle_query(&mut self, message: Message) -> Message {
        let mut tuple = Vec::new();
        tuple.push(self.space.query(message.template).unwrap());
        Message {
            action: MessageType::Ok,
            tuple,
            template: create_template!(),
        }
    }
    fn handle_queryp(&mut self, message: Message) -> Message {
        let mut action = MessageType::Ok;
        let mut tuple = Vec::new();
        match self.space.queryp(message.template) {
            Ok(t) => tuple.push(t),
            Err(_) => {
                action = MessageType::Error;
            }
        };
        Message {
            action,
            tuple,
            template: create_template!(),
        }
    }
    fn handle_getall(&mut self, message: Message) -> Message {
        let tuple = self.space.getall(message.template).unwrap();
        Message {
            action: MessageType::Ok,
            tuple,
            template: create_template!(),
        }
    }

    fn handle_echo(&self, action: MessageType) -> Message {
        Message {
            action,
            tuple: Vec::new(),
            template: create_template!(),
        }
    }

    fn handle_queryall(&mut self, message: Message) -> Message {
        let tuple = self.space.queryall(message.template).unwrap();
        Message {
            action: MessageType::Ok,
            tuple,
            template: create_template!(),
        }
    }

    fn handle_put(&mut self, message: Message) -> Message {
        self.space
            .put(message.tuple.get(0).unwrap().clone())
            .unwrap();
        Message {
            action: MessageType::Ok,
            tuple: Vec::new(),
            template: create_template!(),
        }
    }
}
