fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [1,3,4 , 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3, 432,4324,32432,3,3,4,5,2,3,23,3,4324,3, 432,4324,32432,3,3,4,5,2,3,23,3, 2432,3,3,4,5,2,3,23,3];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
