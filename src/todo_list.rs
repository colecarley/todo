#[path = "./todo_item.rs"] mod todo_item;
use std::fmt;
use sqlite;
use std::env;

pub struct TodoList {
    list: Vec<todo_item::TodoItem>,
    conn: Option<sqlite::Connection>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            list: Vec::new(),
            conn: None,
        }
    }

    pub fn get_conn(&mut self) {
        // create a table accessible at the home directory to ensure global scope
        let foo = env::home_dir().unwrap().to_str().unwrap().to_string() + "/todo_list.db";
        let connection = sqlite::open(foo).expect("Failed to open database");
        self.conn = Some(connection);

        if let Some(ref mut conn) = self.conn {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS todo_item (
                    todo_name TEXT NOT NULL,
                    id TEXT NOT NULL,
                    completed BOOLEAN NOT NULL,
                    important BOOLEAN NOT NULL,
                    folder_id TEXT NOT NULL
                )",
            ).expect("Failed to create table.");
        }
    }

    pub fn load_all_todos(&mut self) {
        if let Some(ref mut conn) = self.conn {

            let query = "SELECT * FROM todo_item";
            for row in conn
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap())
            {
                self.list.push(
                    todo_item::TodoItem {
                        name: row.read::<&str, _>("todo_name").to_string(),
                        id: row.read::<&str, _>("id").to_string(),
                        completed: row.read::<i64, _>("completed") != 0,
                        important: row.read::<i64, _>("important") != 0,
                    }
                )
            }
            self.sort();
        }
    }

    pub fn add_todo(&mut self, name: String) {
        let new_todo = todo_item::TodoItem::new(name);

        if let Some(ref mut conn) = self.conn {
            let query = format!(
                "INSERT INTO todo_item (todo_name, completed, important, id) 
                VALUES ('{}', {}, {}, '{}')", 
                new_todo.name, 
                new_todo.completed, 
                new_todo.important, 
                new_todo.id.to_string()
            );

            conn.execute(query).expect("Failed to insert data.");
        }

        self.list.push(new_todo);
        self.sort();
    }

    pub fn remove_completed(&mut self) {
        if let Some(ref mut conn)  = self.conn {
            conn.execute("DELETE FROM todo_item WHERE completed = true").expect("Failed to delete data.");
        }

        self.list = self.list.iter().cloned().filter(|todo| !todo.completed).collect::<Vec<todo_item::TodoItem>>();
    }

    pub fn mark_not_done(&mut self, index: usize) -> bool {
        let todo = self.get_val(index);
        if todo.is_none() {return false;}
        let id = todo.unwrap().id.clone(); 

        if let Some(ref mut conn) = self.conn {
            let query = format!(
                "UPDATE todo_item SET completed = false WHERE id = '{}'", 
                id.to_string()
            );

            conn.execute(query).expect("Failed to update data.");
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
            let query = format!(
                "UPDATE todo_item SET completed = true WHERE id = '{}'", 
                id.to_string()
            );

            conn.execute(query).expect("Failed to update data.");
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
            let query = format!(
                "DELETE FROM todo_item WHERE id = '{}'", 
                id.to_string()
            );

            conn.execute(query).expect("Failed to delete data.");
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
            let query = format!(
                "UPDATE todo_item SET important = true WHERE id = '{}'", 
                id.to_string()
            );

            conn.execute(query).expect("Failed to update data.");
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
            let query = format!(
                "UPDATE todo_item SET important = false WHERE id = '{}'", 
                id.to_string()
            );

            conn.execute(query).expect("Failed to update data.");
        }

        let result = self.list.iter_mut().find(|todo| todo.id == id).map(|todo| todo.mark_unimportant()).is_some();
        self.sort();
        result
    }

    pub fn remove_all(&mut self) {
        if let Some(ref mut conn) = self.conn {
            conn.execute("DELETE FROM todo_item").expect("Failed to delete data.");
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

