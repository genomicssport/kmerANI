use crate::fasta::read_fasta;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-9-23
*/

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FastaStruct {
    id: String,
    seq: String,
}

#[tokio::main]
pub async fn kmerestimate<'a>(filepath: &str, kmer: &str) -> Result<String, Box<dyn Error>> {
    let fasta_records = read_fasta(filepath).unwrap();
    let mut genomevec: Vec<FastaStruct> = Vec::new();
    for (_id, record) in fasta_records {
        genomevec.push(FastaStruct {
            id: record.id.clone().to_string(),
            seq: record.sequence.clone(),
        })
    }

    let mut tokensize: Vec<(String, Vec<&str>)> = Vec::new();
    for i in genomevec.iter() {
        let seqvec: Vec<&str> = i
            .seq
            .as_bytes()
            .windows(kmer.parse::<usize>().unwrap())
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        let vecinput: (String, Vec<&str>) = (i.id.clone(), seqvec);
        tokensize.push(vecinput);
    }

    // building a uniquekmer from all the sequences asynchronous

    let mut uniquekmer: HashSet<_> = HashSet::new();
    for i in tokensize.iter() {
        for seqiter in i.1.iter() {
            uniquekmer.insert(seqiter);
        }
    }

    // compairing the uniqueness based on the kmer and storing it as a BtreeMap

    let mut hashbtree: BTreeMap<String, Vec<(String, f32)>> = BTreeMap::new();
    for i in tokensize.iter() {
        let iseq = i.1.clone();
        let mut count = 0usize;
        let mut vecseq: Vec<(String, f32)> = Vec::new();
        for j in uniquekmer.iter() {
            for seqiter in iseq.iter() {
                if seqiter == *j {
                    count += 1;
                }
            }
        }
        vecseq.push((i.0.clone(), count as f32));
        hashbtree.insert(i.0.clone(), vecseq);
    }

    let mut finalhashmap: Box<HashMap<String, (String, Vec<(String, f32)>)>> =
        Box::new(HashMap::new());
    for i in genomevec.iter() {
        for j in hashbtree.iter() {
            if i.id.clone() == j.0.clone() {
                finalhashmap.insert(i.id.clone(), (i.seq.clone(), j.1.to_vec()));
            }
        }
    }

    let mut filewrite = File::create("identitymatrix.txt").expect("file not found");
    for i in hashbtree.iter() {
        let unpack = i.1;
        writeln!(filewrite, "{}\t{:?}", i.0, unpack[1]).expect("i not found");
    }

    Ok("value has been written".to_string())
}
