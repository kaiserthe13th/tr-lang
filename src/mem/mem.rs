use crate::mem::Object;
use crate::hashmap;
use std::collections::HashMap;

pub type Stack = Vec<Object>;
type GStack = Vec<Stack>;
type GHash = Vec<HashMap<String, Object>>;

#[derive(Clone)]
pub struct StackMemory {
    gstack: GStack,
}

#[derive(Clone)]
pub struct HashMemory {
    ghashs: GHash,
}

impl StackMemory {
    pub fn new() -> Self {
        Self {
            gstack: vec![vec![]],
        }
    }

    // Stack Manipulation
    pub fn pop(&mut self) -> Option<Object> {
        for stack in self.gstack.iter_mut().rev() {
            match stack.pop() {
                Some(a) => return Some(a.clone()),
                None => (),
            }
        }
        None
    }
    pub fn push(&mut self, a: Object) {
        if let Some(last) = self.gstack.last_mut() {
            last.push(a);
        }
    }
    pub fn push_glob(&mut self, a: Object) {
        if let Some(first) = self.gstack.first_mut() {
            first.push(a);
        }
    }
    pub fn new_stack(&mut self) {
        self.gstack.push(vec![]);
    }
    pub fn del_stack(&mut self) -> Option<Stack> {
        self.gstack.pop()
    }

    // Stack Info
    pub fn len(&self) -> usize {
        let mut l: usize = 0;
        for stack in self.gstack.iter() {
            l += stack.len();
        }
        l
    }

    // Iter
    pub fn iter_vec(&mut self) -> Vec<Object> {
        let mut s = vec![];
        for stack in self.gstack.iter_mut() {
            s.append(stack);
        }
        s
    }
}

impl HashMemory {
    pub fn new() -> Self {
        Self {
            ghashs: vec![hashmap!{}],
        }
    }

    // Map Manipulation
    pub fn remove(&mut self, key: String) -> Option<Object> {
        for hash in self.ghashs.iter_mut().rev() {
            match hash.remove(&key) {
                Some(a) => return Some(a.clone()),
                None => (),
            }
        }
        None
    }

    pub fn insert(&mut self, key: String, val: Object) {
        if let Some(last) = self.ghashs.last_mut() {
            last.insert(key, val);
        }
    }

    pub fn insert_glob(&mut self, key: String, val: Object) {
        if let Some(first) = self.ghashs.first_mut() {
            first.insert(key, val);
        }
    }

    pub fn new_hash(&mut self) {
        self.ghashs.push(hashmap!{});
    }

    pub fn del_hash(&mut self) -> Option<HashMap<String, Object>> {
        self.ghashs.pop()
    }

    pub fn get(&mut self, key: &String) -> Option<&Object> {
        for hash in self.ghashs.iter().rev() {
            match hash.get(key) {
                Some(a) => return Some(a),
                None => (),
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut Object> {
        for hash in self.ghashs.iter_mut().rev() {
            match hash.get_mut(key) {
                Some(a) => return Some(a),
                None => (),
            }
        }
        None
    }

    pub fn into_keys(&mut self) -> Vec<String> {
        let mut s: Vec<String> = vec![];
        for i in self.ghashs.iter_mut() {
            s.append(&mut i.clone().into_keys().collect::<Vec<String>>());
        }
        s
    }
}