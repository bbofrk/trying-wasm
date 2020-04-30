mod utils;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-tree-structure!");
}


fn main() {
    #[wasm_bindgen]
    pub struct TreeStructure {
        directed_graph: Graph<String, usize>
    }
    #[wasm_bindgen]
    impl TreeStructure {
        pub fn new() -> TreeStructure {
            TreeStructure {
                directed_graph: Graph::<String, usize>::new()
            }
        }

        pub fn add_node(&mut self, new_node: String) {
            let find_index = self.find_node(&new_node);
            if find_index.is_none() {
                self.directed_graph.add_node(new_node);
            } else {
                // alert("cant add, this {} already exist in the nodes", newNode);
            }
        }

        pub fn remove_node(&mut self, to_remove_node: String) {
            let find_index = self.find_node(&to_remove_node);
            if find_index.is_some() {
                self.directed_graph.remove_node(find_index.unwrap());
            } else {
                // alert("cant remove, cannot find {} in the nodes", toRemoveNode);
            }
        }

        pub fn add_edge(&mut self, from_node: String, to_node: String) {
            let from_index = self.find_node(&from_node);
            let to_index = self.find_node(&to_node);
            if from_index.is_some() && to_index.is_some() {
                let already_edged = self.directed_graph.contains_edge(from_index.unwrap(), to_index.unwrap());
                if !already_edged {
                    self.directed_graph.add_edge(from_index.unwrap(), to_index.unwrap(), 1);
                } else {
                    // alert("edge already exist from {} to {}", fromNode, toNode);
                }
            }
        }

        pub fn remove_edge(&mut self, from_node: String, to_node: String) {
            let from_index = self.find_node(&from_node);
            let to_index = self.find_node(&to_node);
            if from_index.is_some() && to_index.is_some() {
                let already_edged = self.directed_graph.contains_edge(from_index.unwrap(), to_index.unwrap());
                if already_edged {
                    let edge_index = self.directed_graph.find_edge(from_index.unwrap(), to_index.unwrap());
                    self.directed_graph.remove_edge(edge_index.unwrap());
                } else {
                    // alert("edge does not exist exist from {} to {}", fromNode, toNode);
                }
            }
        }

        pub fn get_neighbors(&self, check_node: String) -> JsValue {
            let check_index = self.find_node(&check_node);
            if check_index.is_none() {
                let v: Vec<String> = Vec::new();
                return JsValue::from_serde(&v).unwrap();
            }
            let v: Vec<&String> = self.directed_graph.neighbors(check_index.unwrap()).map(|i| &self.directed_graph[i] ).collect();
            JsValue::from_serde(&v).unwrap()
        }

        pub fn get_dfs(&self, check_node: String) -> JsValue {
            let check_index = self.find_node(&check_node);
            let mut v: Vec<&str> = Vec::new();
            if check_index.is_none() {
                return JsValue::from_serde(&v).unwrap();
            }
            // let v: Vec<&String> = self.directed_graph.neighbors(check_index.unwrap()).map(|i| &self.directed_graph[i] ).collect();
            let mut dfs = Dfs::new(&self.directed_graph, check_index.unwrap());
            while let Some(visited) = dfs.next(&self.directed_graph) {
                v.push(&self.directed_graph[visited]);
            }
            JsValue::from_serde(&v).unwrap()
        }

    }
    impl TreeStructure {
        pub fn find_node(&self, node_to_find: &String) -> Option<NodeIndex> {
            let find_index = self.directed_graph.node_indices().find(|i| self.directed_graph[*i] == node_to_find.to_string());
            find_index
        }
    }
}
