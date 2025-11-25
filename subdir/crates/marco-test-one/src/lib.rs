pub fn hello_world() {
    println!("Hi, worldddd!!!!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 3;
        assert_eq!(result, 5);
    }
}
