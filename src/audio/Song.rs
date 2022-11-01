use audiotags::Tag;


pub struct SongStruct{
   pub title:String,
   pub artist:String,
   pub file_path:String,
}


pub fn create_song_struct(file_path:&str) -> SongStruct{

    let tags = Tag::default().read_from_path(&file_path);
  
  
    let binding = tags.expect("Unable to find Tags");
    let title = binding.title().unwrap_or("Title Not Found");
    let artist = binding.artist().unwrap_or("Artist Not Found");
  
   
   
     SongStruct{
      file_path:file_path.to_string(),
      title:title.to_string(),
      artist:artist.to_string(),
    }
    
    }