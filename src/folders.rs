use std::fmt;
use uuid;

pub struct Folders {
    pub list: Vec<Folder>,
}

#[derive(Clone)]
pub struct Folder {
    pub name: String,
    pub id: String,
}

impl Folder {
    pub fn new(name: String) -> Folder {
        Folder {
            name: name,
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

impl Folders {
    pub fn new() -> Folders {
        let default = Folder::new("Default".to_string());
        Folders {
            list: vec![default.clone()],
        }
    }

    pub fn add_folder(&mut self, name: String) -> Folder {
        let new_folder = Folder::new(name);
        self.list.push(new_folder.clone());
        return new_folder;
    }

    pub fn remove_folder(&mut self, index: usize) -> bool {
        if (0..self.list.len()).contains(&(index - 1)) {
            self.list.remove(index);
            return true;
        }
        false
    }

    pub fn get_val(&self, index: usize) -> Option<&Folder> {
        self.list.get(index)
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Folders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, folder) in self.list.iter().enumerate() {
            writeln!(f, "{}: {}", i + 1, folder)?;
        }
        Ok(())
    }
}
