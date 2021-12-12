use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Gift = Graph;

#[derive(Debug, Default, Clone)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn add_edge(&mut self, edge: &(&str, &str)) {
        if !self.edges.contains_key(edge.0) {
            self.edges.insert(edge.0.to_string(), HashSet::new());
        }
        if !self.edges.contains_key(edge.1) {
            self.edges.insert(edge.1.to_string(), HashSet::new());
        }
        self.edges.get_mut(edge.1).unwrap().insert(edge.0.to_string());
        self.edges.get_mut(edge.0).unwrap().insert(edge.1.to_string());
    }
    fn get_small_caves(&self) -> HashSet<String> {
        self.edges.keys().filter(|x| x.chars().next().unwrap().is_ascii_lowercase())
            .map(|x| x.to_string()).collect()
    }
    fn get_adjacent(&self, str: &str) -> &HashSet<String> {
        self.edges.get(str).unwrap()
    }
}

#[inline]
fn is_small_cave(str: &str) -> bool {
    str.chars().next().unwrap().is_ascii_lowercase()
}

fn traverse(graph: &Graph, node: &str, path: &mut Vec<String>, mut seen_twice: bool) -> usize {
    if node == "end" {
        return 1;
    }
    if is_small_cave(node) && path.contains(&node.to_string()) {
        if seen_twice || node == "start" {
            return 0;
        }
        seen_twice = true;
    }
    path.push(node.to_string());
    let a = graph.get_adjacent(node).iter()
        .map(|k| traverse(graph, &k, path, seen_twice)).sum();
    path.pop();
    a
}

#[aoc_generator(day12)]
fn read_input(input: &str) -> Gift {
    let mut graph = Graph::default();
    input.lines().map(|line| line.split_once("-").unwrap())
        .for_each(|e| graph.add_edge(&e));
    graph
}

#[aoc(day12, part1)]
fn day1x1(input: &Gift) -> usize {
    traverse(input, "start", &mut Vec::new(), true)
}


#[aoc(day12, part2)]
fn day1x2(input: &Gift) -> usize {
    traverse(input, "start", &mut Vec::new(), false)
}