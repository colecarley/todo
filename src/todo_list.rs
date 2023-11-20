#[path = "./todo_item.rs"] mod todo_item;
use std::fmt;
use mysql::*;
use mysql::prelude::*;

pub struct TodoList {
    list: Vec<todo_item::TodoItem>,
    conn: Option<PooledConn>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            list: Vec::new(),
            conn: None,
        }
    }

    pub fn get_conn(&mut self) {
        let url = "mysql://root:m7c3TDcnU_o@!U9.APnbFm@e@localhost:3306/todo_list";
        let pool = Pool::new(url).expect("Failed to create pool");

        let conn = pool.get_conn().expect("Failed to get connection");
        self.conn = Some(conn);
    }

    pub fn load_all_todos(&mut self) {
        if let Some(ref mut conn) = self.conn {
            let items = conn
            .query_map("SELECT todo_name, id, completed, important FROM todo_item", |(name, id, completed, important): (String, String, bool, bool)| {
                todo_item::TodoItem {
                    name,
                    id,
                    completed,
                    important,
                }
            }).expect("Failed to retrieve data.");
            self.list = items;
            self.sort();
        }
    }

    pub fn add_todo(&mut self, name: String) {
        let new_todo = todo_item::TodoItem::new(name);

        if let Some(ref mut conn) = self.conn {
            conn.exec_drop( 
                "INSERT INTO todo_item (todo_name, completed, important, id) 
                VALUES (:todo_name, :completed, :important, :id)", 
                params! {
                    "todo_name" => new_todo.name.clone(),
                    "completed" => new_todo.completed,
                    "important" => new_todo.important,
                    "id" => new_todo.id.to_string(),
                }
            ).expect("Failed to insert data.");
        }

        self.list.push(new_todo);
    }

    pub fn remove_completed(&mut self) {

        if let Some(ref mut conn)  = self.conn {
            conn.exec_drop(
                "DELETE FROM todo_item WHERE completed = true",
                ()
            ).expect("Failed to delete data.");
        }

        self.list = self.list.iter().cloned().filter(|todo| !todo.completed).collect::<Vec<todo_item::TodoItem>>();
    }

    pub fn mark_not_done(&mut self, index: usize) -> bool {
        let todo = self.get_val(index);
        if todo.is_none() {return false;}
        let id = todo.unwrap().id.clone(); 

        if let Some(ref mut conn) = self.conn {
            conn.exec_drop(
                "UPDATE todo_item SET completed = false WHERE id = :id",
                params! {
                    "id" => id.clone(),
                }
            ).expect("Failed to update data.");
        }

        let result = self.list.iter_mut().find(|todo| todo.id == id).map(|todo| todo.mark_not_done()).is_some();
        self.sort();
        result
    }

    pub fn mark_done(&mut self, index: usize) -> bool {
        let todo = self.get_val(index);
        if todo.is_none() {return false;}
        let id = todo.unwrap().id.clone();

        if let Some(ref mut conn) = self.conn {
            conn.exec_drop(
                "UPDATE todo_item SET completed = true WHERE id = :id",
                params! {
                    "id" => id.clone(),
                }
            ).expect("Failed to update data.");
        }

        let result = self.list.iter_mut().find(|todo| todo.id == id).map(|todo| todo.mark_done()).is_some();
        self.sort();
        result
    }

    pub fn remove(&mut self, index: usize) -> bool {
        let todo = self.get_val(index);
        if todo.is_none() {return false;}
        let id = todo.unwrap().id.clone();

        if let Some(ref mut conn) = self.conn {
            conn.exec_drop(
                "DELETE FROM todo_item WHERE id = :id",
                params! {
                    "id" => id.clone(),
                }
            ).expect("Failed to delete data.");
        }

        self.list = self.list.iter().cloned().filter(|todo| todo.id != id).collect::<Vec<todo_item::TodoItem>>();
        self.sort();
        true
    }

    pub fn mark_important(&mut self, index: usize) -> bool {
        let todo = self.get_val(index);
        if todo.is_none() {return false;}

        let id = todo.unwrap().id.clone();

        if let Some(ref mut conn) = self.conn {
            conn.exec_drop(
                "UPDATE todo_item SET important = true WHERE id = :id",
                params! {
                    "id" => id.clone(),
                } 
            ).expect("Failed to update data.");
        }

        let result = self.list.iter_mut().find(|todo| todo.id == id).map(|todo| todo.mark_important()).is_some();
        self.sort();
        result
    }

    pub fn mark_unimportant(&mut self, index: usize) -> bool {
        let todo = self.get_val(index);
        if todo.is_none() {return false;}

        let id = todo.unwrap().id.clone();

        if let Some(ref mut conn) = self.conn {
            conn.exec_drop(
                "UPDATE todo_item SET important = false WHERE id = :id",
                params! {
                    "id" => id.clone(),
                } 
            ).expect("Failed to update data.");
        }

        let result = self.list.iter_mut().find(|todo| todo.id == id).map(|todo| todo.mark_unimportant()).is_some();
        self.sort();
        result
    }

    pub fn remove_all(&mut self) {
        if let Some(ref mut conn) = self.conn {
            conn.exec_drop(
                "DELETE FROM todo_item",
                ()
            ).expect("Failed to delete data.");
        }
        self.list.clear();
    }

    fn sort (&mut self) {
        self.list.sort_by(|a, b| b.important.cmp(&a.important));
        self.list.sort_by(|a, b| a.completed.cmp(&b.completed));
    }

    fn get_val(&mut self, index: usize) -> Option<todo_item::TodoItem>{
        if (0..self.list.len()).contains(&(index - 1)) {
            return Some(self.list[index - 1].clone());
        }
        None
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.list.len() == 0 {
            return write!(f, "No items in list");
        }

        for (index, todo) in self.list.iter().enumerate() {
            writeln!(f, "{}. {}", index + 1, todo).expect("Error displaying todo");
        }
        Ok(())
    }
}

