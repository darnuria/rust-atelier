# Projet PaintDroid

/!\ Attention sujet non fini!!! prevoir des changements /!\

Dans ce projet nous allons réaliser un jeu joué par l'ordinateur dans
un premier temps depuis un fichier predefini.

## Deplacement orientation

Dans ce jeu des robots vont pouvoir se déplacer dans un espace en deux
dimensions. A chaque tour un robot executera un ordre les ordres concistent en
effectuer une rotation à droite ou à gauche ou avancer ou ne rien faire!

> Conseil: Deux type `enum` vous serons utile! ;)

Dans le fichier d'instructions vous aurrez les caractères suivants possibles:

- L: Oriente le robot de 90 degrée à gauche par exemple passe de orientation: N à W
- R: Oriente le robot de 90 degrée à droite par exemple passe de orientation: N à E
- F: Avance dans le sens de l'orientation d'une case

<!--

TODO: coloration du sol.
## Coloration du terrain

Dans un second temps nos robots gagneront l'instruction qui leur permettra de
posser une couleur au sol et cette couleur au sol restera pour 5 tours et
si un autre robot passe dans cette couleur alors il est eliminé.

Un peu comme dans les "tron" games.
Exemple vidéo: https://www.youtube.com/watch?v=PWvxGX2twcA
-->

## Collisions

En cas de collision avec un autre robot lors d'un déplacement le robot devra
dire sur la sortie standard:

Cas des collisions: Faire dire `"Robot ID<numId> Collision en (x, y)"`

Format du fichier definisant le monde:

```txt
5 5   // X_max Y_max
1 1 N // position du robot en x=1 y=1 orientation = nord
FLLFRF
```

Pour representer:

- un robot une simple structure suffira.
- La structure pour contenir les robots pourras être un `vec<Robot>`

Programmer ce qui est neccessaire pour que a chaque *tick* de façon aléatoire un
robot avance sa liste d'instruction serait symbolisée par simplement aucune liste
d'instructions.

## version 0.4.0 : un peu de couleurs

A présent les robots lorsque ils se déplacent laissent une couleur sur les cases
qu'ils ont traversée, les couleurs sont determinées à partir de l'id d'un robot.

Les couleurs seront des couleurs affichables en terminal, à vous d'ecrire une
fonction qui va des id vers les couleurs. indice: fonction de hashage, trait `Hash`.

Si un robot traverse la couleur d'un autre robot il est mis hors service, il est
hors jeu pour le reste de la partie et sera symbolisé par `'🤖'` par exemple il
reste un element dans lequel les autres robots peuvent entrer en collision.

La collision entre deux robots n'implique pas de mise hors service.

Programmer ce qui est neccessaire pour que a chaque *tick* de façon aléatoire un
robot avance sa liste d'instruction serait symbolisée par simplement aucune liste
d'instructions.

Pour vous aider avec les couleurs vous pouvez utiliser la crate [termion](https://lib.rs/crates/termion)
ou [colored](https://crates.io/crates/colored).
