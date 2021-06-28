use std::collections::HashMap;

// https://github.com/simplestaking/tezedge/blob/master/rpc/src/server/mod.rs#L158

/// Helper for parsing URI queries.
/// Functions takes URI query in format `key1=val1&key1=val2&key2=val3`
/// and produces map `{ key1: [val1, val2], key2: [val3] }`
fn parse(query: &str) -> HashMap<String, Vec<String>> {
    let mut ret: HashMap<String, Vec<String>> = HashMap::new();
    for (key, value) in query.split('&').map(|x| {
        let mut parts = x.split('=');
        (parts.next().unwrap(), parts.next().unwrap_or(""))
    }) {
        if let Some(vals) = ret.get_mut(key) {
            // append value to existing vector
            vals.push(value.to_string());
        } else {
            // create new vector with a single value
            ret.insert(key.to_string(), vec![value.to_string()]);
        }
    }
    ret
}

pub fn parse_query_string(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(&data) {
        let _ = parse(&s);
    }
}

#[cfg(test)]
mod test {
    use no_fuzz::no_fuzz;

    no_fuzz!(parse_query_string, crate::parse_query_string);

}
