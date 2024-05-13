
fn main() {
    // ##### excersize 1
    println!("Hello, world!");
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    // println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
    // ##### exersize end

    // practise difficulties
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    // s3 is difficult to grasp since it is taking the ref of s2 and then using len(s2) which is 11, further subtracting s1.len (6) also uses '..' at end so def have to study this
    // but i understand it as it is doing 11-6 then from 5th index to rest, i made up s4 for making it ez
    let s3: &str = &s2[s2.len() - s1.len()..];
    let s4: &str = &s2[s2.len() - 2..];
    println!("s3: {s3}");
    println!("s4: {s4}");

    println!(r"\n");// == "\\n"
}

// # Excersize 1 : Nested Arrays

// https://google.github.io/comprehensive-rust/tuples-and-arrays/exercise.html
// Arrays can contain other arrays:

// let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

// What is the type of this variable?

// Use an array such as the above to write a function transpose which will transpose a matrix (turn rows into columns):

// transpose [below matrix] = [solution matrix]

// ⎛⎡1 2 3⎤⎞
// ⎜⎢4 5 6⎥⎟ <= [below matrix]
// ⎝⎣7 8 9⎦⎠

// ⎡1 4 7⎤
// ⎢2 5 8⎥ <= [solution matrix]
// ⎣3 6 9⎦

// Copy the code below to https://play.rust-lang.org/ and implement the function. This function only operates on 3x3 matrices.

// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed_matrix: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }
    transposed_matrix
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}
// will keep at top

// // fn main() {
// //     let matrix = [
// //         [101, 102, 103], // <-- the comment makes rustfmt add a newline
// //         [201, 202, 203],
// //         [301, 302, 303],
// //     ];

// //     println!("matrix: {:#?}", matrix);
// //     let transposed = transpose(matrix);
// //     println!("transposed: {:#?}", transposed);
// // }

// // # Ex 1 end