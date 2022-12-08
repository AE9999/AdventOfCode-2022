use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
        static ref CD_CMD: Regex = Regex::new(r"^\$ cd\s+([\w\./]+)$").unwrap();
        static ref DIR_RESULT: Regex = Regex::new(r"^dir\s+([\w\.]+)$").unwrap();
        static ref FILE_RESULT: Regex = Regex::new(r"^(\d+)\s+([\w\.]+)$").unwrap();
}

fn main() ->  io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let node_manager = read_input(&args[1])?;
    println!("{:?} is the sum of the total sizes of those directories", node_manager.solve1());

    println!("{:?} is the total size of that directory", node_manager.solve2());
    Ok(())
}

fn read_input(filename: &String) -> io::Result<NodeManager> {
    let mut node_manager = NodeManager::new();

    let file_in = File::open(filename)?;
    BufReader::new(file_in).lines().map(|x| x.unwrap()).for_each(|line| {
        if CD_CMD.is_match(line.as_str()) {
            let mut cap = CD_CMD.captures_iter(line.as_str());
            let cap  = cap.next().unwrap();
            node_manager.set_active_node(&cap[1]);
        } else if line.starts_with("$ ls") {
            // IGNORE
        } else if DIR_RESULT.is_match(line.as_str()) {
            let mut cap = DIR_RESULT.captures_iter(line.as_str());
            let cap  = cap.next().unwrap();
            node_manager.add_dir_to_current_node(&cap[1]);
        } else if FILE_RESULT.is_match(line.as_str()) {
            let mut cap = FILE_RESULT.captures_iter(line.as_str());
            let cap  = cap.next().unwrap();
            node_manager.add_file_to_current_node(&cap[1].parse::<usize>().unwrap(),
                                                &cap[2]);
        } else {
            panic!("Unexpected line {:?} ..", line)
        }
    });
    Ok(node_manager)
}

type NodeId = usize;

#[derive(Debug, Clone)]
pub struct NodeManager {
    nodes: Vec<Node>,
    current_node_id: NodeId,
}

impl NodeManager {
    fn new() -> Self {
        let mut node_manager = NodeManager {
            nodes: Vec::new(),
            current_node_id: 0,
        };
        node_manager.create_node(None, "/", None);
        node_manager
    }

    fn create_node(&mut self,
                   parent: Option<NodeId>,
                   name: &str,
                   size: Option<usize>) -> NodeId {
        let id =  self.nodes.len();
        self.nodes.push(Node::new(parent,
                                        name.to_string(),
                                       size));
        id
    }

    fn root(&self) -> NodeId {
        return 0
    }

    fn set_active_node(&mut self, name: &str) {
        self.current_node_id =
            if name == "/" {
                self.root()
            } else if name == ".." {
                self.nodes[self.current_node_id].parent.unwrap()
            } else {
                *self.get_child_node_by_name(name).unwrap()
            }
    }

    fn add_dir_to_current_node(&mut self, name :&str) {
        if self.get_child_node_by_name(name).is_some() {
            return
        }
        let child_node = self.create_node(Some(self.current_node_id),
                                                    name,
                                                None);
        self.nodes[self.current_node_id]
            .children.push(child_node);
    }

    fn add_file_to_current_node(&mut self, size: &usize, name :&str) {
        if self.get_child_node_by_name(name).is_some() {
            return
        }
        let child_node = self.create_node(Some(self.current_node_id),
                                                        name,
                                                    Some(*size));
        self.nodes[self.current_node_id]
            .children.push(child_node);
    }

    fn get_child_node_by_name(&self,
                              name: &str) -> Option<&usize> {
        let children =
            self.nodes.get(self.current_node_id)
                .unwrap()
                .children
                .iter()
                .filter(|child_id| self.nodes
                    .get(**child_id)
                    .unwrap().name == name)
                .take(1).collect::<Vec<&usize>>();
        if children.len() > 0 {
            Some(*children.get(0).unwrap())
        }  else {
            None
        }
    }

    fn solve1(&self) -> usize {
        let max_size: usize = 100000;
        self.nodes.iter()
                  .filter(|node| node.size.is_none())
                  .map(|node| node.size(self))
                  .filter(|size| *size <= max_size)
                  .fold(0, |sum, val| sum + val)
    }

    fn solve2(&self) -> usize {
        let needed_size: usize = 30000000;
        let system_size: usize = 70000000;
        let current_size: usize = system_size - self.nodes[self.root()].size(self);

        let mut candidates =
            self.nodes.iter()
                      .filter(|node| node.size.is_none())
                      .map(|node| node.size(self)
                      )
                      .filter(|size| *size + current_size >= needed_size)
                      .collect::<Vec<usize>>();

        candidates.sort();

        candidates[0] // guaranteed to exist, worse case we need to delete root.
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    parent: Option<NodeId>,
    size: Option<usize>,
    children: Vec<NodeId>,
}

impl Node {
    fn new(parent: Option<NodeId>,
           name: String,
           size: Option<usize>) -> Self {
        Node {
            name,
            parent,
            size,
            children: Vec::new(),
        }
    }

    fn size(&self, node_manager: &NodeManager)  -> usize {
        if self.size.is_some() {
            self.size.unwrap()
        } else {
            self.children.iter()
                .map(|child_id|node_manager.nodes
                    .get(*child_id)
                    .unwrap()
                    .size(node_manager))
                .fold(0, |sum, val| sum + val)
        }
    }
}
