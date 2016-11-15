extern crate toml;
extern crate xdg;

use rustc_serialize::{Encodable, Decodable};
use std::fmt;
use std::error;
use std::io;
use std::sync::Arc;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use todo::Todo;

#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct TodoList {
    pub todos: Vec<Arc<Todo>>,
    pub last_id: u64,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::<Arc<Todo>>::new(),
            last_id: 0,
        }
    }

    pub fn show(&self) {
        for i in self.clone().todos {
            i.show();
            println!("");
        }
    }

    pub fn get_next_id(&self) -> u64 {
        self.last_id
    }

    pub fn add_todo(&self, t: Todo) -> Self {
        let mut list: TodoList = TodoList {
            todos: self.todos.clone(),
            last_id: self.last_id + 1,
        };
        list.todos.push(Arc::new(t));
        list
    }

    pub fn find_todo(&self, todo: &Todo) -> Option<usize> {
        self.todos.iter().position(|t| todo.eq(t))
    }

    pub fn find_by_id(&self, id: u64) -> Option<(usize, Arc<Todo>)> {
        self.todos.iter().position(|t| t.id == id)
            .map(|pos| (pos, self.todos[pos].clone()))
    }

    // pub fn replace_todo(&self, old: &Todo, new: Todo) -> Self {
    //     let mut list: TodoList = self.clone();
    //     let index = list.find_todo(old).unwrap();
    //     list.todos[index] = Arc::new(new);
    //     list
    // }

    pub fn replace_todo(&self, index: usize, new: Todo) -> Self {
        let mut list: TodoList = self.clone();
        list.todos[index] = Arc::new(new);
        list
    }

    pub fn remove_todo(&self, index: usize) -> Self {
        let mut list: TodoList = self.clone();
        list.todos.remove(index);
        list
    }

    fn from_toml(toml: toml::Table) -> Result<Self, Error> {
        Self::decode(&mut toml::Decoder::new(toml::Value::Table(toml))).map_err(Error::DecodeError)
    }

    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let mut file = try!(File::open(path).map_err(Error::IOError));
        let mut r = String::new();
        try!(file.read_to_string(&mut r).map_err(Error::IOError));
        let mut parser = toml::Parser::new(&mut r);
        let data = match parser.parse() {
            Some(d) => d,
            None => {
                return Err(Error::ParserError(Vec::from_iter(parser.errors.clone().iter().map(move |ref e| {
                    (parser.to_linecol(e.lo), path.to_owned(), (*e).clone())
                }).into_iter())))
            },
        };
        Self::from_toml(data)
    }

    pub fn get_default_path() -> Result<PathBuf, Error> {
        xdg::BaseDirectories::with_prefix("rust-todo")
            .map_err(Error::BaseDirectoriesError)
            .and_then(|dir| dir.place_config_file("tasks.toml").map_err(Error::IOError))
    }

    pub fn load() -> Result<Self, Error> {
        Self::get_default_path()
            .and_then(|p| Self::from_path(p.as_path()))
    }

    pub fn save_to(&self, path: &Path) -> Result<(), Error> {
        let mut file: File = try!(File::create(path).map_err(Error::IOError));
        let mut encoder = toml::Encoder::new();
        try!(self.encode(&mut encoder).map_err(Error::EncodeError));
        write!(&mut file, "{}", toml::Value::Table(encoder.toml)).map_err(Error::IOError)
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = try!(Self::get_default_path());
        self.save_to(path.as_path())
    }
}

#[derive(Debug)]
pub enum Error {
    BaseDirectoriesError(xdg::BaseDirectoriesError),
    DecodeError(toml::DecodeError),
    EncodeError(toml::Error),
    IOError(io::Error),
    ParserError(Vec<((usize, usize), PathBuf, toml::ParserError)>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BaseDirectoriesError(ref err) => write!(f, "BaseDirectories Error: {}", err),
            Error::DecodeError(ref err) => write!(f, "Decode Error: {}", err),
            Error::EncodeError(ref err) => write!(f, "Encode Error: {}", err),
            Error::IOError(ref err) => write!(f, "IO Error: {}", err),
            Error::ParserError(ref errs) => {
                errs.into_iter().fold(write!(f, "Parse Errors:"), |result, &(loc, ref file, ref err)| {
                    result.and(write!(f, "\n  {}:{}:{} - {}", file.display(), loc.0, loc.1, err))
                })
            },
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BaseDirectoriesError(ref err) => err.description(),
            Error::DecodeError(ref err) => err.description(),
            Error::EncodeError(ref err) => err.description(),
            Error::IOError(ref err) => err.description(),
            Error::ParserError(ref errs) => match errs.first() {
                Some(&(_, _, ref err)) => err.description(),
                None => "ParserError",
            },
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::BaseDirectoriesError(ref err) => Some(err),
            Error::DecodeError(ref err) => Some(err),
            Error::EncodeError(ref err) => Some(err),
            Error::IOError(ref err) => Some(err),
            Error::ParserError(ref errs) => errs.first().and_then(|&(_, _, ref err)| err.cause()),
        }
    }
}

