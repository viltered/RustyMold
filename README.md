

# RustyMold

Rust re-write of [Cute Mold](https://erytau.itch.io/cute-mold) by erytau. A simulation of molds undergoing virtual evolution. 

Compared to [the original](https://github.com/abcdeab/CuteMold), this has no cap on the number of molds and genomes, but keyboard controls and copying genomes to the clipboard are not implemented yet.


### Rules
- Molds grow based on their genetic code. Every cell may grow in the forward, left or right direction.
- New growth consist of either a new cell or a spore. Spores inherit the genome of their parent mold with occasional mutations.
- As molds get bigger and older they require more and more energy to stay alive.
- An empty space provides energy to a mold when it is the only mold neighboring it.
- Spores become active some time after their initial creation.
- When a mold runs out of energy it dies and its active spores turn into new molds.

---

### How to build

After cloning, the usual `cargo build --release` should suffice. Only tested on Linux so far.
Benchmark is available using `cargo bench`.

### But why?

To learn Rust.