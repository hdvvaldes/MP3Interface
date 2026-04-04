
## Backend

Language: **Rust**

Files Description:
- data_miner.rs: ADS to model deciphering the information off a mp3 files directory.
- song_repo.rs: Interface with the information. (In this case off of a database).

**General Purpose:**
- file_manager.rs: Write to files and read files.
- data_types.rs: Types declaration. 
    - Song 
    - Artist 
    - Album


```mermaid
classDiagram
    class DataMiner {
        +mine(directory: String) Song[]
    }
    class SongRepo {
        +save(song: Song)
        +get_all() Song[]
    }
    class FileManager {
        +read(path: String) String
        +write(path: String, content: String)
    }
    class DataTypes {
        +Song
        +Artist
        +Album
    }
    class Controller {
        +find_songs(filter: func)
        +add_songs(songs: iterable)
        +delete_songs(songs: iterable)
    }

    DataMiner ..> DataTypes
    SongRepo ..> DataTypes
    Controller ..> SongRepo
    Frontend ..> Controller
```

## API

Language: Python

```mermaid
classDiagram
    class Controller {
        +add_songs(songs: iterable)
        +get_songs(filter: func)
        +update_song(song: Song)
        +delete_songs(songs: iterable)
    }
```

## FrontEnd

Language: Flutter

```mermaid
classDiagram
    class View{
        +display_welcome()
        +display_prompt()
        +display_error(message: String)
        +display_song(song: Song)
        +display_player()
    }
    class Controller{
          
    }

```

