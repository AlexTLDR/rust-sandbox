pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_double() {
        assert_eq!(double(3).await, 6);
    }
}
