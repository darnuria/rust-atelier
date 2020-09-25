# ⚙🦀 Ressources pour le cours de Programmation Avancée en Rust ⚙🦀

Au travers d'exercices simples on va s'approprier la syntaxe du langage
[Rust](https://www.rust-lang.org/).

Retenez ceçi pour la suite: Toute question est bonne, dans le doute demandez moi.
Rust est un langage complexe donc n'hésitez pas a poser des questions!

## Notes de Cours

Vous pourrez trouver des [notes de cours](cours/readme.md) dans le dossier `cours`, vous pouvez
soumettre les votres par courriel ou directement dans une merge request ou une
issue sur [framagit](https://framagit.org/darnuria/rust-initiation)

## A faire pour la prochaine fois

Pour le mardi 29 septembre ou vendredi 02 octobre selon votre groupe!
Avoir avancé dans rustlings jusque a l'exercice `quiz2` au moins.
laissez vous guider par `rustlings watch`. :)

Avancer dans les exercices jusque au 08 au minimum, idéalement avoir tout fait.
Préparer vos questions pour le prochain cours ou me les envoyer par courriel!

## Installation de Rust et environnement de developpement

Ces instructions sont valide pour votre ordinateur personnel ou à l'université!
Pour juste tester vous pouvez utiliser [l'environnement de brouillon de Rust en ligne](https://play.rust-lang.org/).

## Installer son environnement de travail

### Vscodium

Visual studio mais sans le support de microsoft ni la télémetrie:
<https://github.com/VSCodium/vscodium>

Pensez à lire la procédure d'installation [disponible
ici](https://github.com/VSCodium/vscodium#downloadinstall)

Vous pourrez choisir sous linux le `.AppImage` (pensez à le rendre executable),
ou le `.deb` à installer avec `sudo dpkg -i mon_packet`

Pensez à installer l'addon Rust!

### Vim / Emacs / Spacemacs

Quelques liens pour les personnes qui désireraient utiliser [emacs](https://www.gnu.org/software/emacs/)
avec [spacemacs](https://www.spacemacs.org/) ou [vim](https://www.vim.org/) pour Rust:

* Vim: <https://opensource.com/article/20/7/vim-rust-ide>
* Emacs avec Spacemacs: <https://github.com/rust-lang/rust-mode>

> *Note:* Spacemacs est une configuration rapide a l'emploi pour emacs version
> 25 ou supérieure qui simplifie l'usage de emacs.

### Pycharm - Rust

Si vous préférez les environnements intégrés de developpement je vous recommande [pycharm](https://www.jetbrains.com/pycharm/download/download-thanks.html) avec [l'addon Rust](https://plugins.jetbrains.com/plugin/8182-rust/versions/stable).

## Installation de rust avec Rustup

Pour installer le compilateur Rust Rustc et Cargo, le package manager de Rust
on va utiliser [rustup](https://rustup.rs/):

Vous pouvez l'installer avec la commande suivante:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

> *Note:* Attention! Ce type de commande va télécharger avec le programme curl
> un script puis va l'executer! Soyez sur de ce que vous faites lorsque vous
> installez quelque chose avec cette methodologie.

Ensuite choissisez les choix par défauts.

⚠ Attention ⚠ : Il faut bien penser d'ajouter `source $HOME/.cargo/bin` à la fin de
fichier de configuration local de bash disponnible au chemin `~/.bashrc`.

### Ajouter au chemin les binaires installé par cargo et rustc

Si vous voulez allez vite vous pouvez faire cela va écrire la chaine entre `"`
dans votre fichier `.bashrc`: `echo "source $HOME/.cargo/env" >> ~/.bashrc`

Sinon vous aurez des erreurs du type: `Command 'rustc' not found`

### Vérifier que rust est bien installé

Tentez de faire les commandes suivantes: `rustc --version` et `cargo --version`.

Vous devriez obtenir quelque chose comme :

```bash
$ rustc --version
rustc 1.46.0 (04488afe3 2020-08-24)
```

Et pour cargo le gestionnaire de paquet de l'ecosystème de Rust:

```bash
$ cargo --version
cargo 1.46.0 (149022b1d 2020-07-17)
```

### Rustlings

[Rustlings](https://github.com/rust-lang/rustlings/) est un programme qui permet d'apprendre en autonomie la syntaxe de Rust,
le programme compile à chaque changement pour vos vos fichiers de code et passe
a l'exercice suivant quand vous avez réussi! Pratique pour se concentrer sur
le domptage du compilateur Rust et de ses messages d'erreurs!

Pour l'installer vous pouvez suivre les instructions [ici](https://github.com/rust-lang/rustlings#macoslinux):

Pour la praticité je vous reporte la procédure:

```bash
# Si vous voulez installer ailleurs que dans votre $HOME il suffit de changer
# la ligne après `bash -s`
curl -L https://git.io/rustlings | bash -s $HOME/rustlings
```

Ensuite vous pouvez aller dans le répertoire de rustlings: `cd $HOME/rustlings`,
lancer `rustlings` en tappant `rustlings watch` dans votre terminal.

Ensuite il suffit de se laisser guider d'editer les exercices disponnibles dans:
`~/rustlings/exercices`.

vous devriez quelque chose mais en couleur comme:

```txt
! Compiling of exercises/variables/variables1.rs failed! Please try again. Here's the output:
error[E0425]: cannot find value `x` in this scope
  --> exercises/variables/variables1.rs:12:5
   |
12 |     x = 5;
   |     ^ not found in this scope

error[E0425]: cannot find value `x` in this scope
  --> exercises/variables/variables1.rs:13:36
   |
13 |     println!("x has the value {}", x);
   |                                    ^ not found in this scope

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
```

Une fois la correction faite lisez bien les messages du compilateur on est pas en C!
Vous devriez voir quelque chose comme il est temps de passer au prochain exercice!

```txt
✓ Successfully ran exercises/variables/variables1.rs!
! Compiling of exercises/variables/variables2.rs failed! Please try again. Here's the output:
error[E0282]: type annotations needed
 --> exercises/variables/variables2.rs:7:9
  |
7 |     let x;
  |         ^ consider giving `x` a type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
```

> Astuce: Les chemins que donne Rustlings sont utilisable pour naviguer directement
si vous ouvrez un terminal dans votre editeur/IDE!

### Bonus trèèèèèès recommandé

Si vous ne souhaitez pas vous faire un compte sur exercism vous pouvez pratiquer
en clonant le dépot : <https://github.com/exercism/rust/>

* [Exercism.io](https://exercism.io/tracks/rust) bases d'exercices en licence
libre

### Documentation et références

Beaucoup des ressources seront en anglais, en cours je tenterais de françiser les explications.

* [The Rust programming Language](https://doc.rust-lang.org/book/) Livre d'apprentissage de référence
* [Documentation de la bibliothèque standard](https://doc.rust-lang.org/std/) : Indispensable
* [Rust by exemple](https://doc.rust-lang.org/stable/rust-by-example/) Rust illustré par des exemples

En français vous pourrez aussi trouver le tutoriel de [Guillaume Gomez](https://github.com/GuillaumeGomez/), avoir une seconde façon ou troisième façon d'approcher un problème peut vous débloquer: <https://blog.guillaume-gomez.fr/Rust>

### Environnement de developpement programmes utiles

Pour faciliter l'édition il existe des utilitaires utilisable entre tout les editeurs, comme
l'analyseur de Rust:

`rustup component add rust-analysis`

Puis le serveur de compilation pour l'editeur de code:

`rustup component add rls`

Il en existe d'autre nous le verrons plus tard.
