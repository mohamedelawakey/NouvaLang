use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Parameter, Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
    Break,
    Continue,
    Function {
        parameters: Vec<Parameter>,
        return_type: String,
        body: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    },
    BuiltinFunction(String),
    Error(String),
}

impl Object {
    pub fn type_name(&self) -> &str {
        match self {
            Object::Integer(_) => "int",
            Object::Float(_) => "float",
            Object::String(_) => "str",
            Object::Boolean(_) => "bool",
            Object::Null => "null",
            Object::ReturnValue(val) => val.type_name(),
            Object::Break => "break",
            Object::Continue => "continue",
            Object::Function { .. } => "function",
            Object::BuiltinFunction(_) => "builtin_function",
            Object::Error(_) => "error",
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.to_string(),
            Object::Float(f) => f.to_string(),
            Object::String(s) => s.clone(),
            Object::Boolean(b) => b.to_string(),
            Object::Null => "null".to_string(),
            Object::ReturnValue(val) => val.inspect(),
            Object::Break => "break".to_string(),
            Object::Continue => "continue".to_string(),
            Object::Function { .. } => "[function]".to_string(),
            Object::BuiltinFunction(name) => format!("[builtin function: {}]", name),
            Object::Error(msg) => format!("Runtime Error: {}", msg),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(b) => *b,
            Object::Integer(i) => *i != 0,
            Object::Float(f) => *f != 0.0,
            Object::String(s) => !s.is_empty(),
            Object::Null => false,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    types: HashMap<String, String>,
    constants: HashMap<String, bool>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        store.insert("print".to_string(), Object::BuiltinFunction("print".to_string()));

        Environment {
            store,
            types: HashMap::new(),
            constants: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            types: HashMap::new(),
            constants: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn define(&mut self, name: String, val: Object, var_type: String, is_const: bool) -> Result<Object, String> {
        if self.store.contains_key(&name) {
            return Err(format!("Variable '{}' already declared in this scope", name));
        }

        self.store.insert(name.clone(), val.clone());
        self.types.insert(name.clone(), var_type);
        if is_const {
            self.constants.insert(name, true);
        }
        Ok(val)
    }

    pub fn set(&mut self, name: &str, val: Object) -> Result<Object, String> {
        if self.store.contains_key(name) {
            if self.constants.get(name).copied().unwrap_or(false) {
                return Err(format!("Cannot reassign to constant variable '{}'", name));
            }
            self.store.insert(name.to_string(), val.clone());
            return Ok(val);
        }

        match &self.outer {
            Some(outer) => outer.borrow_mut().set(name, val),
            None => Err(format!("Undefined variable '{}'", name)),
        }
    }
}
