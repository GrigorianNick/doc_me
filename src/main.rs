use clap::Parser;
use serde::{Deserialize, Serialize};

use std::collections::{HashSet, HashMap};
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_delimiter = ' ', num_args = 2.., value_names(["Root", "Other"]))]
    map: Option<Vec<String>>,

    #[arg(short, long, default_value("develop"))]
    branch: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Mapping {

    file_mapping: HashMap<String, HashSet<String>>
}

impl Mapping {
    pub fn new(file_path: String) -> Mapping {
        if let Ok(file) = std::fs::File::open(&file_path) {
            serde_json::from_reader(file).expect("Failed to parse file")
        } else {
            let map = Mapping::default();
            map.save(file_path);
            map
        }
    }

    pub fn save(&self, file_path: String) {
        let target_file = std::fs::File::create(file_path).expect("Failed to open file");
        serde_json::to_writer_pretty(target_file, self).expect("Failed to save Mapping");
    }

    pub fn update(&mut self, save_file: String, files: Vec<String>) {
        let root = &files[0];
        if let Some(deps) = self.file_mapping.get_mut(root) {
            for file in files.iter().skip(1) {
                deps.insert(file.clone());
            }
        } else {
            self.file_mapping.insert(root.clone(), HashSet::from_iter(files.iter().skip(1).cloned()));
        }
        self.save(save_file);
    }
}

fn main() -> Result<(), u32> {
    let args = Args::parse();
    let mut mapping = Mapping::new(String::from("./doc_me_map.json"));
    
    if let Some(map) = args.map {
        mapping.update(String::from("./doc_me_map.json"), map);
        return Ok(());
    }

    let output = Command::new("cmd")
        .args(["/C", [String::from("git diff --name-only"), args.branch].join(" ").as_str()])
        .output()
        .expect("Failed to execute process");
    let diff_output = String::from_utf8_lossy(&output.stdout).to_string();
    let changed_files: Vec<String> = diff_output.split_ascii_whitespace().map(|s| s.to_string()).collect();
    let file_set: HashSet<String> = HashSet::from_iter(changed_files.iter().cloned());

    let mut missing_deps: u32 = 0;

    for changed_file in &changed_files {
        if let Some(deps) = mapping.file_mapping.get(changed_file) {
            if !deps.is_subset(&file_set) {
                let diff = deps.difference(&file_set).map(|d| d.to_string()).collect::<Vec<String>>();
                println!("Missing update:{}->{:?}", changed_file, diff);
                missing_deps = missing_deps.saturating_add(diff.len() as u32);
            }
        }
    };
    if missing_deps == 0 {
        Ok(())
    } else {
        Err(missing_deps)
    }
}
