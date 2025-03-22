pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    if ptr.is_null() || len == 0 {
        return 1;
    }
    let mut product = 1;
    for i in 0..len {
        product *= *ptr.add(i);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}

fn main() {
    println!("Hello, world!");
}