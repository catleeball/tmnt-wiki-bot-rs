use std::path::Path;


mod cmu_dict;
mod wiki;




fn main() {
    // TODO: get path from flags or a config file

    let cmu_dict_file = Path::new("data/cmudict-0.7b");
    let dict = cmu_dict::cmu_dict_file_to_map(cmu_dict_file);

    let titles = wiki::get_wiki_titles(100, 5);

    println!("{}", titles.join(",\n"));
    // TODO: Serialize to file, then check for serialized file on startup.
    // let dict = cmu_dict::cmu_dict_file_to_map(cmu_dict_file);
    // let mut i = 0;
    // for (word, vec) in dict {
    //     if i >= 10 {
    //         break
    //     }
    //     print!("{word}: ");
    //     for p in vec {
    //         print!("{p}, ");
    //     }
    //     println!();
    //     i += 1;
    // }
}
