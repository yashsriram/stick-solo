# stick-solo
## description
- Simple planning methods for 2D stick-figure free-solo climbing agents.
- Climbing route is generated by probabilistic roadmap and A\* search.
- Stick figure movements are computed using (analytical/iterative) inverse kinematics.
## math
- Details on core ideas can be found in `report.pdf`
## code
- Code is written in java, should work with JRE 8+.
    - `src/` contains all source code.
    - `jars/` contain all libraries bundled as jars.
        - processing is used as a rendering library
    - `data/` contains resources such as images, obj, mtl files.
## documentation
- For most of the code, the documentation is itself.
## usage
- Open a terminal at project root (the directory containing this file).
- Use `javac -Xlint:unchecked -cp "jars/processing/*:jars/ejml-v0.39-libs/*:jars/minim/*" -d build/ $(find -name "*.java")` to compile and put all output class files under `build/`.
- Use `java -cp "build/:jars/processing/*:jars/ejml-v0.39-libs/*:jars/minim/*" <package>.<to>.<path>.<class>` to run any simulation.
    - For example `java -cp "build/:jars/processing/*:jars/ejml-v0.39-libs/*:jars/minim/*" demos.serialagent.RRAnalyticalAgentOnPRM`.
- Tested on Ubuntu 18.04
    - If you use a distrubution that uses rolling release cycle (like Arch) you might have to install some older version of JRE and mesa (opensource intel openGL driver) that work with processing library.
## demonstration
### videos
The journey from a two-link agent to a human-like agent.

[![](http://img.youtube.com/vi/bZg6pS2gGPw/0.jpg)](https://www.youtube.com/watch?v=bZg6pS2gGPw)

Individual features.

- A 2-link analytical agent.

[![](http://img.youtube.com/vi/OWxjBbHqCp0/0.jpg)](https://www.youtube.com/watch?v=OWxjBbHqCp0)

- A 4-link jacobian transpose (iterative) agent.

[![](http://img.youtube.com/vi/4sJRt_mUX0I/0.jpg)](https://www.youtube.com/watch?v=4sJRt_mUX0I)

- A 4-link pseudo inverse (iterative) agent.

[![](http://img.youtube.com/vi/ikn62R2-2CE/0.jpg)](https://www.youtube.com/watch?v=ikn62R2-2CE)

- Simulating worms using large-count-small-length link agents.

[![](http://img.youtube.com/vi/bz4wo3fvw58/0.jpg)](https://www.youtube.com/watch?v=bz4wo3fvw58)

- A 2-arm agent.

[![](http://img.youtube.com/vi/RF-y1tmfS_8/0.jpg)](https://www.youtube.com/watch?v=RF-y1tmfS_8)

- A 2-arm 2-leg agent moving up/down/left/right/diagnol directions.

[![](http://img.youtube.com/vi/MM86jNcRnC0/0.jpg)](https://www.youtube.com/watch?v=MM86jNcRnC0)

- Race b/w 2-arm 2-leg agents.

[![](http://img.youtube.com/vi/4JfZWP0Xbcc/0.jpg)](https://www.youtube.com/watch?v=4JfZWP0Xbcc)

- A 2-arm 2-leg agent in the wild (replanning on rock slip, energy depletion, wind, waterfall).

[![](http://img.youtube.com/vi/NdP2i9E-D2E/0.jpg)](https://www.youtube.com/watch?v=NdP2i9E-D2E)
### images
- 4-link agent using jacobian-transpose.

![](github/jacobian-transpose.jpg)

- 4-link agent using pseudo-inverse method.

![](github/pseudo-inverse.jpg)

- Worm agent's tail moving to the red node.

![](github/worm.jpg)

- 2-arm agent avoiding obstacles.

![](github/2-arm.jpg)

- Simulated waterfall using a particle system.

![](github/waterfall.png)

- Wind effects, leaves and falling stones.

![](github/wind.jpg)
