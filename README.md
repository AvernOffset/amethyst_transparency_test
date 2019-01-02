# amethyst_transparency_test

Shows odd behavior with transparency layers. Probably a configuration error.

It appears that some sprites get priority, and when placed at a higher z-axis, the transparent parts of a higher-priority sprite will write over lower-priority sprites with the clear color. (So in this example, the checkerboard has the highest priority and won't show anything through its transparent area, smiley is in the middle, allowing the checkerboard through but not the hex, and the hex has the least priority, and allows the other two sprites to display).

W - Swap top and middle layers
S - Swap middle and bottom layers

Behavior in different configurations (listed from bottom-most layer to top-most)

**Checkerboard -> Smiley -> Hex**: No odd behavior

**Checkerboard -> Hex -> Smiley**: Checkerboard shows through smiley, but turns overlapped parts of the hex to clear color

**Smiley -> Checkerboard -> Hex**: Checkerboard turns smiley to clear color, hex has no adverse affects on checkerboard

**Smiley -> Hex -> Checkerboard**: Checkerboard turns smiley and hex to clear color

**Hex -> Smiley -> Checkerboard**: Same as above

**Hex -> Checkerboard -> Smiley**: Hex turned to clear color, checkerboard unaffected by smiley

