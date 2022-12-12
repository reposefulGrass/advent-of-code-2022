
use std::collections::HashSet;
use std::hash::Hash;

pub struct ElfPacket(String);

impl ElfPacket {
    pub fn new(datastream: &str) -> Self {
        ElfPacket(String::from(datastream))
    }

    pub fn find_start_of_packet_marker(&self) -> Option<usize> {
        let chars: Vec<char> = self.0.chars().collect();
        for (index, window) in chars.windows(4).enumerate() {
            if has_unique_elements(window) {
                return Some(index + 4);
            }
        }

        None
    }

    pub fn find_start_of_message_marker(&self) -> Option<usize> {
        let chars: Vec<char> = self.0.chars().collect();
        for (index, window) in chars.windows(14).enumerate() {
            if has_unique_elements(window) {
                return Some(index + 14);
            }
        }

        None
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_of_packet_marker() {
        let ep = ElfPacket::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(ep.find_start_of_packet_marker(), Some(7));
    }

    #[test]
    fn test_find_start_of_message_marker() {
        let ep = ElfPacket::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(ep.find_start_of_message_marker(), Some(19));
    }
}