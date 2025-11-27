fn main() {
    // arrays
    let mut a: [i8; 5] = [1, 2, 3, 4, 5];
    a[4] = 2;
    println!("array: {a:?}");

    let a = [4; 5];
    println!("array: {a:?}");

    // array iteration
    let primes = [2, 3, 5, 7, 11, 13, 17, 23];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    // tuples
    let t: (i8, bool) = (3, true);
    dbg!(t.0);
    dbg!(t.1);

    // unpacking tuples
    let (x, y) = t;
    dbg!(x);
    dbg!(y);

    // - tuples cant be iterated as they can hold different types which is not compatible with loops

    // exercise: nested arrays
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in &matrix {
        println!("{:?}", row);
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in &transposed {
        println!("{:?}", row);
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut out = matrix;
    for i in 0..3 {
        for j in 0..i {
            out[i][j] = matrix[j][i];
            out[j][i] = matrix[i][j];
            // mem::swap(out[i][j], out[j][i]);
        }
    }
    return out;
}
