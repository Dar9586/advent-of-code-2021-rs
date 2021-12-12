use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Gift = Graph;

#[derive(Debug, Default, Clone)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn add_edge(&mut self, edge: &(&str, &str)) {
        if self.nodes.insert(edge.0.to_string()) {
            self.edges.insert(edge.0.to_string(), HashSet::new());
        }
        if self.nodes.insert(edge.1.to_string()) {
            self.edges.insert(edge.1.to_string(), HashSet::new());
        }
        self.edges.get_mut(edge.1).unwrap().insert(edge.0.to_string());
        self.edges.get_mut(edge.0).unwrap().insert(edge.1.to_string());
    }
    fn get_small_caves(&self) -> HashSet<String> {
        self.nodes.iter().filter(|x| x.chars().next().unwrap().is_ascii_lowercase()).map(|x| x.to_string()).collect()
    }
    fn get_adjacent(&self, str: &str) -> &HashSet<String> {
        self.edges.get(str).unwrap()
    }
}

#[inline]
fn is_small_cave(str: &str) -> bool {
    str.chars().next().unwrap().is_ascii_lowercase()
}

fn traverse(node: &str, graph: &Graph, mut small: HashSet<String>, mut path: Vec<String>, small_twice: bool, paths: &mut HashSet<Vec<String>>) {
    if node == "end" {
        paths.insert(path);
        return;
    }
    path.push(node.to_string());
    if is_small_cave(node) {
        if small.contains(node) && small_twice && node != "start" {
            graph.get_adjacent(node).iter().for_each(|k| traverse(&k, graph, small.clone(), path.clone(), false, paths));
        }
        if !small.remove(node) {
            return;
        }
    }

    graph.get_adjacent(node).iter().for_each(|k| traverse(&k, graph, small.clone(), path.clone(), small_twice, paths));
}

#[aoc_generator(day12)]
fn read_input(input: &str) -> Gift {
    let mut graph = Graph::default();
    input.lines().map(|line| line.split_once("-").unwrap()).for_each(|e| graph.add_edge(&e));
    graph
}

#[aoc(day12, part1)]
fn day1x1(input: &Gift) -> usize {
    let small = input.get_small_caves();
    let mut paths = HashSet::new();
    traverse("start", input, small, Vec::new(), false, &mut paths);
    paths.len()
}


#[aoc(day12, part2)]
fn day1x2(input: &Gift) -> usize {
    let small = input.get_small_caves();
    let mut paths = HashSet::new();
    traverse("start", input, small, Vec::new(), true, &mut paths);
    paths.len()
}