// internal
mod exercises;

use crate::exercises::exercise1::exercise_1;
use crate::exercises::exercise2::exercise_2;


fn main() {
    // exercise 1
    exercise_1(&vec![1, 2, 3, 412, 123, 123, 123, 23, -1, -123, 1230, 45, 75, 356, 7]);

    // exercise 2
    exercise_2("Change to a bigger text.");
}