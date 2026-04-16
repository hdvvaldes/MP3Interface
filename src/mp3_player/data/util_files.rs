
use std::path::Path;

/*
 * Performs breath first search over a directory.
 * Returns inner files with NO directories or reason why failed.
 *
 */


pub fn bfs_directory(root_dir: &String) -> Result< Vec<String>,String>{
    let path = Path::new(root_dir);
    if !path.is_dir() {
        return Err(!format!("Not a directory: {}", root_dir))
    }
    let mut files = Vec::new();
    let mut aux_dire= Vec::new();

    for entry in path.read_dir()? {
        match entry { 
            Ok(dir) => files.push(dir), 
            Err(_) => 
                return 
        }
    }
    Ok(files)

}

