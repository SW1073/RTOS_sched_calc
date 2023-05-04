# SCHEDULER CLI

## Usage
En primer lloc, és necesari tenir una versió actualitzada del compilador de ![Rust](https://www.rust-lang.org/es/tools/install).

Un cop instalada, descarreguem o clonem el repositori. Dintre del repositori i de la carpeta scheduler, llancem la comanda ```cargo run```. Això possarà en marxa la aplicació amb les optimitzacions més baixes i amb informació de debugat. Si es dessitja i prefereix, es pot user la comanda ```cargo run --release``` per a optimitzar el binari resultant.

En cas de voler fer input desde fitxers al programa, ho podem fer de la manera estàndard, amb ```cargo run < path/to/input_file``` a bash, i amb ```cat path/to/input_file | cargo run``` per a PowerShell a Window per a PowerShell a Windows.

## Estructura del codi
Existeixen 4 planificadors dintre d'aquest sistema:
 - Cíclic
 - Rate Monotonic
 - Deadline Monotonic
 - Earliest Deadline First

S'han definit una sèrie de _traits_ que ens permeten fer que tots els planificadors hagin de tenir una certa interfície implementada. Aquestos traits son:
 - AddNewTaks
    - Demana a l'scheduler implementar la funció ```add_task()```, que inclou una nova task al planificador i comprova la correctesa del paràmetres d'aquesta.
 - CheckSchedulable
   - Demana al scheduler implementar la funció ```is_schedulable()```, que retorna si el planificador és planificable o no, i un log del proces.
 - SchedulerInterface
   - Supertrait que inclou els dos anteriors. Ens permet crear trait objects que asseguren que tot scheduler implementa les dues funcionalitats anteriors.

```lib.rs``` és el punt més proper a l'usuari, on realment es decideix que veu la interfície i que no. Des d'aquí, exposem a l'usuari tot el módul d'schedulers i el log, per a poder llegir i imprimir els resultats. A més declarem que existeix un módul anomenat task per a que tota la resta de móduls de dins de la aplicació puguin accedir, però no l'usuari. 

# TODO(...)