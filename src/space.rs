use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::sync::MutexGuard;

use rand::thread_rng;
use rand::Rng;

use crate::drain_filter::drain_filter;
use crate::new_template;
use crate::Message;
use crate::MessageType;
use crate::Template;
use crate::Tuple;

pub trait Space: Send + Sync {
    /// Finds a tuple matching the template in the space, removes it from the space and returns it.
    ///
    /// Will block the current thread until a tuple is found
    ///
    /// # Error
    /// Errors will only occur when used on a remotespace
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    ///
    /// // Create a query template for the tuple with a 5 followed by a char
    /// let template = new_template!(5.actual(), 'b'.formal());
    ///
    /// //Query the space for the tuple
    /// let tuple = space.get(template).unwrap();
    ///
    /// assert_eq!(5, *tuple.get_field::<i32>(0));
    /// assert_eq!('a', *tuple.get_field::<char>(1));
    ///
    /// ```
    fn get(&self, template: Template) -> std::io::Result<Tuple>;

    /// Tries to get a matching tuple from the space by removing it without blocking
    ///
    /// # Errors
    /// This will return an error if no tuple is found, but also if an io error occurs in a remotespace
    ///
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    ///
    /// // Create a query template for the tuple with a 5 followed by a char
    /// let template = new_template!(5.actual(), 'b'.formal());
    ///
    /// //Query the space for the tuple
    /// if let Ok(tuple) = space.getp(template) {
    /// assert_eq!(5, *tuple.get_field::<i32>(0));
    /// assert_eq!('a', *tuple.get_field::<char>(1));
    /// } else {
    /// assert!(false);
    /// }
    ///
    /// ```
    ///
    fn getp(&self, template: Template) -> std::io::Result<Tuple>;

    /// Puts the given tuple into the tuple space
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    /// ```
    fn put(&self, tuple: Tuple) -> Result<(), std::io::Error>;

    /// Finds a tuple matching the template in the space, and returns it without removing it.
    ///
    /// This does not blcok the current thread and therefore returns an option, as theres no garantuee for finding a tuple
    ///
    /// # Errors
    /// This will return an error if no tuple is found, but also if an io error occurs in a remotespace
    ///
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    ///
    /// // Create a query template for the tuple with a 5 followed by a char
    /// let template = new_template!(5.actual(), 'b'.formal());
    ///
    /// //Query the space for the tuple
    /// if let Ok(tuple) = space.queryp(template) {
    /// assert_eq!(5, *tuple.get_field::<i32>(0));
    /// assert_eq!('a', *tuple.get_field::<char>(1));
    /// } else {
    /// assert!(false);
    /// }
    ///
    /// ```
    ///
    fn queryp(&self, query: Template) -> std::io::Result<Tuple>;

    /// Finds a tuple matching the template in the space, and returns it without removing it.
    ///
    /// Will block the current thread until a tuple is found
    ///
    /// # Error
    /// Errors will only occur when used on a remotespace
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    ///
    /// // Create a query template for the tuple with a 5 followed by a char
    /// let template = new_template!(5.actual(), 'b'.formal());
    ///
    /// //Query the space for the tuple
    /// let tuple = space.query(template).unwrap();
    ///
    /// assert_eq!(5, *tuple.get_field::<i32>(0));
    /// assert_eq!('a', *tuple.get_field::<char>(1));
    ///
    ///
    /// ```
    fn query(&self, template: Template) -> std::io::Result<Tuple>;
    /// Gets all tuples in the space matching the template and removes them from the space
    ///
    /// # Error
    /// Errors will only occur when used on a remotespace
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    /// space.put(new_tuple!(4, 'b'));
    /// space.put(new_tuple!(4, 'c'));
    ///
    /// // Create a query template for the tuple with a 5 followed by a char
    /// let template = new_template!(4.actual(), 'b'.formal());
    ///
    /// //Query the space for all tuples matching
    /// let tuples = space.getall(template).unwrap();
    ///
    /// assert_eq!(2, tuples.len());
    /// for tuple in tuples.iter(){
    ///     assert_eq!(4, *tuple.get_field::<i32>(0));
    /// }
    ///
    /// ```
    fn getall(&self, template: Template) -> std::io::Result<Vec<Tuple>>;
    /// Gets all tuples in the space matching the template without removing them
    /// # Error
    /// Errors will only occur when used on a remotespace
    ///
    /// # Example
    /// ```
    /// # use rspaces::*;
    /// # let space = LocalSpace::new_sequential();
    /// //Put the tuple (5, 'a') in the space
    /// space.put(new_tuple!(5, 'a'));
    /// space.put(new_tuple!(4, 'b'));
    /// space.put(new_tuple!(4, 'c'));
    ///
    /// // Create a query template for the tuple with a 5 followed by a char
    /// let template = new_template!(4.actual(), 'b'.formal());
    ///
    /// //Query the space for all tuples matching
    /// let tuples = space.queryall(template).unwrap();
    ///
    /// assert_eq!(2, tuples.len());
    /// for tuple in tuples.iter(){
    ///     assert_eq!(4, *tuple.get_field::<i32>(0));
    /// }
    ///
    /// ```
    fn queryall(&self, template: Template) -> std::io::Result<Vec<Tuple>>;
}

enum SpaceType {
    Sequential,
    Queue,
    Stack,
    Pile,
    Random,
}

/// A tuple space for storing tuples and retrieving tuples
///
/// # Example
///
/// ```
/// # use rspaces::*;
/// # let space = LocalSpace::new_sequential();
/// //Put the tuple (5, 'a') in the space
/// space.put(new_tuple!(5, 'a'));
///
/// // Create a query template for the tuple with a 5 followed by a char
/// let template = new_template!(5.actual(), 'b'.formal());
///
/// //Query the space for the tuple
/// let tuple = space.get(template).unwrap();
///
/// assert_eq!(5, *tuple.get_field::<i32>(0));
/// assert_eq!('a', *tuple.get_field::<char>(1));
/// ```
pub struct LocalSpace {
    v: Mutex<Vec<Tuple>>,
    listeners: Mutex<Vec<Sender<()>>>,
    spacetype: SpaceType,
}

//Constructors
impl LocalSpace {
    /**
    Create a new sequential space

    get and query will return the oldest tuple matching the template

    */
    pub fn new_sequential() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Sequential,
        }
    }
    /**
    Create a new queue space

    get and query will return the oldest tuple if it matches the template

    */
    pub fn new_queue() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Queue,
        }
    }
    /**
    Create a new stack space

    get and query will return the newest tuple if it matcehs the template

    */
    pub fn new_stack() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Stack,
        }
    }
    /**
    Create a new sequential space

    get and query will return the newest tuple matching the template

    */
    pub fn new_pile() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Pile,
        }
    }
    /**
    Create a new sequential space

    get and query will return a random tuple matching the template

    */
    pub fn new_random() -> LocalSpace {
        LocalSpace {
            v: Mutex::new(Vec::new()),
            listeners: Mutex::new(Vec::new()),
            spacetype: SpaceType::Random,
        }
    }

    fn look(
        &self,
        query: Template,
        destroy: bool,
        v: &mut MutexGuard<Vec<Tuple>>,
    ) -> std::io::Result<Tuple> {
        let index: usize;
        match self.spacetype {
            SpaceType::Sequential => {
                if let Some(i) = v.iter().position(|t| query.query(t)) {
                    index = i;
                } else {
                    return Err(Error::from(std::io::ErrorKind::NotFound));
                }
            }
            SpaceType::Queue => {
                if v.len() > 0 && query.query(v.get(0).unwrap()) {
                    index = 0;
                } else {
                    return Err(Error::from(std::io::ErrorKind::NotFound));
                }
            }
            SpaceType::Pile => {
                if let Some(i) = v.iter().rev().position(|t| query.query(t)) {
                    index = v.len() - i - 1;
                } else {
                    return Err(Error::from(std::io::ErrorKind::NotFound));
                }
            }
            SpaceType::Stack => {
                if v.len() > 0 && query.query(v.get(v.len() - 1).unwrap()) {
                    index = v.len() - 1;
                } else {
                    return Err(Error::from(std::io::ErrorKind::NotFound));
                }
            }
            SpaceType::Random => {
                let matches = self.queryall(query)?;
                if matches.len() == 0 {
                    return Err(Error::from(std::io::ErrorKind::NotFound));
                }
                let mut rng = thread_rng();
                index = rng.gen_range(0..matches.len());
            }
        }
        match destroy {
            true => Ok(v.remove(index)),
            false => {
                let ret = (*v.get(index).unwrap()).clone();
                Ok(ret)
            }
        }
    }
}

impl Space for LocalSpace {
    fn get(&self, template: Template) -> std::io::Result<Tuple> {
        loop {
            let (tx, rx) = mpsc::channel();
            {
                let mut v = self.v.lock().unwrap();
                match self.look(template.clone(), true, &mut v) {
                    Ok(t) => return Ok(t),
                    Err(_) => {
                        let mut l = self.listeners.lock().unwrap();
                        l.push(tx);
                    }
                };
            }
            let _ = rx.recv();
        }
    }

    fn getp(&self, template: Template) -> std::io::Result<Tuple> {
        let mut v = self.v.lock().unwrap();
        self.look(template, true, &mut v)
    }

    fn put(&self, tuple: Tuple) -> Result<(), std::io::Error> {
        let mut v = self.v.lock().unwrap();
        v.push(tuple);
        let mut l = self.listeners.lock().unwrap();
        let mut remove = Vec::new();
        for i in 0..l.len() {
            let tx = l.get(i).unwrap();
            match tx.send(()) {
                Err(e) => panic!("panic: {:?}", e),
                Ok(_) => remove.push(i),
            }
        }
        for i in remove.iter() {
            l.remove(*i);
        }
        Ok(())
    }

    fn queryp(&self, template: Template) -> std::io::Result<Tuple> {
        let mut v = self.v.lock().unwrap();
        self.look(template, false, &mut v)
    }

    fn query(&self, template: Template) -> std::io::Result<Tuple> {
        loop {
            let (tx, rx) = mpsc::channel();
            {
                let mut v = self.v.lock().unwrap();
                match self.look(template.clone(), false, &mut v) {
                    Ok(t) => return Ok(t),
                    Err(_) => {
                        let mut l = self.listeners.lock().unwrap();
                        l.push(tx);
                    }
                };
            }
            let _ = rx.recv();
        }
    }

    fn getall(&self, template: Template) -> std::io::Result<Vec<Tuple>> {
        let mut v = self.v.lock().unwrap();
        Ok(drain_filter(&mut v, |t| template.query(t)).collect::<Vec<_>>())
    }

    fn queryall(&self, template: Template) -> std::io::Result<Vec<Tuple>> {
        let v = self.v.lock().unwrap();
        let viter = v.iter().filter(|t| template.query(t));
        let mut res = Vec::new();
        for t in viter {
            res.push(t.clone());
        }
        Ok(res)
    }
}

pub struct RemoteSpace {
    stream: Mutex<TcpStream>,
}

impl RemoteSpace {
    pub fn new(mut conn: String) -> std::io::Result<RemoteSpace> {
        let ip_offset = match conn.find('/') {
            Some(c) => c,
            None => return Err(Error::from(std::io::ErrorKind::InvalidInput)),
        };
        let ip_string: String = conn.drain(0..ip_offset).collect();
        let mut stream = TcpStream::connect(ip_string)?;
        stream.set_nonblocking(false).unwrap();
        conn.remove(0);
        stream.write_all(conn.as_bytes())?;
        let mut buf = [0; 2];

        let n = stream.read(&mut buf[..])?;
        let inc_string = String::from_utf8_lossy(&buf[..n]);
        match inc_string.as_ref() {
            "t" => {}
            _ => return Err(Error::from(std::io::ErrorKind::NotFound)),
        }

        Ok(RemoteSpace {
            stream: Mutex::new(stream),
        })
    }

    fn send(&self, m: Message) -> Result<(), std::io::Error> {
        let mut stream = self.stream.lock().unwrap();
        let m_json = serde_json::to_string(&m)?;
        stream.write_all(m_json.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn recv(&self) -> Result<Tuple, std::io::Error> {
        let mut stream = self.stream.lock().unwrap();
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer[..])?;
        let inc_string = String::from_utf8_lossy(&buffer[..n]);
        let mut message = serde_json::from_str::<Message>(&inc_string)?;
        if message.tuple.len() == 1 {
            Ok(message.tuple.remove(0))
        } else {
            Err(Error::from(std::io::ErrorKind::NotFound))
        }
    }

    fn recv_multiple(&self) -> Result<Vec<Tuple>, std::io::Error> {
        let mut stream = self.stream.lock().unwrap();
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer[..])?;
        let inc_string = String::from_utf8_lossy(&buffer[..n]);
        let message = serde_json::from_str::<Message>(&inc_string)?;
        Ok(message.tuple)
    }

    fn send_recv(&self, m: Message) -> Result<Tuple, std::io::Error> {
        self.send(m)?;
        self.recv()
    }
}

impl Space for RemoteSpace {
    fn get(&self, template: Template) -> Result<Tuple, std::io::Error> {
        let m = Message {
            action: MessageType::Get,
            tuple: Vec::new(),
            template,
        };
        self.send_recv(m)
    }

    fn getp(&self, template: Template) -> Result<Tuple, std::io::Error> {
        let m = Message {
            action: MessageType::Getp,
            tuple: Vec::new(),
            template,
        };
        self.send_recv(m)
    }

    fn put(&self, tuple: Tuple) -> Result<(), std::io::Error> {
        let m = Message {
            action: MessageType::Put,
            tuple: Vec::from([tuple]),
            template: new_template!(),
        };
        self.send(m)?;
        let mut stream = self.stream.lock().unwrap();
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer[..])?;
        let inc_string = String::from_utf8_lossy(&buffer[..n]);
        let message = serde_json::from_str::<Message>(&inc_string)?;
        if message.action == MessageType::Ok {
            Ok(())
        } else {
            Err(Error::from(std::io::ErrorKind::Other))
        }
    }

    fn queryp(&self, template: Template) -> Result<Tuple, std::io::Error> {
        let m = Message {
            action: MessageType::Queryp,
            tuple: Vec::new(),
            template,
        };
        self.send_recv(m)
    }

    fn query(&self, template: Template) -> Result<Tuple, std::io::Error> {
        let m = Message {
            action: MessageType::Query,
            tuple: Vec::new(),
            template,
        };
        self.send_recv(m)
    }

    fn getall(&self, template: Template) -> Result<Vec<Tuple>, std::io::Error> {
        let m = Message {
            action: MessageType::Getall,
            tuple: Vec::new(),
            template,
        };
        self.send(m)?;
        self.recv_multiple()
    }

    fn queryall(&self, template: Template) -> Result<Vec<Tuple>, std::io::Error> {
        let m = Message {
            action: MessageType::Getp,
            tuple: Vec::new(),
            template,
        };
        self.send(m)?;
        self.recv_multiple()
    }
}
