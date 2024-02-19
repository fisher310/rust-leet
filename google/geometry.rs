// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude<T>(vec: T) -> f64
where
    T: Sized + AsRef<[f64]>,
{
    vec.as_ref()
        .iter()
        .fold(0.0, |sum, x| sum + x.powi(2))
        .sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize<T>(mut vec: T)
where
    T: Sized + AsMut<[f64]> + AsRef<[f64]>,
{
    let magnitude = magnitude(vec.as_ref());
    for x in vec.as_mut() {
        *x /= magnitude
    }
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude([0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(v));

    println!("-------------------------------------");

    println!(
        "Magnitude of a unit vector: {}",
        magnitude(vec![0.0, 1.0, 0.0])
    );

    let mut v = vec![1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
