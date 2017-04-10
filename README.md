# Renderer

Usage: See example in main.rs. Not yet tested on the stm32f7.

Currently only lines can be plotted. Features coordinate abstraction. Define lines in your own, abstract coortinate system, then define the Area on the screen (a 2D AABB). The renderer will scale, move and plot your lines.
