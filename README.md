# Particle Simulator

This is a 2D graphics simulator of particles and their collisions. Rust crates `flo_draw` and `flo_canvas` are used for the rendering and graphics. Particle collisions are inelastic, and I simply used the [Wikipedia Formula](https://en.wikipedia.org/wiki/Elastic_collision) for the post-collision new particle velocities.

## Further Details

---

The sweep and prune algorithm was used for broad phase collision detection, with the x-axis being the specific axis where sweep and prune is applied. The `utils.rs` file
contains a function `read_args()` for allowing command line setting of the number of particles in the simulation. For example, to initiate the simulation with 50 particles, run:

```
cargo run -- 50
```

or run the executable directly with the `-- 50 ` argument if using the compiled binary.

## 15-Second Gif of Simulation

Below is a gif made from a 15-second screen recording of the code running with 50 particles.

![particle collisions gif](https://github.com/dStern98/particle-simulator/blob/main/Particle_Simulator_Rust_Video_AdobeExpress.gif)
