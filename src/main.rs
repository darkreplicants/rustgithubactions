fn main() {
    println!("Hello, world!");
    cuadrado(5);
}

fn cuadrado(num: i32) -> i32{
    num*num
}


// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_is_the_square_of_four() {
        assert_eq!(16, cuadrado(4));
    }

    #[test]
    fn six_is_the_square_of_four() {
        assert_eq!(6, cuadrado(4));
    }

}

