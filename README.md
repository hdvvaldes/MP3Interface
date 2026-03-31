### Especificaciones del proyecto
El proyecto 2 se entregará el 24 de abril de este año. 

1. Una base de datos, que les mandamos el esquema de la misma aquí en este correo.
2. Un minero, que puede recorrer un directorio y analizar archivos MP3 con etiquetas Id3v2.4[1,2,3] y agregue esa información a la base de datos de arriba.
3. Una interfaz gráfica (de nuevo, como a la que se adjunta), donde puedan modificar  ciertos datos de la base de ídem (también se adjunta un ejemplo).
4. Un "compilador", muy sencillo, que les permita buscar por los distintos campos de la base de datos, sin necesidad de que el usuario escriba SQL.
5. Punto extra si además pueden reproducir los archivos MP3.



## Here is the project 2

# Plans: 

1. Rust - **backend**
2. Python - **middle part**
3. Tauri - **frontend**

┌─────────────────────────────────┐
│   Frontend (TypeScript/React)   │  ← UI, search bar, playlist, waveform
│   hosted in Tauri web view      │
├─────────────────────────────────┤
│     IPC / REST boundary         │  ← JSON messages, commands
├─────────────────────────────────┤
│   Rust Core                     │  ← tags, SQLite, audio engine
│   (id3 + sqlx + rodio + axum)   │
└─────────────────────────────────┘

