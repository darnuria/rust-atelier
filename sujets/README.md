# Instructons pour compiler les fichiers latex

Minted est utilisé pour les exemples de code, il faut l'installer et installer
l'application pygmentize python ensuite faire la commande:

```sh
latexmk -shell-escape  -pdf mon_latex.tex
```
