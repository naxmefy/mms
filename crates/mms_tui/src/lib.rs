#[derive(Debug)]
pub struct MMSTuiApplication {}

impl MMSTuiApplication {
    pub fn new() -> Result<MMSTuiApplication, ()> {
        Ok(MMSTuiApplication {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_without_params() {
        let result = MMSTuiApplication::new();
        assert!(result.is_ok(), "should initialize with new");
    }
}
