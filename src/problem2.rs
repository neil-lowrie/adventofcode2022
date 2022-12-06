use std::fs;
use substring::Substring;

pub(crate) fn problem2() {
    let contents = fs::read_to_string("inputs/input2.txt")
        .expect("Input file missing");

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

        fn get_winning_shape(opponents_shape: Shape) -> Shape {
            match opponents_shape {
                Shape::Rock  => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            }
        }

        fn get_losing_shape(opponents_shape: Shape) -> Shape {
            match opponents_shape {
                Shape::Rock  => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            }
        }

        fn map_my_hand_specific(opponents_shape: Shape, input: &str) -> Result<Shape, &str> {
            match input {
                "X"  => Ok(Shape::get_losing_shape(opponents_shape)),
                "Y"  => Ok(opponents_shape),
                "Z"  => Ok(Shape::get_winning_shape(opponents_shape)),
                _      => Err("Cannot map hand"),
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

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    contents.lines().for_each(| row| {
        println!("{row}");

        let opponent_hand: Shape = Shape::map_opponent_hand(row.substring(0, 1)).unwrap();
        let my_hand: Shape = Shape::map_my_hand(row.substring(2, 3)).unwrap();
        let my_hand_specific: Shape = Shape::map_my_hand_specific(opponent_hand, row.substring(2, 3)).unwrap();

        println!("{:?} {:?} {:?}", opponent_hand, my_hand, my_hand_specific);

        let mut score = Shape::get_my_score(opponent_hand, my_hand);
        println!("Score part 1 : {score}");

        sum_part_1 += score;

        score = Shape::get_my_score(opponent_hand, my_hand_specific);
        println!("Score part 2 : {score}");

        sum_part_2 += score;
    });

    println!("Sum of scores part 1 : {sum_part_1}");
    println!("Sum of scores part 2 : {sum_part_2}");
}