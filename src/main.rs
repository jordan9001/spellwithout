use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        Error,
        Write,
    },
    vec::Vec,
    string::String,
    collections::HashMap,
};

fn make_map_file(goodwords: &str, ipa: &str) -> Result<(), Error> {
    let wrd_file = File::open(&goodwords)?;
    let mut words: Vec<String> = Vec::new();
    for line in BufReader::new(wrd_file).lines() {
        // add to our list
        // we don't need a fancy graph here
        // even if we can have multiple levels deep
        // for now we can just try it the simple way

        // check if it is a long enough word
        //TODO other checks?

        if let Ok(l) = line {
            if l.len() > 3 {
                words.push(l.to_lowercase());
            }
        }
    }

    println!("{} words", words.len());

    let ipa_file = File::open(&ipa)?;
    let mut ipamap: HashMap<String, String> = HashMap::new();
    for line in BufReader::new(ipa_file).lines() {
        if let Ok(l) = line {
            // split into two
            let sl: Vec<&str> = l.split_whitespace().collect();
            if sl.len() != 2 {
                // these words have multiple pronunciations
                // maybe skip these for now
                //println!("{}", l);
                continue;
            }

            if words.contains(&String::from(sl[0])) {
                ipamap.insert(String::from(sl[0]), String::from(sl[1]));
            }
        }
    }

    println!("{} ipas", ipamap.len());

    let mut out_file = File::create("./out_ipa.txt")?;

    for (key, val) in ipamap.iter() {
        out_file.write(key.as_bytes())?;
        out_file.write(b" ")?;
        out_file.write(val.as_bytes())?;
        out_file.write(b"\n")?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 || args.len() > 3 {
        println!("Usage: xwy [<file with acceptible words>] <file with IPA>");
        return Err(Error::other("Not enough arguments"));
    }

    // if 3 then get an updated file
    // otherwise expect an updated file
    if args.len() == 3 {
        return make_map_file(&args[1], &args[2]);
    }


    let ipa_file = File::open(&args[1])?;
    let mut ipamap: HashMap<String, String> = HashMap::new();
    for line in BufReader::new(ipa_file).lines() {
        if let Ok(l) = line {
            // split into two
            let sl: Vec<&str> = l.split_whitespace().collect();

            // build a simplified ipa
            let newipa = sl[1].replace(&['/', '.', ',', '\'', '`', '.', 'ˈ', 'ˌ'][..], "");
            ipamap.insert(String::from(sl[0]), newipa);
        }
    }

    println!("{} ipas", ipamap.len());

    for (key_i, val_i) in ipamap.iter() {
        for (key_j, val_j) in ipamap.iter() {
            if key_i == key_j {
                continue;
            }

            if key_i.len() == key_j.len() {
                continue;
            }

            // should be in the middle
            if key_i.starts_with(key_j) || key_i.ends_with(key_j) {
                continue;
            }

            //TODO is there a nice way to get a fuzzy match here?

            if key_i.contains(key_j) && !val_i.contains(val_j) {
                println!("You can't spell {key_i} without {key_j}");
            }
        }
    }

    Ok(())
}
