+++
[extra]
wasm_example = "gd_reach"
+++

# absolute angle formulation

The robotic arm dynamics can be formulated as the end postion in terms of lenghts and orientation of each arm.

$$
P
= \Sigma l_i e^{i \theta_i}
= \Sigma [ l_i cos(\theta_i) + i l_i sin(\theta_i) ]
= \Sigma l_i cos(\theta_i) + i \Sigma l_i sin(\theta_i)
$$

Given goal $ G $. We can formulate a cost function $ C $.

$$
C
= \frac{1}{2} (P - G) \overline{(P - G)}
$$

$$
C
= \frac{1}{2} ( (\Sigma l_i cos(\theta_i) - G_x) + i (\Sigma l_i sin(\theta_i) - G_y) ) ( \overline{(\Sigma l_i cos(\theta_i) - G_x) + i (\Sigma l_i sin(\theta_i) - G_y)} )
$$

$$
C
= \frac{1}{2} (\Sigma l_i cos(\theta_i) - G_x)^2 + (\Sigma l_i sin(\theta_i) - G_y)^2
$$

$$
\frac{\partial C}{\partial \theta_i}
= \frac{1}{2} 2 . (\Sigma l_i cos(\theta_i) - G_x) . - l_i sin(\theta_i) + 2 . (\Sigma l_i sin(\theta_i) - G_y) . l_i cos(\theta_i) 
$$

$$
= (\Sigma l_i cos(\theta_i) - G_x) . - l_i sin(\theta_i) + (\Sigma l_i sin(\theta_i) - G_y) . l_i cos(\theta_i)
$$

$$
=  l_i cos(\theta_i)(\Sigma l_i sin(\theta_i) - G_y)- l_i sin(\theta_i)(\Sigma l_i cos(\theta_i)  - G_x)
$$

A small step in direction of negative gradient is

$$
-\alpha \frac{\partial C}{\partial \theta_i}
= - \alpha l_i cos(\theta_i)(\Sigma l_i sin(\theta_i) - G_y)- l_i sin(\theta_i)(\Sigma l_i cos(\theta_i)  - G_x)
$$

