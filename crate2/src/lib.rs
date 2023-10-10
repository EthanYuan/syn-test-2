pub fn add(left: usize, right: usize) -> usize {
    let hello = "Hello, world!";
    println!("Ethan says: {}", hello);
    eprintln!("Customizing consensus parameters for chain spec only works for dev chains.");
    eprintln!(
        "For optimal performance, CKB wants to migrate the data into new format.\n\
        You can use the old version CKB if you don't want to do the migration.\n\
        We strongly recommended you to use the latest stable version of CKB, \
        since the old versions may have unfixed vulnerabilities.\n\
        Run `\"{}\" migrate -C \"{}\"` and confirm by typing \"YES\" to migrate the data.\n\
        We strongly recommend that you backup the data directory before migration.",
        "bin_name", "root_dir"
    );

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
