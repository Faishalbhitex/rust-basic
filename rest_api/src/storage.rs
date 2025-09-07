use crate::models::Todo;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;

const DATA_FILE: &str = "todos.json";

pub struct Storage {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl Storage {
    pub fn new() -> io::Result<Self> {
        let mut storage = Storage {
            todos: HashMap::new(),
            next_id: 1,
        };
        
        // Try to load existing data
        if let Ok(data) = fs::read_to_string(DATA_FILE) {
            if let Ok(todos_vec) = serde_json::from_str::<Vec<Todo>>(&data) {
                let mut max_id = 0;
                for todo in todos_vec {
                    if todo.id > max_id {
                        max_id = todo.id;
                    }
                    storage.todos.insert(todo.id, todo);
                }
                storage.next_id = max_id + 1;
            }
        }
        
        Ok(storage)
    }
    
    pub fn get_all(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }
    
    pub fn get_by_id(&self, id: u32) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }
    
    pub fn create(&mut self, title: String) -> Todo {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };
        
        self.todos.insert(self.next_id, todo.clone());
        self.next_id += 1;
        self.save().unwrap(); // In real app, handle this error properly
        todo
    }
    
    pub fn update(&mut self, id: u32, title: Option<String>, completed: Option<bool>) -> Option<Todo> {
        if let Some(todo) = self.todos.get_mut(&id) {
            if let Some(t) = title {
                todo.title = t;
            }
            if let Some(c) = completed {
                todo.completed = c;
            }
            let updated_todo = todo.clone(); // Clone dulu sebelum save
            self.save().unwrap();
            Some(updated_todo)
        } else {
            None
        }
    }
    
    pub fn delete(&mut self, id: u32) -> bool {
        if self.todos.remove(&id).is_some() {
            self.save().unwrap();
            true
        } else {
            false
        }
    }
    
    fn save(&self) -> io::Result<()> {
        let todos_vec: Vec<Todo> = self.todos.values().cloned().collect();
        let data = serde_json::to_string_pretty(&todos_vec)?;
        fs::write(DATA_FILE, data)
    }
}
