use walkdir::{WalkDir};
use dirs;


pub mod get_files{

    use super::*;

    pub fn get_mp3_files() -> Vec<String> {
        let mut file_list:Vec<String> = Vec::new();

        let home_dir = dirs::home_dir().expect("Failed to read dir").into_os_string().into_string().unwrap();

        for file in WalkDir::new(&home_dir).into_iter().filter_map(|file| file.ok()){
            if file.path().display().to_string().contains(".mp3"){
                 file_list.push(file.path().display().to_string());
                // file_list.push(create_song_data(&file.path().display().to_string().to_string()));
            }
        }

        file_list
    }


    
            
    }

   
