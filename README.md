# Sunny Side ☀️
My dream rust ide (wip)

## Features
* Move cursors with arrow keys (with wrapping!)
* Delete/insert text
* Dont worry if you mess it up, you cant save! (also definitly a feature)
* File menu side bar to choose with file to edit

## Usage
Press tab to switch focus bettween text erea and file menu. When if file menu type in the number of the file you want to use and press enter to open it.

## Parts
### Pane
For layout I am using a custom layout system and tui renderer. Each pane needs a render function that take in a canvas for it to draw too and an event function to propogate events throught the tree. For now Im using termion events to handle user input and pass it throught.
