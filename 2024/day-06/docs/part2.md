- if we reach the start from the right again, we can close the loop by assigning to x-1 (if in grid)
- we can block in the exit (if there is something blocking other exits)
- we can block points where walls meet
- something where we have been before, but no turns were completed
- there is an intersection where we can turn right onto a previous path
	(including with distance: line of sight onto start_pos)
- save unique obstacle position

.|.
O^-

?|?
-P-
.0.

-|0
-+

++
+?
#0


.#+-^-+-+.
..|.!.|.#.
#O+-!-+...