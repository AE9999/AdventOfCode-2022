use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut problem = read_input(&args[1], 1)?;

    println!("{:?} is the sum of the three numbers that form the grove coordinates",
             problem.solve(1));

    let mut problem = read_input(&args[1], 811589153)?;
    println!("{:?} is the sum of the three numbers that form the grove coordinates",
             problem.solve(10));

    Ok(())
}

fn read_input(filename: &String,
              decryption_key: i64) -> io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut tree_manager =  LinkedListManager::new();
    let mut read_nodes : Vec<NodeId> = vec!();

    for data in
        BufReader::new(file_in)
            .lines()
            .map(|line|line.unwrap())
            .map(|line| line.parse::<i64>().unwrap() * decryption_key ) {
        read_nodes.push(tree_manager.alloc_node(data));
    }

    Ok(Problem {
        linked_list_manager: tree_manager,
        read_node_ids: read_nodes
    })
}


#[derive(Debug, Clone)]
struct Problem {
    linked_list_manager: LinkedListManager,
    read_node_ids: Vec<NodeId>,
}

impl Problem {

    fn solve(&mut self, amount: usize) -> i64 {

        for _ in 0..amount {
            for node_id in &self.read_node_ids {
                self.linked_list_manager.move_node(*node_id, self.linked_list_manager.value(*node_id));
            }
        }

        self.value_of_node_with_distance_from_zero_node(1000)
         + self.value_of_node_with_distance_from_zero_node(2000)
         + self.value_of_node_with_distance_from_zero_node(3000)
    }

    fn value_of_node_with_distance_from_zero_node(&self, distance: i64) -> i64 {
        let node_zero_id = self.linked_list_manager.zero_to_node_id.unwrap();
        let node_id = self.linked_list_manager.find_node_with_distance_from_node(node_zero_id, distance);
        self.linked_list_manager.value(node_id)
    }
}

type NodeId = usize;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    data: i64,
    next: Option<NodeId>,
    previous: Option<NodeId>,
}

#[derive(Debug, Clone)]
pub struct LinkedListManager {
    nodes: Vec<Node>,
    zero_to_node_id: Option<NodeId>,
}

impl LinkedListManager {

    fn new() -> Self {
        LinkedListManager {
            nodes: Vec::new(),
            zero_to_node_id: None,
        }
    }

    fn alloc_node(&mut self, data: i64) -> NodeId {
        let id: NodeId = self.nodes.len();

        if data == 0 {
            self.zero_to_node_id = Some(id);
        }

        self.nodes.push(Node {
            data,
            next: Some(0),
            previous: Some(if id == 0 { 0 }  else { id - 1 }),
        });

        self.nodes.get_mut(if id == 0 { 0 }  else { id - 1 }).unwrap().next = Some(id);
        self.nodes.get_mut(0).unwrap().previous = Some(id);

        id
    }

    fn value(&self, node_id: NodeId) -> i64 {
        self.nodes[node_id].data
    }

    fn move_node(&mut self, node_id: NodeId, mut distance: i64) {

        if distance == 0 {
            return
        }

        distance =
            if distance > 0 {
                1 +  ((distance.abs() - 1) % ((self.nodes.len() - 1) as i64))
            } else {
                -1 - ((distance.abs() - 1) % ((self.nodes.len() - 1) as i64))
            }
        ;

        loop {
            let node_id_previous = self.nodes[node_id].previous.unwrap();
            let node_id_next = self.nodes[node_id].next.unwrap();

            if distance == 0 {
                return
            } else if distance > 0 {
                let node_id_next_next =
                    self.nodes.get(self.nodes.get(node_id).unwrap().next.unwrap())
                        .unwrap()
                        .next
                        .unwrap();

                self.nodes.get_mut(node_id_previous).unwrap().next = Some(node_id_next);

                self.nodes.get_mut(node_id).unwrap().next = Some(node_id_next_next);
                self.nodes.get_mut(node_id).unwrap().previous = Some(node_id_next);

                self.nodes.get_mut(node_id_next).unwrap().previous = Some(node_id_previous);
                self.nodes.get_mut(node_id_next).unwrap().next = Some(node_id);

                self.nodes.get_mut(node_id_next_next).unwrap().previous = Some(node_id);

                distance = distance -1;
            } else if distance < 0 {
                let node_id_previous_previous =
                    self.nodes.get(self.nodes.get(node_id).unwrap().previous.unwrap())
                        .unwrap()
                        .previous
                        .unwrap();

                // Fix this
                self.nodes.get_mut(node_id_previous).unwrap().next = Some(node_id_next);
                self.nodes.get_mut(node_id_previous).unwrap().previous = Some(node_id);

                self.nodes.get_mut(node_id).unwrap().next = Some(node_id_previous);
                self.nodes.get_mut(node_id).unwrap().previous = Some(node_id_previous_previous);

                self.nodes.get_mut(node_id_next).unwrap().previous = Some(node_id_previous);

                self.nodes.get_mut(node_id_previous_previous).unwrap().next = Some(node_id);

                distance = distance + 1;
            }
        }
    }

    fn find_node_with_distance_from_node(&self,
                                         mut node_id: NodeId,
                                         mut distance: i64) -> NodeId {
        loop {
            if distance == 1 {
                return self.nodes[node_id].next.unwrap()
            } else if distance == -1 {
                return self.nodes[node_id].previous.unwrap()
            } else if distance > 0 {
                node_id = self.nodes[node_id].next.unwrap();
                distance = distance - 1;
            } else if distance < 0 {
                node_id = self.nodes[node_id].previous.unwrap();
                distance = distance +1;
            } else {
                panic!("Unreachable code")
            }
        }
    }
}
