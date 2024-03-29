/// Create an empty vector
#[must_use]
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
#[must_use]
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
#[must_use]
pub fn fibonacci() -> Vec<u8> {
    let mut vec = create_buffer(5);
    for idx in 0..vec.len() {
        if idx < 2 {
            vec[idx] = 1;
        } else {
            vec[idx] = vec[idx - 1] + vec[idx - 2];
        }
    }
    vec
}
