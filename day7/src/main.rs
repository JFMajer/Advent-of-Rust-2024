pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}


impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        for log_line in self.logs.iter() {
            if log_line.contains(keyword) {
                result.push(&log_line);
            }
        }
        result
    }
}

