//extern crate cpuprofiler;
//use cpuprofiler::PROFILER;

extern crate bit_vec;
use bit_vec::BitVec;

use std::collections::HashSet;

fn cycle_len(num: u32, mask: u32, skip_steps: usize) -> usize {
    let mut left = num >> (skip_steps + 1);
    let mut mask = mask >> skip_steps;
    let mut steps = skip_steps;
    while left != num & mask {
        left >>= 1;
        mask >>= 1;
        steps += 1;
    }
    steps
}

fn all_cycles(size_log: usize) -> HashSet<Vec<usize>> {
    let mut set = HashSet::new();
    if size_log == 0 {
        set.insert(vec![]);
        return set;
    } else if size_log == 1 {
        set.insert(vec![0]);
        set.insert(vec![1]);
        return set;
    }
    let size: usize = 1 << size_log;
    let half_size: usize = 1 << size_log - 1;
    let shift_and_mask: Vec<(usize, u32)> = (1..size_log)
        .map(|subsize_log| {
            let subsize = 1 << subsize_log;
            (size - subsize, (1 << (subsize - 1)) - 1)
        })
        .collect();
    let size_mask = (1 << (size - 1)) - 1;
    for block in 0..(1 << (half_size - 1)) as u32 {
        let start: u32 = block << half_size;
        if block % 1024 == 0 {
            eprintln!(
                "{} ({:.2}%): {}",
                start,
                start as f64 / (1u64 << size - 1) as f64 * 100f64,
                set.len()
            );
        }
        let leader: Vec<usize> = shift_and_mask.iter().map(|&(shift, mask)|{
                let subnum = start >> shift;
                cycle_len(subnum, mask, 0)
            }).collect();
        let &end = leader.last().unwrap();
        if (end..size).all(|count| {
            let mut new = leader.clone();
            new.push(count);
            set.contains(&new)
        })
        {
            continue;
        }
        let mut subset = BitVec::from_elem(size, false);
        for num in start..start + (1 << half_size) {
            subset.set(cycle_len(num, size_mask, end), true);
        }
        for (unique_cycle_len, _) in subset.into_iter().enumerate().filter(|x| x.1) {
            let mut new_l = leader.clone();
            new_l.push(unique_cycle_len);
            set.insert(new_l);
        }
    }
    set
}

fn main() {
    let size: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let size_log = size.log2() as usize;
    //PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
    let cycles = all_cycles(size_log);
    //PROFILER.lock().unwrap().stop().unwrap();
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
