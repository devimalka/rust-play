use walkdir::WalkDir;

pub mod get_files{

    use super::*;

    pub fn get_mp3_files() -> Vec<String> {
        let mut file_list:Vec<String> = Vec::new();

        for file in WalkDir::new("/home/imalka").into_iter().filter_map(|file| file.ok()){
            if file.path().display().to_string().contains(".mp3"){
                file_list.push(file.path().display().to_string());
            }
        }

        file_list
    }
}
