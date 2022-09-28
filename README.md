# stick-solo

## description
- Simple planning methods for 2D stick-figure free-solo climbing agents.
- This problem is studied under couple of my projects listed here.
    - `1.gradient_descent_ik` uses gradient descent based approach.
    - `2.ceo_rl_and_random_sample_solves` builds on previous one, adding random sample solves and cross-entropy method based rl.
- For more details on a project, just click on it.

### showcase

- The tour of `1.gradient_descent_ik`. Link to youtube video. Click to play.

[![](http://img.youtube.com/vi/bZg6pS2gGPw/0.jpg)](https://www.youtube.com/watch?v=bZg6pS2gGPw)

- The final result of `2.ceo_rl_and_random_sample_solves`.

![](./2.ceo_rl_and_random_sample_solves/github/12.gif)

```mermaid
flowchart TD
    problem --> config_space --> finish_point_calculation --> obstacles --> path_planning --> interpolate --> animation
```
