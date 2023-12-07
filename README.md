

# RustyMold

Rust re-write of [Cute Mold](https://erytau.itch.io/cute-mold) by erytau. A simulation of molds undergoing virtual evolution. 

Compared to [the original](https://github.com/abcdeab/CuteMold) this implementation is slightly different in some ways:
- Sharing genomes through the clipboard is not implemented yet.
- The window can be resized + panning and zooming is more accurate.
- The cap on number of molds and genomes is removed.
- The range of colors is wider, and spores have a different color than their parent mold.
- Single threaded (for now).

### Rules
- Molds grow based on their genetic code. Every cell may grow in the forward, left or right direction.
- New growth consist of either a new cell or a spore. Spores inherit the genome of their parent mold with occasional mutations.
- As molds get bigger and older they require more and more energy to stay alive.
- An empty space provides energy to a mold when it is the only mold neighboring it.
- Spores become active some time after their initial creation.
- When a mold runs out of energy it dies and its active spores turn into new molds.

### Controls

| key | action |
|:---:|--------|
|  G  | Generate random new molds |
|  D  | Delete all molds |
| Q/W | Increase/decrease light level |
|  P  | Pause/Play |
| right mouse button | Drag the canvas around |
| scroll wheel | Zoom in/out |

---

### How to build

After cloning, the usual `cargo build --release` should suffice. Only tested on Linux so far. Benchmark is available using `cargo bench`.

### But why?

To learn Rust.

### Screenshots

![mold1](https://github.com/viltered/RustyMold/assets/10100093/98931ed9-a310-4c3a-8d8e-cbb5a9647432)
![mold2](https://github.com/viltered/RustyMold/assets/10100093/945b1f09-b81e-4ad1-90f1-eecfe3d87b0e)
![mold3](https://github.com/viltered/RustyMold/assets/10100093/06c3aec5-1b80-4dc7-8e12-7e2cdfd2f076)
![mold4](https://github.com/viltered/RustyMold/assets/10100093/1fd43404-08ef-4513-9a21-d586168caea8)
![mold5](https://github.com/viltered/RustyMold/assets/10100093/98973dc8-64bc-4760-a853-9e0f78272dec)

