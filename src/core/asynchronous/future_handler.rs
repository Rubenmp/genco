use std::future::Future;

pub(crate) fn wait<T>(async_call: impl Future<Output = T>) -> T {
    let rt = tokio::runtime::Runtime::new().expect("Error trying to wait for asynchronous task");
    
    rt.block_on(async_call)
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use crate::core::asynchronous::future_handler::wait;

    #[test]
    fn wait_fake_async() {
        let async_call = async { 1 };

        let result = wait(async_call);

        assert_eq!(1, result);
    }

    #[test]
    fn wait_async() {
        let async_call = async {
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
            1
        };

        let result = wait(async_call);

        assert_eq!(1, result);
    }
}
