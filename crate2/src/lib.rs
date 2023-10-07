pub fn add(left: usize, right: usize) -> usize {
    let hello = "Hello, world!";
    println!("Ethan says: {}", hello);
    eprintln!("Customizing consensus parameters for chain spec only works for dev chains.");

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
