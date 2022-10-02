// Copyright 2022 nitepone <luna@night.horse>

fn main() {
    println!("Hi friend!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
