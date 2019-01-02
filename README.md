# amethyst_transparency_test

Shows odd behavior with transparency layers.

W - Swap top and middle layers
S - Swap middle and bottom layers

Behavior in different configurations (listed from bottom-most layer to top-most)

Checkerboard -> Smiley -> Hex: No odd behavior
Checkerboard -> Hex -> Smiley: Smiley allows checkerboard to show through, but converts overlapped parts of the hex to clear color
Smiley -> Checkerboard -> Hex: Checkerboard turns smiley to clear color, hex has no adverse affects on checkerboard
Smiley -> Hex -> Checkerboard: Checkerboard turns smiley and hex to clear color
Hex -> Smiley -> Checkerboard: Same as above
Hex -> Checkerboard -> Smiley: Hex turned to clear color, checkerboard unaffected by smiley
