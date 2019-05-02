LUNAR for Rust
==============

This is a port of a classic ["lunar lander" game][wikipedia] to [Rust][rust].

The first time I saw a computer was when my father took me to an open-house at the IBM headquarters in Atlanta in the late 1970's, when I was around ten years old.  I wasn't impressed with the big sterile glass rooms filled with big blue computers, but there was a room in the basement where a couple of bearded guys asked me to sit down at a terminal and play a game.

It was this lunar landing game.  For each ten seconds of game time, it asks how much thrust you want to use, and then it tells you your new altitude, velocity, and remaining fuel.  I crashed, and then I had to get up to let the next kid take a turn.

It was simple, primitive even, but I was immediately fascinated with computers.  I saw that a computer would let me create little simulated universes that followed whatever rules I could imagine.

So after that I kept bugging my dad to buy me books about programming.  A couple of years later, my parents bought me a computer.  Thanks Mom and Dad!

The program here is pretty close to what I remember.  One difference is that, on crashing, that program announced "IT'S ALL OVER BUT THE SHOUTING", which confused me as a ten-year-old.  I thought the shouting was for joy.

This code is based upon these sources:

* [Jim Storer's original lunar landing simulation code in FOCAL][storer]
* [The code from _BASIC Computer Games_ (1978)][ahl]

[wikipedia]: https://en.wikipedia.org/wiki/Lunar_Lander_(video_game_genre)#Text_games
[rust]: https://www.rust-lang.org
[storer]: http://www.cs.brandeis.edu/~storer/LunarLander/LunarLander/LunarLanderListing.jpg
[ahl]: https://www.atariarchives.org/basicgames/showpage.php?page=106