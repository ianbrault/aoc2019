/*
** src/puzzles/day6.rs
*/

use std::collections::HashMap;
use std::iter;

use crate::puzzles::Puzzle;
use crate::types::DAG;
use crate::utils::PuzzleInput;

pub struct Day6 {
    // maps an object to all objects that orbit it
    // all objects are 3 characters long so they can be losslessly packed into
    // a single string
    orbit_map: HashMap<String, String>,
}

impl Day6 {
    fn get_orbit_map(input: impl Iterator<Item=String>) -> HashMap<String, String> {
        let mut orbits = HashMap::new();

        for line in input {
            let center = String::from(&line[..3]);
            let object = &line[4..];

            // append the object to the center's orbits if the center already
            // exists in the map; otherwise insert a new entry
            orbits.entry(center)
                .and_modify(|o: &mut String| o.push_str(object))
                .or_insert_with(|| object.to_owned());
        }

        orbits
    }

    pub fn new() -> Self {
        let input = PuzzleInput::new(6);
        Self {
            orbit_map: Self::get_orbit_map(input),
        }
    }

    // unpack objects from a string
    fn objects_from_str(obj_str: &str) -> Vec<String> {
        let mut objs = vec![];
        for i in (0..obj_str.len()).step_by(3) {
            objs.push(String::from(&obj_str[i..(i+3)]));
        }
        objs
    }

    // returns an iterator of tuples, which consist of an object zipped to all
    // objects which orbit it
    fn orbit_pairs(&self, obj: String) -> impl Iterator<Item=(String, String)> {
        let orbits = Self::objects_from_str(&self.orbit_map[&obj]);
        iter::repeat(obj).zip(orbits)
    }
}

impl Puzzle for Day6 {
    /// What is the total number of direct and indirect orbits in your
    /// map data?
    fn part_1(&self) -> i64 {
        // maps an object to its total number of orbits (direct + indirect)
        // the center-of-mass (COM) object has 0 orbits
        // else, object B - which orbits A - has 1 + the number of orbits for A
        let mut orbit_cnts = HashMap::new();
        let mut queue = vec![];

        // insert the COM, add its orbits to the queue
        orbit_cnts.insert("COM".to_owned(), 0);
        queue.extend(self.orbit_pairs("COM".to_owned()));

        // continue this process for the objects which orbit the COM and the
        // objects which orbit them until all objects have been processed
        while !queue.is_empty() {
            let (center, orbit) = queue.pop().unwrap();
            orbit_cnts.insert(orbit.clone(), orbit_cnts[&center] + 1);

            if self.orbit_map.contains_key(&orbit) {
                queue.extend(self.orbit_pairs(orbit));
            }
        }

        orbit_cnts.values().sum::<i64>()
    }

    /// What is the minimum number of orbital transfers required to move from
    /// the object YOU are orbiting to the object SAN is orbiting?
    fn part_2(&self) -> i64 {
        // load orbits into a directed-acyclic-graph (DAG)
        let mut dag = DAG::new();
        let mut queue = vec![];

        // center-of-mass (COM) is the root
        dag.insert_root("COM".to_owned());
        queue.extend(self.orbit_pairs("COM".to_owned()));

        // continue this process for the objects which orbit the COM and the
        // objects which orbit them until all objects have been processed
        while !queue.is_empty() {
            let (center, orbit) = queue.pop().unwrap();
            dag.insert(orbit.clone(), center);

            if self.orbit_map.contains_key(&orbit) {
                queue.extend(self.orbit_pairs(orbit));
            }
        }

        // get the paths to the root from YOU and SAN
        let you_path = dag.path_to_root("YOU".to_owned());
        let san_path = dag.path_to_root("SAN".to_owned());

        // find the indices of the first common item between the paths
        let mut you_idx = None;
        let mut san_idx = None;
        for (you_i, obj) in you_path.iter().enumerate() {
            if let Some((san_i, _)) = san_path.iter().enumerate().find(|(_, o)| o == &obj) {
                you_idx = Some(you_i);
                san_idx = Some(san_i);
                break;
            }
        }

        if let (Some(you_idx), Some(san_idx)) = (you_idx, san_idx) {
            (you_idx + san_idx) as i64
        } else {
            panic!("no common orbit found between YOU and SAN");
        }
    }
}
