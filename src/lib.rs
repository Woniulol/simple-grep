pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    let query_lower_case = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower_case) {
            res.push(line);
        }
    }

    res
}
