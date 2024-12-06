// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str>  {
    let s1_trimmed = s1.trim();
    let s2_trimmed = s2.trim();

    let s1_count = s1_trimmed.chars().count();
    let s2_count = s2_trimmed.chars().count();
    
    if s1_count > s2_count {
        Some(s1)
    } else if s1_count == s2_count {
        None
    } else {
        Some(s2)
    }
}

