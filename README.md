# Tuilemap

Tuilemap is a minimalistic WYSIWYG ASCII tilemap editor. It provides clean,
Vim-like keybindings, a bare-bones but full interface, region selection and
bulk editing, tileset hotkeys, and an arbitrary editing mode.

Tuilemap is not, however, good for editing large maps; it has no scrolling
mechanism, so if it can't fit on-screen (with a little wiggle-room to boot),
your editing experience will suck.

Keybindings are not customizable, but if that's something people would like
to see, it will be feature-gated to leave the basic binary as tiny as possible
(currently, it's at 4.6MB).

While tuilemap is designed for designing tilemaps, it is usable as an ASCII
art editor; be warned, the experience may not be great, and there are no
colors.

## Keybindings

 - `1-0`: Place a tile from the tileset
 - `t`: Change the tileset
 - `a / i`: Enter edit mode to enter arbitrary characters
 - `v / V`: Enter visual mode to select a region of text
 - `Escape`: Return to normal mode
 - `Space`: Erase the current tile
 - `Backspace`: Erase the current tile and go left one tile
 - `f / F`: Go to the top / bottom of a file
 - `Home / End OR g / G`: Go to the start / end of a row
 - `s`: Save the file (type `cancel` to cancel)
 - `l`: Load a file (type `cancel` to cancel)
 - `n`: Create a new file with given dimensions
 - `h`: Show a keybindings help screen

