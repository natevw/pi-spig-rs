# Pi Spigot in Rust

This is ± the classic [Rabinowitz and Wagon (pdf)](http://stanleyrabinowitz.com/download/spigot-revised.pdf) spigot algorithm which streams out a fixed number of Pi digits as it calculates them.

## Background

A few years ago I wrote a Python version of this using <https://www.cut-the-knot.org/Curriculum/Algorithms/SpigotForPi.shtml> as a reference. For now I have for the most part simply ported that version into Rust (which I am learning — this being my first successful endeavor in the language) despite some of its idiosyncracies.

Now that this new Rust version is at least basically working for some initial dozens of digits, I hope to make some refinements to align it better with "the literature" and other optimizations including some noted in the book /Pi Unleashed/ (Arndt and Haenel, 2001, pp. 75–85) which I now have a copy of.

My ultimate intent — perhaps for another year's Pi day — is actually to split up the calculation between parallel processes, such that the carries also can be streamed from separate CPUs (whether multiple cores or even separate clustered machines) and maybe generate a little faster. But at least to get a little supercomputer (or rather, a "little supercomputer") experience.

## See also

This isn't the fastest way to calculate a kajillion digits of Pi; it just looked like a fun one to play with. Besides the links above, here's some further reading I've found on related to this algorithm:

* http://pi314.net/eng/goutte.php (another discussion of how this algorithm works)
* https://www.cs.ox.ac.uk/jeremy.gibbons/publications/spigot.pdf (**Unbounded** Spigot Algorithms for the Digits of Pi)
* https://www.gavalas.dev/blog/spigot-algorithms-for-pi-in-python/ (sample implementations of above)
* https://stackoverflow.com/questions/4084571/implementing-the-spigot-algorithm-for-%CF%80-pi (misc. discussion and other links)
* https://en.wikipedia.org/wiki/Approximations_of_%CF%80 (incl. https://en.wikipedia.org/wiki/Mil%C3%BC ;-)


## License

© 2023 Nathan Vander Wilt.

Shared under MIT license terms.
