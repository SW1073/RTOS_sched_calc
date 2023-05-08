# SCHEDULER CLI

## Usage
En primer lloc, és necesari tenir una versió actualitzada del compilador de [Rust](https://www.rust-lang.org/es/tools/install).

Un cop instalada, descarreguem o clonem el repositori. Dintre del repositori i de la carpeta scheduler, llancem la comanda ```cargo run```. Això possarà en marxa la aplicació amb les optimitzacions més baixes i amb informació de debugat. Si es dessitja i prefereix, es pot user la comanda ```cargo run --release``` per a optimitzar el binari resultant.

En cas de voler fer input desde fitxers al programa, ho podem fer de la manera estàndard, amb ```cargo run < path/to/input_file``` a bash, i amb ```cat path/to/input_file | cargo run``` per a PowerShell a Window per a PowerShell a Windows.
