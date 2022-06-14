use std::{
    net::{SocketAddr, TcpListener},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::{Repository, Space, Template, Tuple};

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
    pub target: String,
    pub tuple: Tuple,
    pub template: Template,
}

pub struct Gate {
    handle: Mutex<Sender<()>>,
    repo: Arc<Repository>,
}

impl Gate {
    pub fn new_gate(addr: SocketAddr, repo: Arc<Repository>) -> std::io::Result<Arc<Gate>> {
        let (tx, rx) = mpsc::channel();
        match TcpListener::bind(addr) {
            Ok(listener) => {
                let gate = Arc::new(Gate {
                    handle: Mutex::new(tx),
                    repo,
                });
                let clone = Arc::clone(&gate);
                (*clone).start(listener, rx);
                Ok(gate)
            }
            Err(e) => Err(e),
        }
    }
    fn start(&self, listener: TcpListener, rx: Receiver<()>) {
        thread::spawn(move || loop {
            rx.recv_timeout(Duration::from_millis(10));
            let l = listener.accept().unwrap();
        });
    }
}
