# Particle Simulator

This is a 2D graphics simulator of particles and their collisions. Rust crates `flo_draw` and `flo_canvas` are used for the rendering and graphics. Particle collisions are inelastic, and I simply used the [Wikipedia Formula](https://en.wikipedia.org/wiki/Elastic_collision) for the post-collision new particle velocities.

## Further Details

---

The sweep and prune algorithm was used for broad phase collision detection, with the x-axis being the specific axis where sweep and prune is applied. The `utils.rs` file
contains a function `read_args()` for allowing command line setting of the number of particles in the simulation. For example, to initiate the simulation with 20 particles, run:

```
cargo run -- 20
```

or run the executable directly with the `-- 20 ` argument if using the compiled binary.

## Todo

---

The current program is single-threaded. I intend to implement multithreading at some point in the future to make the collision detection more efficient, and allow for more particles or a higher frame rate in the simulation.
