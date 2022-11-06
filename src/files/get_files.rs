use dirs;
use walkdir::WalkDir;

pub fn get_mp3_files() -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();

    let home_dir = dirs::home_dir()
        .expect("Failed to read dir")
        .into_os_string()
        .into_string()
        .unwrap();

    for file in WalkDir::new(&home_dir)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.path().display().to_string().ends_with(".mp3") {
            file_list.push(file.path().display().to_string());
        }
    }

    file_list
}
