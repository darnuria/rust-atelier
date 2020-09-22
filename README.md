# Ressources pour le cours de Programmation Avancée

Au travers d'exercices simples on va s'approprier la syntaxe du langage [Rust](https://www.rust-lang.org/).

Retenez ceçi pour la suite: Toute question est bonne, dans le doute demandez moi.
Rust est un langage complexe donc n'hésitez pas a poser des questions!

## Ressources utiles

- [rustlings](https://github.com/rust-lang/rustlings/) exercices très simples
- [Exercism.io](https://exercism.io/tracks/rust) bases d'exercices en licence
libre

### Documentation et références

Beaucoup des ressources seront en anglais, en cours je tenterais de françiser les explications.

- [The Rust programming Language](https://doc.rust-lang.org/book/) Livre d'apprentissage de référence
- [Documentation de la bibliothèque standard](https://doc.rust-lang.org/std/) : Indispensable
- [Rust by exemple](https://doc.rust-lang.org/stable/rust-by-example/) Rust illustré par des exemples

En français vous pourrez aussi trouver le tutoriel de [Guillaume Gomez](https://github.com/GuillaumeGomez/), avoir une seconde façon ou troisième façon d'approcher un problème peut vous débloquer: <https://blog.guillaume-gomez.fr/Rust>

## Installation de Rust et environnement de developpement

Ces instructions sont valide pour votre ordinateur personnel ou à l'université!
Pour juste tester vous pouvez utiliser [l'environnement de brouillon de Rust en ligne](https://play.rust-lang.org/).

### Installation de rust avec Rustup

Pour installer le compilateur Rust Rustc et Cargo, le package manager de Rust on va utiliser [rustup](https://rustup.rs/):

Vous pouvez l'installer avec la commande suivante:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

> Note: Attention! Ce type de commande va télécharger avec le programme curl
> un script puis va l'executer! Soyez sur de ce que vous faites lorsque vous
> installez quelque chose avec cette methodologie.

Ensuite choissisez les choix par défauts.

### Environnement de developpement

Pour faciliter l'édition il existe des utilitaires utilisable entre tout les editeurs, comme
l'analyseur de Rust:

`rustup component add rust-analysis`

Puis le serveur de compilation pour l'editeur de code:

`rustup component add rls`

Il en existe d'autre nous le verrons plus tard.
