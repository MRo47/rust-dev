fn main() {
    // shared references
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);

    // exclusive (mutable) references
    let mut point = (2, 1);
    let x_coord = &mut point.0;
    *x_coord = 20; // dereference to assign value
    println!("point: {point:?}");

    // slices
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4]; // slice with indices 2 and 3, no length specified
    println!("s: {s:?}");

    // slcies are borrowed so the array a must be alive as long as its slice is used

    // Strings

    // &str is a slice of UTF-8 encoded bytes, similar to &[u8]. or std::string_view in c++
    // String is an owned buffer of UTF-8 encoded bytes, similar to Vec<T>.

    let s1: &str = "World";
    println!("s1: {s1:?}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2:?}");

    s2.push_str(s1);
    println!("s2: {s2:?}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3:?}");

    // reference validity
    // references dont outlive the data they point to
    // let x_ref = {
    //     let x = 10;
    //     &x
    // }; // fails at compile time as x does not live beyond this scope

    // exercise: Geometry
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

fn magnitude(v: &[f64; 3]) -> f64 {
    let mut mag_sq: f64 = 0.0;
    for coord in v {
        mag_sq += coord * coord;
    }
    mag_sq.sqrt()
}

fn normalize(v: &mut [f64; 3]) {
    let mag = magnitude(v);
    for coord in v {
        *coord /= mag;
    }
}
