use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{new_template, space::Space, Repository, Template, Tuple};

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
    children: Mutex<Vec<JoinHandle<()>>>,
    pub join: Mutex<Option<JoinHandle<()>>>,
}

impl Gate {
    pub(crate) fn new_gate(addr: String, repo: Arc<Repository>) -> std::io::Result<Arc<Gate>> {
        let (tx, rx) = mpsc::channel();
        match TcpListener::bind(addr) {
            Ok(listener) => {
                let gate = Arc::new(Gate {
                    handle: Mutex::new(tx),
                    repo,
                    connections: Mutex::new(Vec::new()),
                    children: Mutex::new(Vec::new()),
                    join: Mutex::new(None),
                });
                let clone = Arc::clone(&gate);
                Gate::start(clone, listener, rx);
                Ok(gate)
            }
            Err(e) => Err(e),
        }
    }
    fn start(gate: Arc<Gate>, listener: TcpListener, rx: Receiver<()>) {
        let gateclone = Arc::clone(&gate);
        let handle = thread::spawn(move || {
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
                                s.write_all("f".as_bytes()).ok();
                                continue;
                            }
                        }
                        let space = match gate.repo.get_space(space_string) {
                            Some(space) => {
                                s.write_all("t".as_bytes()).ok();
                                space
                            }
                            None => {
                                s.write_all("f".as_bytes()).ok();
                                continue;
                            }
                        };
                        let mut c = Connection {
                            signal: rx,
                            stream: s,
                            space,
                        };
                        let mut cons = gate.connections.lock().unwrap();
                        cons.push(tx);
                        thread::spawn(move || c.handle_connection());
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        match rx.recv_timeout(Duration::from_millis(10)) {
                            Ok(_) => {
                                break;
                            }
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
                let _ = con.send(());
            }
            let mut children = gate.children.lock().unwrap();
            let n = children.len();
            for handle in children.drain(0..n) {
                handle.join().unwrap();
            }
        });
        let mut join = gateclone.join.lock().unwrap();
        let _ = join.insert(handle);
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
            match self.stream.read(&mut buffer[..]) {
                Ok(n) => {
                    let inc_string = String::from_utf8_lossy(&buffer[..n]);
                    if inc_string.len() < 1 {
                        break;
                    }
                    let response = self.handle_message(inc_string.to_string());
                    let r_json = serde_json::to_string(&response).expect("Serialization error");
                    self.stream.flush().expect("Could not flush stream");
                    self.stream
                        .write_all(r_json.as_bytes())
                        .expect("Connection error");
                    self.stream.flush().expect("Could not flush stream");
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_e) => break,
            }
        }
    }

    fn handle_message(&mut self, inc: String) -> Message {
        let message = match serde_json::from_str::<Message>(&inc) {
            Ok(m) => m,
            Err(e) => panic!(
                "Malformed message, panicking with message: {}, Message was: ({})",
                e, inc
            ),
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
            template: new_template!(),
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
            template: new_template!(),
        }
    }
    fn handle_query(&mut self, message: Message) -> Message {
        let mut tuple = Vec::new();
        tuple.push(self.space.query(message.template).unwrap());
        Message {
            action: MessageType::Ok,
            tuple,
            template: new_template!(),
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
            template: new_template!(),
        }
    }
    fn handle_getall(&mut self, message: Message) -> Message {
        let tuple = self.space.getall(message.template).unwrap();
        Message {
            action: MessageType::Ok,
            tuple,
            template: new_template!(),
        }
    }

    fn handle_echo(&self, action: MessageType) -> Message {
        Message {
            action,
            tuple: Vec::new(),
            template: new_template!(),
        }
    }

    fn handle_queryall(&mut self, message: Message) -> Message {
        let tuple = self.space.queryall(message.template).unwrap();
        Message {
            action: MessageType::Ok,
            tuple,
            template: new_template!(),
        }
    }

    fn handle_put(&mut self, message: Message) -> Message {
        self.space
            .put(message.tuple.get(0).unwrap().clone())
            .unwrap();
        Message {
            action: MessageType::Ok,
            tuple: Vec::new(),
            template: new_template!(),
        }
    }
}
