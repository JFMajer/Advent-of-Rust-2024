use std::{fs::File, io::Write};

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁
        let logs = self.search(keyword);
        if logs.is_empty() {
           return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Logs are empty, nothing to write"));
        }
        let mut file = File::create(path)?;
        for line in logs.iter() {
            file.write_all(line.as_bytes())?;
            file.write_all("\n".as_bytes() )?;
        }
        Ok(())    
    }
}

fn main() {
    // Sample logs
    let logs = vec![
        "User login at 10:00 AM".to_string(),
        "Error: Something bad".to_string(),
        "User login at 11:00 AM".to_string(),
        "Error: Another issue".to_string(),
        "User login at 12:00 PM".to_string(),
    ];

    let log_query = LogQuery::new(&logs);

    // Test export to file with the keyword "User login"
    match log_query.export_to_file("User login", "user_log.txt") {
        Ok(_) => println!("Logs successfully exported to 'user_log.txt'"),
        Err(e) => eprintln!("Failed to export logs: {}", e),
    }

    // Test export to file with a keyword that doesn't exist
    match log_query.export_to_file("Admin login", "empty_log.txt") {
        Ok(_) => println!("Logs successfully exported to 'empty_log.txt'"),
        Err(e) => eprintln!("Failed to export logs: {}", e),
    }

    // Test export to a non-existent file path to trigger an error
    match log_query.export_to_file("Error", "/nonexistent/folder/error_log.txt") {
        Ok(_) => println!("Logs successfully exported to '/nonexistent/folder/error_log.txt'"),
        Err(e) => eprintln!("Failed to export logs: {}", e),
    }
}