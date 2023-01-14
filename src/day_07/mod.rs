use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, fs, rc::Rc};

const FILE_PATH: &str = "src/day_07/resources/input.txt";

const DIRECTORY_SIZE_THRESHOLD: u32 = 100000;
const TOTAL_DISK_SPACE: u32 = 70000000;
const REQUIRED_DISK_SPACE: u32 = 30000000;

lazy_static! {
    static ref CD_PARENT_PATTERN: Regex = Regex::new(r"cd \.\.").unwrap();
    static ref CD_CHILD_PATTERN: Regex = Regex::new(r"cd [a-zA-Z]+").unwrap();
    static ref LS_PATTERN: Regex = Regex::new(r"ls").unwrap();
    static ref DIRECTORY_CHILD: Regex = Regex::new(r"dir [a-zA-Z]+").unwrap();
    static ref FILE_CHILD: Regex = Regex::new(r"[0-9]+ *").unwrap();
}

struct Command {
    command: String,
    output: Vec<String>,
}

struct Node {
    size: Option<u32>,
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String) -> Node {
        return Node {
            name,
            size: None,
            children: vec![],
            parent: None,
        };
    }

    fn add_child(&mut self, child_node: Rc<RefCell<Node>>) {
        self.children.push(child_node);
    }

    fn get_child_by_name(&self, name: String) -> &Rc<RefCell<Node>> {
        let child = self
            .children
            .iter()
            .find(|node| node.borrow().name == name)
            .unwrap();

        return child;
    }

    fn get_total_size(&self) -> u32 {
        if self.size.is_none() {
            return self
                .children
                .iter()
                .fold(0, |acc: u32, cur: &Rc<RefCell<Node>>| {
                    return acc + cur.borrow().get_total_size();
                });
        }

        return self.size.unwrap();
    }

    fn get_subdirs_sizes(&self) -> Vec<u32> {
        let mut subdirs_sizes = Vec::new();

        let subdirs = self
            .children
            .iter()
            .filter(|child| child.borrow().size.is_none());

        let mut subsubdirs_sizes: Vec<u32> = subdirs
            .clone()
            .map(|subdir| subdir.borrow().get_subdirs_sizes())
            .flatten()
            .collect();

        subdirs.for_each(|item| subdirs_sizes.push(item.borrow().get_total_size()));

        subdirs_sizes.append(&mut subsubdirs_sizes);

        return subdirs_sizes;
    }
}

fn process_raw_command(raw_command: &str) -> Command {
    let contents: Vec<&str> = raw_command.split("\n").collect();

    let command = contents.first().unwrap().to_string();
    let output = &contents[1..];

    Command {
        command: command,
        output: output
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>(),
    }
}

fn add_children_to_node(command_output: &Vec<String>, current_node: &Rc<RefCell<Node>>) {
    for item in command_output {
        let splitted: Vec<&str> = item.split(" ").collect();

        let new_node = Rc::new(RefCell::new(Node::new(splitted[1].to_string())));
        let mut mut_new_node = new_node.borrow_mut();

        mut_new_node.parent = Some(Rc::clone(&current_node));

        if DIRECTORY_CHILD.is_match(item) {
            current_node.borrow_mut().add_child(Rc::clone(&new_node));
        }
        if FILE_CHILD.is_match(item) {
            mut_new_node.size = Some(splitted[0].parse::<u32>().unwrap());

            current_node.borrow_mut().add_child(Rc::clone(&new_node));
        }
    }
}

fn build_filesystem(commands: &Vec<Command>) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new("/".to_string())));
    let mut current_node = Rc::clone(&root);

    for command in commands {
        if CD_PARENT_PATTERN.is_match(&command.command) {
            let current_clone = Rc::clone(&current_node);

            current_node = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        }

        if CD_CHILD_PATTERN.is_match(&command.command) {
            let splitted: Vec<&str> = command.command.split(" ").collect();
            let child_to_access = Rc::clone(
                current_node
                    .borrow()
                    .get_child_by_name(splitted[2].to_string()),
            );

            current_node = Rc::clone(&child_to_access);
        }

        if LS_PATTERN.is_match(&command.command) {
            add_children_to_node(&command.output, &current_node)
        }
    }

    root
}

fn get_directory_to_delete_size(filesystem: &Rc<RefCell<Node>>) -> u32 {
    let space_to_free =
        REQUIRED_DISK_SPACE - (TOTAL_DISK_SPACE - filesystem.borrow().get_total_size());

    let directories_sizes = filesystem.borrow().get_subdirs_sizes();

    let smallest_suitable_dir = directories_sizes
        .iter()
        .filter(|size| size >= &&space_to_free)
        .min()
        .unwrap();

    *smallest_suitable_dir
}

pub fn execute() {
    let content = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let commands = content
        .split("\n$")
        .map(process_raw_command)
        .collect::<Vec<Command>>();

    let filesystem = build_filesystem(&commands);

    let result_1: u32 = filesystem
        .borrow()
        .get_subdirs_sizes()
        .iter()
        .filter(|size| size < &&DIRECTORY_SIZE_THRESHOLD)
        .sum();

    let result_2 = get_directory_to_delete_size(&filesystem);

    println!("Result 1 {}", result_1);
    println!("Result 2 {}", result_2);
}
