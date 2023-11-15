Per chiunque voglia interfacciarsi alla programmazione Rust deve ricordare bene che qualsiasi cosa fatta al terminale è molto più semplice e intuitiva per chiunque voglia programmare in Rust. Usare programmi come Visual Studio ecc. dovrebbe servire solo per analizzare e debuggare il funzionamento del programma e/o da considerarsi solo come "editing" del file con supporti. Nel caso di visual studio... Le "estensioni renderanno migliore l'esperienza di editing del codice.

Fatta questa premessa. Rustup non è da usare grazie all'esistenza di Cargo e del suo ".toml".

Ricordate che il .toml è l'archivio del linguaggio rust per usare tutto ciò che serve nel file e compilatore.

Quindi. Per programmare in Rust si consiglia l'uso del terminale, soprattutto se siamo su Windows 10! 

Quindi bisogna fare tutto a mano per .exe e da noi faremo il .toml? SBAGLIATO! Useremo sempre !Cargo!

Bisogna bene realizzare che i file sono "cartelle" contenente il file.rs, il Cargo.toml e il file.exe dopo la compilazione.

Quindi vogliamo chiamare il programma "hello"?.

Andiamo nella cartella dove vogliamo inserire il file. Quindi installato rustup e tutto chiamiamo:

Cargo new <nomefile>

ci spostiamo nella cartella tramite "cd <cartella>" e guardando i file con "dir"

a questo punto siete liberi di editare il file "Cargo.toml" o nella cartella "src" troverete "main.rs".

se volete usare come editor "Visual Studio", allora bastera fare dentro la cartella <nomefile>: code . (e premere invio)

Se volete aggiungere librerie come rand. basterà entrare in Cargo.toml e modificare le dipendenze in base ad i consigli del "Cargo Book" che troverete su internet (trovare link)

finito di editare, CONSIGLIO di non usare il debugger o il run di Visual Studio, bensì, se si è sicuri di aver scritto bene il codice... Usare sempre Cargo manualmente. Come?

entriamo sempre tramite prompt/terminale, dentro la cartella "src" e fare "cargo run"

se non ci saranno errori il compilatore finirà e si creerà la cartella "target", in cui ci sarà la cartella di debug e di conseguenza il file.exe

se tutto funzionerà correttamente, Cargo, oltre che compilare, eseguirà il programma

se vogliamo solo compilare allora faremo: "Cargo build"

se vogliamo invece eseguirlo: "./target/debug/<nomefile>"

!!!! DA NOTARE CHE IL FILE ESEGUIBILE AVRA' IL NOME DELLA CARTELLA, NON DI "MAIN"!!!!