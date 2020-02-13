//use ferris_says::say;
//use std::io::{stdout, BufWriter};

use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string;
extern crate ndarray;

type DistanceTable = ndarray::Array2::<f32>;
type TRSPath = ndarray::Array1::<usize>;

fn main() {

    let (cities, distance_table) = load_cities("./european_cities.csv");
    println!("{}", ndarray::Array::from(cities.clone()));
    println!("{}", distance_table);

    let test_path: TRSPath = ndarray::array![1, 2, 3, 4, 5];

    println!("{}", TRSPath::path_to_string(&cities, &test_path));
    
}

trait DistanceTableCompatible {
    fn path_to_string(cities: &Vec<String>, path: &TRSPath) -> String;
}

impl DistanceTableCompatible for TRSPath {
    fn path_to_string(cities: &Vec<String>, path: &TRSPath) -> String {
        let mut ret = String::new();
        for i in 0..path.len() {
            ret.push_str(&cities[path[i]]);
        }
        ret
    }
}

fn load_cities<P>(filename: P) -> (Vec<String>, DistanceTable)
where P: AsRef<Path> {
    if let Ok(line_buffer) = read_lines(filename) {
        let mut cities: Vec<String> = vec!();
        let mut distance_table: DistanceTable;
        let mut lines: Vec<String> = vec!();
        for line in line_buffer {
            if let Ok(l) = line {
                lines.push(l);
            }
        }
        let size = lines.len() - 1;
        distance_table = ndarray::Array::zeros((size, size));
        
        // extract city names
        for w in lines[0].split(";") {
            cities.push(String::from(w));
        }

        //extract distance table
        let mut i = 0;
        for line in &lines[1..] {
            let mut j = 0;
            for w in line.split(";") {
                distance_table[[i, j]] = w.parse::<f32>().unwrap();
                j += 1;
            }
            i += 1;
        }
        (cities, distance_table)    
    }
    else {
        panic!()
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}