fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut ans = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            ans[j][i] = matrix[i][j];
        }
    }

    ans
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
