use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct Command {
    pub argurments: Vec<Argument>,
    pub pattern: String,
    pub data_pointer_map: Vec<DataPointer>,
}

impl Command {
    pub fn execute(&mut self, storage: &mut PointerStorage) {
        for arg in &self.argurments {
            match arg {
                Argument::Test => Argument::test(),

                Argument::OpenFile => match storage.check_duplicate(&self.data_pointer_map) {
                    (false, _) => {
                        storage.pointer_list.extend(self.data_pointer_map.clone());
                    }
                    (_, list) => list
                        .iter()
                        .for_each(|item| println!("file {} đã được mở", item.file.get_name_file())),
                },
                Argument::Search => {
                    let mut result: Vec<(usize, String)> = Vec::new();
                    self.data_pointer_map.iter().for_each(|item| {
                        match storage.pointer_list.iter().any(|i| i.name == item.name) {
                            true => {
                                let file = File::open(item.file.get_name_file());
                                match file {
                                    Ok(o) => {
                                        for (i, val) in BufReader::new(o).lines().enumerate() {
                                            match val {
                                                Ok(lines) => {
                                                    if lines.contains(&self.pattern.as_str()) {
                                                        result.push((i, item.name.to_string()));
                                                    }
                                                }
                                                Err(_) => (),
                                            }
                                        }
                                    }
                                    Err(_) => panic!("không mở được file"),
                                }
                            }
                            _ => (),
                        }
                    });
                    result
                        .iter()
                        .for_each(|i| println!("dòng thứ {} ở file {}", i.0, i.1));
                }
            }
        }
    }
}

impl PointerStorage {
    pub fn get_opened_names(&self) -> HashSet<String> {
        self.pointer_list
            .iter()
            .map(|item| item.name.clone())
            .collect()
    }
}

pub struct PointerStorage {
    pub pointer_list: Vec<DataPointer>,
}

impl PointerStorage {
    pub fn new() -> Self {
        Self {
            pointer_list: Vec::new(),
        }
    }

    fn check_duplicate(&self, another_vec: &[DataPointer]) -> (bool, Vec<DataPointer>) {
        let hash: HashSet<String> = another_vec.iter().map(|item| item.name.clone()).collect();
        let duplicated_list: Vec<DataPointer> = self
            .pointer_list
            .iter()
            .filter(|item| hash.contains(&item.name))
            .cloned()
            .collect();
        let bool_value = !duplicated_list.is_empty();
        (bool_value, duplicated_list)
    }
}

#[derive(Clone, Debug)]
pub struct MyFile {
    pub file_path: String,
}

impl MyFile {
    pub fn get_name_file(&self) -> &str {
        &self.file_path.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct DataPointer {
    pub name: String,
    pub file: MyFile,
    pub content: Text,
}

#[derive(Clone, Debug)]
pub enum Text {
    Content(String),
    NoneAtAll,
}

impl Text {
    pub fn get_text(&self) -> String {
        match self {
            Text::Content(s) => s.to_string(),
            _ => String::from("Chưa có chuỗi nào được cho vào đây cả"),
        }
    }
}

pub enum Argument {
    OpenFile,
    Test,
    Search,
}

impl Argument {
    fn test() {
        let a_new = DataPointer {
            name: String::from("một cái string khác"),
            file: MyFile {
                file_path: String::from("hello"),
            },
            content: Text::NoneAtAll,
        };
        println!(
            "🧪 [Test Command Output]: {}, {}, {}",
            a_new.name,
            a_new.file.get_name_file(),
            a_new.content.get_text()
        );
    }
}
