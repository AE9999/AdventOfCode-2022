use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() ->  io::Result<()> {
    let args: Vec<String> = env::args().collect();

    //
    // TODO: Next year ignore the empty lines and just just chunks.
    //
    let mut tree_manager = TreeManager::new();
    let pairs = read_input(&args[1], &mut tree_manager)?;
    println!("{:?} is the sum of the indices of those pairs",
             solve1(&pairs, &mut tree_manager));

    let mut tree_manager = TreeManager::new();
    let part2 = read_input_flat(&args[1], &mut tree_manager)?;
    println!("{:?} is the decoder key for the distress signal",
             solve2(part2, &mut tree_manager));

    Ok(())
}

fn solve1(pairs: &Vec<(NodeId, NodeId)>,  tree_manager: &mut TreeManager) -> usize {
    pairs.iter()
         .enumerate()
         .map(|(index, pair)|
             match cmp(pair.0, pair.1, tree_manager) {
                 Ordering::Less => { index + 1 }
                 _ => { 0 }
             }
         ).sum()
}

fn solve2(mut part2: Part2, tree_manager: &mut TreeManager) -> usize {
    part2.packets
         .sort_by(|left, right| cmp(*left, *right, tree_manager));

    let index_1 = part2.packets
                             .iter()
                             .position(|&node_id| node_id == part2.divider_packets[0])
                             .unwrap();

    let index_2 = part2.packets
                             .iter()
                             .position(|&node_id| node_id == part2.divider_packets[1])
                             .unwrap();

    (index_1 + 1) * (index_2 + 1)
}

fn cmp(left: NodeId, right: NodeId, tree_manager: &mut TreeManager) -> Ordering  {
    if tree_manager.is_leaf(left)
        && tree_manager.is_leaf(right) {
        tree_manager.to_value(left).cmp(&tree_manager.to_value(right))
    } else if tree_manager.is_list(left) && tree_manager.is_list(right) {
        let left_children = tree_manager.to_children(left);
        let right_children = tree_manager.to_children(right);
        for i in 0..left_children.len() {
            let left_child = left_children[i]; // guaranteed to be there
            let right_child = right_children.get(i);
            if right_child.is_none() {
                return Ordering::Greater
            }
            let cmp = cmp(left_child,
                                  *right_child.unwrap(),
                                       tree_manager);
            match cmp {
                Ordering::Equal => { /* skip */}
                _ => { return cmp }
            }
        }
        if left_children.len() != right_children.len() { Ordering::Less }
        else { Ordering::Equal }
    } else if tree_manager.is_list(left) && tree_manager.is_leaf(right) {
        let mut new_children: Vec<NodeId> = Vec::new();
        new_children.push(right);
        let new_right = tree_manager.alloc_node(new_children);
        cmp(left, new_right, tree_manager)
    } else if tree_manager.is_leaf(left) && tree_manager.is_list(right) {
        let mut new_children: Vec<NodeId> = Vec::new();
        new_children.push(left);
        let new_left = tree_manager.alloc_node(new_children);
        cmp(new_left, right, tree_manager)
    } else {
        panic!("Should be unreachable")
    }
}

fn read_input(filename: &String, tree_manager: &mut TreeManager) -> io::Result<Vec<(NodeId, NodeId)>> {
    let mut rvalue: Vec<(NodeId, NodeId)> = Vec::new();

    let file_in = File::open(filename)?;
    let mut it =  BufReader::new(file_in).lines();
    loop {
        let left = parse_line(it.next().unwrap().unwrap().as_str(), tree_manager);
        let right = parse_line(it.next().unwrap().unwrap().as_str(), tree_manager);

        rvalue.push((left, right));
        let blank_line = it.next();
        if blank_line.is_none() { break; }
    }

    Ok(rvalue)
}

fn read_input_flat(filename: &String, tree_manager: &mut TreeManager) -> io::Result<Part2> {
    let mut packets: Vec<NodeId> = Vec::new();
    let mut divider_packets: Vec<NodeId> = Vec::new();

    let file_in = File::open(filename)?;
    let mut it =  BufReader::new(file_in).lines();

    loop {
        let line = it.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap().unwrap();
        if !line.is_empty() {
            packets.push(parse_line(line.as_str(), tree_manager))
        }
    }

    let package_id = parse_line("[[2]]", tree_manager);
    packets.push(package_id);
    divider_packets.push(package_id);

    let package_id = parse_line("[[6]]", tree_manager);
    packets.push(package_id);
    divider_packets.push(package_id);

    Ok(Part2 {
        packets,
        divider_packets,
    })
}

struct Part2 {
    packets: Vec<NodeId>,
    divider_packets: Vec<NodeId>,
}

fn parse_line(line: &str, tree_manager: &mut TreeManager) -> NodeId {
    let mut nodes : Vec<Vec<NodeId>> = Vec::new();
    let mut digit_buffer: Vec<char> = Vec::new();

    for c in line.chars() {

        match c {
            '[' => {
                assert!(digit_buffer.is_empty());
                nodes.push(Vec::new())
            },
            ']' => {
                if !digit_buffer.is_empty() {
                    let node = tree_manager.alloc_leaf (&mut digit_buffer);
                    nodes.last_mut().unwrap().push(node)
                }

                let children = nodes.pop().unwrap();
                let node = tree_manager.alloc_node(children);
                if nodes.is_empty() {
                    return node;
                } else {
                    nodes.last_mut().unwrap().push( node)
                }
            },
            ',' => {
                if !digit_buffer.is_empty() {
                    let node = tree_manager.alloc_leaf (&mut digit_buffer);
                    nodes.last_mut().unwrap().push(node)
                }
            },
            _  => {
                digit_buffer.push(c);
            }
        }
    }

    panic!("Should not be reached ..");
}

type NodeId = usize;

#[derive(Debug)]
struct Node {
    data: Option<u32>,
    children: Option<Vec<NodeId>>
}

#[derive(Debug)]
pub struct TreeManager {
    nodes: Vec<Node>,
}

impl TreeManager {
    fn new() -> Self {
        TreeManager {
            nodes: Vec::new()
        }
    }

    fn alloc_leaf(&mut self, digit_buffer: &mut  Vec<char>) -> NodeId {
        let data =
            digit_buffer.iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
        digit_buffer.clear(); // Ensure buffer is empty
        let id = self.nodes.len();
        self.nodes.push(Node {
            data: Some(data),
            children: None,
        });
        id
    }

    fn alloc_node(&mut self, children: Vec<NodeId>) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node {
            data: None,
            children: Some(children),
        });
        id
    }

    fn is_leaf(&self, node_id: NodeId) -> bool {
        self.nodes[node_id].data.is_some()
    }

    fn is_list(&self, node_id: NodeId) -> bool {
        self.nodes[node_id].children.is_some()
    }

    fn to_value(&self, node_id: NodeId) -> u32 {
        assert!(self.is_leaf(node_id), "Can't get the value from a non-leaf node");
        self.nodes[node_id].data.unwrap()
    }

    fn to_children(&self, node_id: NodeId) -> Vec<NodeId> {
        assert!(self.is_list(node_id), "Can't get the children for a non-tree node");
        self.nodes[node_id].children.as_ref().unwrap().clone()
    }
}
