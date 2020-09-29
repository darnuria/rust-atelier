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

<!--

# Améliorations :


Debug des collisions:

`[ID du robot] BeBop!? Collision with <Id du robot collisionné>`
-->