use std::fs;
use substring::Substring;

pub(crate) fn part1() {
    let contents = fs::read_to_string("inputs/input2.txt")
        .expect("Input file missing");

    #[derive(Debug, PartialEq, Eq)]
    pub enum Shape {
        Rock,
        Paper,
        Scissors
    }

    impl Shape {
        fn value(&self) -> i32 {
            match *self {
                Shape::Rock=> 1,
                Shape::Paper=> 2,
                Shape::Scissors=> 3,
            }
        }

        fn map_opponent_hand(input: &str) -> Result<Shape, &str> {
            match input {
                "A"  => Ok(Shape::Rock),
                "B"  => Ok(Shape::Paper),
                "C"  => Ok(Shape::Scissors),
                _      => Err("Cannot map hand"),
            }
        }

        fn map_my_hand(input: &str) -> Result<Shape, &str> {
            match input {
                "X"  => Ok(Shape::Rock),
                "Y"  => Ok(Shape::Paper),
                "Z"  => Ok(Shape::Scissors),
                _      => Err("Cannot map hand"),
            }
        }

        fn get_my_score(opponent_hand: Shape, my_hand: Shape) -> i32 {
            if my_hand == opponent_hand {
                3 + my_hand.value()
            } else if (my_hand == Shape::Scissors) && (opponent_hand == Shape::Paper) ||
                (my_hand == Shape::Paper) && (opponent_hand == Shape::Rock) ||
                (my_hand == Shape::Rock) && (opponent_hand == Shape::Scissors) {
                6 + my_hand.value()
            } else {
                my_hand.value()
            }
        }
    }

    let mut sum = 0;
    contents.lines().for_each(| row| {
        println!("{row}");

        let opponent_hand: Shape = Shape::map_opponent_hand(row.substring(0, 1)).unwrap();
        let my_hand: Shape = Shape::map_my_hand(row.substring(2, 3)).unwrap();

        println!("{:?} {:?}", opponent_hand, my_hand);

        let score = Shape::get_my_score(opponent_hand, my_hand);
        println!("Score : {score}");

        sum += score;
    });
    println!("Sum of scores : {sum}");
}