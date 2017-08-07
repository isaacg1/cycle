extern crate cpuprofiler;
use cpuprofiler::PROFILER;


use std::collections::HashSet;

fn cycle_len(num: u64, size: usize) -> usize {
    let mut left = num;
    let mut mask = (1 << (size - 1)) - 1;
    for steps in 0..size {
        left >>= 1;
        if left ^ (num & mask) == 0 {
            return steps + 1;
        }
        mask >>= 1;
    }
    size
}

fn all_cycles(size_log: usize) -> HashSet<Vec<usize>> {
    let mut set = HashSet::new();
    if size_log == 0 {
        set.insert(vec![]);
        return set;
    }
    let size: usize = 1 << size_log;
    let half_size: usize = 1 << size_log - 1;
    let subsizes: Vec<usize> = (1..size_log)
        .map(|subsize_log| {
            let subsize = 1 << subsize_log;
            subsize
        })
        .collect();
    for block in 0..(1 << (half_size - 1)) as u64 {
        let start: u64 = block << half_size;
        if block % 1024 == 0 {
            eprintln!(
                "{} ({:.2}%): {}",
                start,
                start as f64 / (1u64 << size - 1) as f64 * 100f64,
                set.len()
            );
        }
        let leader = {
            let mut cycles = Vec::new();
            for &subsize in &subsizes {
                let subnum = start >> (size - subsize);
                cycles.push(cycle_len(subnum, subsize));
            }
            cycles
        };
        let &end = leader.last().unwrap();
        if (end..(size+1)).all(|count| {
            let mut new = leader.clone();
            new.push(count);
            set.contains(&new)
        })
        {
            continue;
        }
        let mut subset = HashSet::new();
        for num in start..start + (1 << half_size) {
            subset.insert(cycle_len(num, size));
        }
        for unique_num in subset {
            let mut new_l = leader.clone();
            new_l.push(unique_num);
            set.insert(new_l);
        }
    }
    set
}

fn main() {
    let size: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let size_log = size.log2() as usize;
    PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
    let cycles = all_cycles(size_log);
    PROFILER.lock().unwrap().stop().unwrap();
    println!(
        "Number of distinct arrays of periods of bitstrings of length {} is {}",
        1 << size_log,
        cycles.len()
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn cycle() {
        assert_eq!(1, cycle_len(0b1111, 4));
        assert_eq!(1, cycle_len(0b0000, 4));
        assert_eq!(2, cycle_len(0b1010, 4));
        assert_eq!(3, cycle_len(0b1001, 4));
        assert_eq!(4, cycle_len(0b1110, 4));
    }
}
