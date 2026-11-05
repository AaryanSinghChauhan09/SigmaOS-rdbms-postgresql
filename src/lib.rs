#[derive(Debug, Clone)]
pub struct PostgresConn { pub conn_string: String }
impl PostgresConn {
    pub fn new(s: impl Into<String>) -> Self { Self { conn_string: s.into() } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_pg() {
        let c = PostgresConn::new("postgres://localhost/test");
        assert!(c.conn_string.starts_with("postgres://"));
    }
}
