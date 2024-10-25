#[macro_export]
macro_rules! check_token {
    ($status:expr, $cancelled_status:expr, $token:expr, $app:expr) => {
        if $token.is_cancelled() {
            $app.emit($status, $cancelled_status).unwrap();
            return Ok(());
        }
    };
}
