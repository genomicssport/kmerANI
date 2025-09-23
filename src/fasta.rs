use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-29
*/

#[derive(Debug)]
pub struct FastaRecord {
    pub id: String,
    pub sequence: String,
}

pub fn read_fasta<P: AsRef<Path>>(path: P) -> io::Result<HashMap<String, FastaRecord>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut records = HashMap::new();
    let mut current_id = String::new();
    let mut current_sequence = String::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.insert(
                    current_id.clone(),
                    FastaRecord {
                        id: current_id.clone(),
                        sequence: current_sequence.clone(),
                    },
                );
                current_sequence.clear();
            }
            current_id = line[1..].to_string();
        } else {
            current_sequence.push_str(&line);
        }
    }

    if !current_id.is_empty() {
        records.insert(
            current_id.clone(),
            FastaRecord {
                id: current_id,
                sequence: current_sequence,
            },
        );
    }

    Ok(records)
}
