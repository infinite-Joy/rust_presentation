use std::{thread, time};
use std::string::String;
use rayon::prelude::*;


/// Generate accuracy for ytest and ypreds
/// 
/// ytest and ypred needs to be list of u32's (int32s)
/// # Examples
/// 
/// Basic Usage:
/// ```
/// let xs: [u32; 5] = [1, 2, 3, 4, 5];
/// let ys: [u32; 5] = [1, 2, 3, 4, 5];
/// let acc = accuracy(&xs, &ys);
/// ```
pub fn accuracy(y_test: &[u32], y_preds: &[u32]) -> f32 {
    let mut correct_hits = 0;
    for (predicted, actual) in y_preds.iter().zip(y_test.iter()) {
        if predicted == actual {
            correct_hits += 1;
        }
    }
    let acc: f32 = correct_hits as f32 / y_test.len() as f32;
    acc
}

fn slow_mul(x: u64) -> u64 {
    let five_seconds = time::Duration::new(2, 0);
    thread::sleep(five_seconds);
    x * x
}

fn sum_of_squares_normal() {
    let res: u64 = (0..5u64)
        .map(|x| slow_mul(x)).sum();
    println!("sum_of_squares_normal -> {}", res);
}

fn sum_of_squares_rayon() {
    let res: u64 = (0..5u64).into_par_iter()
        .map(|x| slow_mul(x)).sum();
    println!("sum_of_squares_normal -> {}", res);
}

struct Tennis {
    tennis_ball: String,
}

struct Cricket {
    red_ball: String,
}

trait Play {
    fn play(&self, player: &str);
}

impl Play for Tennis {
    fn play(&self, player: &str) {
        println!("{player} plays in US open with {ball}",
            player=player.to_string(), ball=self.tennis_ball);
    }
}

impl Play for Cricket {
    fn play(&self, player: &str) {
        println!("{player} plays in IPL with {ball}",
            player=player.to_string(), ball=self.red_ball);
    }
}


fn main() {
    // using functions
    // let xs: [u32; 5] = [1, 2, 3, 4, 5];
    // let ys: [u32; 5] = [1, 2, 3, 4, 5];
    // let acc = accuracy(&xs, &ys);
    // println!("accuracy {}", acc);

    // // rust concurrency using libraries
    // // we will also benchmark the result.
    // // for benchmarking though the recommended one is to use
    // // the fdefault benchmarking tool
    // // https://stackoverflow.com/a/57341631/5417164
    // let now = Instant::now();
    // {
    //     sum_of_squares_normal();
    // }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:#?}", elapsed);

    // let now = Instant::now();
    // {
    //     sum_of_squares_rayon();
    // }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:#?}", elapsed);

    // // normal concurrency
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(time::Duration::new(1, 0));
    //     }
    // });
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(time::Duration::new(1, 0));
    // }
    // handle.join().unwrap();

    // traits and structs
    let tennis = Tennis{tennis_ball: "tennis ball".to_string()};
    tennis.play("player");
    let cricket = Cricket{red_ball: "red ball".to_string()};
    cricket.play("player");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy() {
        let xs: [u32; 5] = [1, 2, 3, 4, 5];
        let ys: [u32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(accuracy(&xs, &ys), 1.);
    }

}