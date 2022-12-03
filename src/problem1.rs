use std::fs;

pub(crate) fn part1() {
    let contents = fs::read_to_string("inputs/input1.txt")
        .expect("Input file missing");

    let mut max = 0;
    let mut sum = 0;

    contents.lines().enumerate().for_each(| (i,x)| {
        println!("{i} : {x}");
        if x.is_empty() {
            println!("Set! Sum : {sum}");
            if sum > max {
                println!("New max : {sum}");
                max = sum;
            }
            sum = 0;
        } else {
            sum += x.parse::<u32>().unwrap();
        }
    });

    println!("Max calories : {max}");
}

pub(crate) fn part2() {
    let contents = fs::read_to_string("inputs/input1.txt")
        .expect("Input file missing");

    let mut maxvec = Vec::new();
    let mut sum = 0;

    contents.lines().enumerate().for_each(| (i,x)| {
        println!("{i} : {x}");
        if x.is_empty() {
            println!("Set! Sum : {sum}");
            maxvec.push(sum);
            sum = 0;
        } else {
            sum += x.parse::<u32>().unwrap();
        }
    });

    maxvec.sort();

    println!("Total calories : {}", maxvec[maxvec.len() - 1] + maxvec[maxvec.len() - 2] + maxvec[maxvec.len() - 3]);
}