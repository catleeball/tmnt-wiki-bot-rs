use std::path::Path;

mod cmu_dict;

fn main() {
    // TODO: get these from flags or a config file
    let cmu_dict_file = Path::new("data/cmudict-0.7b");

    // TODO: Serialize to file, then check for serialized file on startup.
    let dict = cmu_dict::cmu_dict_file_to_map(cmu_dict_file);
    let mut i = 0;
    for (word, vec) in dict {
        if i >= 10 {
            break
        }
        print!("{word}: ");
        for p in vec {
            print!("{p}, ");
        }
        println!();
        i += 1;
    }
}
