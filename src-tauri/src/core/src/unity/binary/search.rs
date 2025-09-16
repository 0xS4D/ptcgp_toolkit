use memchr::memchr_iter;

pub fn find_pattern(data: &[u8], needle: &[u8]) -> Vec<usize> {
    let mut results = Vec::new();

    if needle.len() == 1 {
        for candidate in memchr_iter(needle[0], data) {
            results.push(candidate);
        }
    } else {
        for candidate in memchr_iter(needle[0], data) {
            if candidate + needle.len() <= data.len()
                && &data[candidate..candidate + needle.len()] == needle
            {
                results.push(candidate);
            }
        }
    }
    results
}
