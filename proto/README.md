# proto

> prôtos: first

![ploom5.rs](https://github.com/magi-1/nomos/blob/main/proto/images/ploom5_3.png) ![ploom5.rs](https://github.com/magi-1/nomos/blob/main/proto/images/ploom3.png) 

# Ploom

A ```Ploom``` is defined by a collection of `circles` subject to random noise that follow a `focus`. These circles/points are subject to gaussian noise `dx`and an attractive force towards the focus. By controling the location of the foci (which can be invisible), you inherently control the circles associated with it. Furthermore, it is possible to make these foci interact with eachother. There is a lot you can do with a `Ploom`. 


```rust
struct Circle {
    x: Vec2,
    r: f32
}

struct Ploom {
    circles: Vec<Circle>,
    focus: Vec2,
    sigma : f32,
    v: Vec2
}
```

Note that there are additional features as the version of `ploom*.rs` increases such as edges drawn between foci within some maximum distance etc. 

```rust
// nannou params
const WINDOW_SIZE: u32 = 1000;

// point and line parameters
const MAX_RADIUS: f32 = 3.0;
const MAX_DISTANCE: f32 = 100.0;
const ALPHA: f32 = 0.02; // trailing effect

// number of objects
const NUM_PLOOMS: usize = 150;
const NUM_CIRCLES: usize = 2;

// randomness and physics parameters
const SCALE: f32 = 100.0; // controls explosiveness
const DECAY: f32 = 0.98; // variance decay
const THRESHOLD: f32 = 0.2; // min variance
const PLOOM: f32 = SCALE/5.0; // 2-10 is a reasonable
const DAMPENING: f32 = 0.02; // 0.001-0.1 is reasonable
```
