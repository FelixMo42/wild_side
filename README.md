# Sunny Side ☀️
My dream rust ide (wip)

## Features
* Move cursors with arrow keys (with wrapping!)
* Delete/insert text
* Dont worry if you mess it up, you cant save! (also definitly a feature)
* File menu side bar to choose with file to edit using fuzzy search

## Usage
Press tab to switch focus bettween text area and file menu. File menu uses fuzzy search to find files. Use up and down arrows to navigate options. Press enter to select.

## Parts
### Pane
For layout I am using a custom layout system and tui renderer. Each pane needs a render function that take in a canvas for it to draw too and an event function to propogate events throught the tree. For now Im using termion events to handle user input and pass it throught.
