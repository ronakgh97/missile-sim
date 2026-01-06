# MATHS BEHIND HOMING MISSILE GUIDANCE LAWS

SCENARIO: Missile (M) Pursuing Target (T) in 3D Space

```shell

                                    T [target]
                                   /│\     V_t (target velocity)
                                  / │ \
                                 /  │  \
                        R(t) →  /   │   \
                               /    │    \
                              /     │LOS  \
                             /  λ̇ (rate)   \
                            /       │       \
                           /        │        \
                          /         │         \
                      M [missile]   ↓         ┴ (perpendicular)
                         \
                          \  V_m (missile velocity)
                           ↘
                            a_c (commanded acceleration)
```

## Core Variables and Vectors

- **R(t)** = Target Position - Missile Position (range vector)
- **r** = ‖R‖ (magnitude of range vector)
- **λ̂** = R / r (line-of-sight unit vector)
- **λ̇** = dλ/dt (LOS angular rate)
- **V_m** = missile velocity vector
- **V_t** = target velocity vector
- **V_c** = -(R · Ṙ) / r (closing speed, rate of range decrease)
- **Ṙ** = V_t - V_m (relative velocity)

---

## Guidance Law Equations

### 1. Proportional Navigation (PN)

**Classical guidance law, foundation of most modern systems.**

```
a_c = N × V_m × λ̇
```

**Parameters:**

- N = Navigation constant (typically 3-5)
- V_m = ‖V_m‖ (missile speed)
- λ̇ = LOS angular rate

**Physical Interpretation:**

- Command acceleration proportional to how fast the target appears to move in the missile's field of view
- If λ̇ = 0, the missile is on a collision course (no correction needed)
- Constant bearing, decreasing range (CBDR) principle

**Advantages:**

- Simple to implement
- Well-understood behavior
- Effective against non-maneuvering targets

**Disadvantages:**

- Less efficient fuel consumption
- Suboptimal against maneuvering targets
- Uses missile speed rather than closing speed

---

### 2. True Proportional Navigation (TPN)

**Enhanced version using closing velocity instead of missile velocity.**

```
a_c = N × V_c × λ̇
```

**Parameters:**

- N = Navigation constant
- V_c = -dr/dt = -(R · Ṙ) / r (closing speed)
- λ̇ = LOS angular rate

**Computation of Closing Speed:**

```
V_c = -(R · (V_t - V_m)) / ‖R‖
```

**Advantages over PN:**

- More fuel-efficient for non-head-on intercepts
- Better performance against maneuvering targets
- Optimal energy trajectory
- Accounts for actual rate of range closure

**Physical Interpretation:**

- Uses the rate at which range is decreasing, not just missile speed
- More accurate representation of intercept geometry
- Particularly effective in tail-chase scenarios

---

### 3. Augmented Proportional Navigation (APN)

**Predictive guidance law that compensates for target maneuvers.**

```
a_c = N × V_c × λ̇ + (N/2) × a_t⊥
```

**Parameters:**

- N = Navigation constant
- V_c = Closing speed
- λ̇ = LOS angular rate
- a_t⊥ = Target acceleration perpendicular to LOS

**Target Acceleration Projection:**

```
a_t⊥ = a_t - (a_t · λ̂)λ̂
```

**Components:**

1. **Feedback term:** N × V_c × λ̇ (same as TPN)
2. **Feedforward term:** (N/2) × a_t⊥ (target maneuver compensation)

**Advantages:**

- Anticipates target evasive maneuvers
- Reduces miss distance against agile targets
- Optimal against maneuvering targets
- Shorter intercept time

**Requirements:**

- Needs target acceleration measurement or estimation
- More complex sensor/processing requirements
- Higher computational load

---

### 4. Pure Pursuit (PP)

**Simplest guidance strategy: point directly at the target.**

```
V_m_desired = V_max × λ̂
```

**Characteristics:**

- Always points velocity vector directly at target
- Follows curved path behind target
- No lead angle

**Advantages:**

- Extremely simple implementation
- Minimal computational requirements
- Intuitive behavior

**Disadvantages:**

- Inefficient energy usage
- Longer intercept paths
- Can fail against fast-moving targets
- Like you pure chasing your crush who keeps running away. 😞

---

### 5. Lead Pursuit (LP)

**Predictive strategy: aim ahead of target based on intercept geometry.**

```
t_intercept = r / V_closing
aim_point = T_pos + V_t × t_intercept
aim_direction = (aim_point - M_pos) / ‖aim_point - M_pos‖
```

**Algorithm:**

1. Estimate time to intercept
2. Predict target future position
3. Aim at predicted position

**Advantages:**

- More direct intercept path
- Better fuel efficiency than pure pursuit
- Shorter intercept time

**Disadvantages:**

- Sensitive to velocity estimation errors
- Can diverge if predictions are inaccurate
- Requires accurate target velocity measurement

---

### 6. Deviated Pursuit (DP)

**Hybrid approach blending Pure Pursuit and Lead Pursuit.**

```
PP_direction = λ̂
LP_direction = (aim_point - M_pos) / ‖aim_point - M_pos‖
aim_direction = α × PP_direction + (1 - α) × LP_direction
```

**Parameters:**

- α ∈ [0, 1] = Blend factor
    - α = 1: Pure Pursuit
    - α = 0: Lead Pursuit
    - α = 0.5: Equal blend

**Advantages:**

- Balances simplicity with prediction
- Adaptable to different scenarios
- Robust against estimation errors
- Smoother transitions

---

## Key Mathematical Computations

### LOS Rate Calculation

The angular rate of the line-of-sight vector:

```
λ̇ = (R × Ṙ) / r²
```

**In 3D:**

```
‖λ̇‖ = ‖R × (V_t - V_m)‖ / ‖R‖²
```

**Derivation:**

- λ̂ = R / r
- dλ̂/dt = (Ṙ × r - R × ṙ) / r²
- Since R × Ṙ gives perpendicular component
- ‖λ̇‖ = ‖R × Ṙ‖ / r²

### Closing Velocity

Rate at which range is decreasing:

```
V_c = -dr/dt = -(R · Ṙ) / r
```

**Interpretation:**

- V_c > 0: Range decreasing (approaching)
- V_c < 0: Range increasing (separating)
- V_c = 0: Constant range (parallel motion)

**Expanded form:**

```
V_c = -(R · (V_t - V_m)) / ‖R‖
```

### Acceleration Command Application

**Step-by-step process:**

1. **Calculate desired acceleration** from guidance law
2. **Clamp to maximum:**
   ```
   a_actual = min(‖a_c‖, a_max) × (a_c / ‖a_c‖)
   ```
3. **Project perpendicular to velocity:**
   ```
   V̂_m = V_m / ‖V_m‖
   a_perp = a_c - (a_c · V̂_m)V̂_m
   ```
4. **Integrate velocity:**
   ```
   V_m(t + Δt) = V_m(t) + a_perp × Δt
   ```
5. **Integrate position:**
   ```
   X_m(t + Δt) = X_m(t) + V_m(t) × Δt
   ```

**Note:** Perpendicular projection ensures realistic flight dynamics where acceleration changes direction but speed
adjusts gradually.

---

## Performance Metrics

### Miss Distance

```
d_miss = min{‖R(t)‖ : t ∈ [0, T]}
```

Minimum separation achieved during engagement.

### Time to Intercept

```
t_intercept = min{t : ‖R(t)‖ < threshold}
```

Time when separation drops below hit threshold.

### Energy Expenditure

```
E = ∫₀ᵀ ‖a_c(t)‖² dt
```

Total energy used for maneuvering (proportional to fuel consumption).

### Maximum Lateral Acceleration

```
a_max_lateral = max{‖a_perp(t)‖ : t ∈ [0, T]}
```

Peak acceleration requirement (determines structural limits).

### Hit Probability

```
P_hit = 1  if d_miss < threshold
      = 0  otherwise
```

Binary outcome based on lethal radius.

---

## Simulation Integration Loop

**High-frequency discrete-time simulation (10 kHz):**

```
Δt = 0.0001 seconds

For each timestep t:
    1. Compute geometry:
       R = X_t - X_m
       r = ‖R‖
       λ̂ = R / r
       
    2. Compute kinematics:
       Ṙ = V_t - V_m
       λ̇ = (R × Ṙ) / r²
       V_c = -(R · Ṙ) / r
       
    3. Apply guidance law:
       a_c = GuidanceLaw(λ̇, V_c, N, ...)
       
    4. Enforce physical limits:
       a_actual = clamp(a_c, a_max)
       
    5. Update missile state:
       V_m ← V_m + a_actual × Δt
       X_m ← X_m + V_m × Δt
       
    6. Update target state:
       X_t ← X_t + V_t × Δt
       
    7. Record metrics:
       Store(t, X_m, X_t, V_m, V_t, a_actual, λ̇, V_c, r)
       
    8. Check termination:
       if r < threshold → HIT
       if t > t_max     → MISS
```

---

## Navigation Constant Selection

**Theoretical optimal value:**

```
N* = 3 + (dimensionality - 1)
```

- 2D intercept: N* = 3
- 3D intercept: N* = 4

**Practical considerations:**

- N = 3: Conservative, robust
- N = 4: Balanced performance
- N = 5: Aggressive, faster convergence
- N > 5: Risk of instability, high acceleration demands

**Trade-offs:**

- Higher N → Faster convergence, higher acceleration
- Lower N → Smoother trajectory, lower acceleration
- Choose based on missile maneuverability and target dynamics

---

## Implementation Notes

### SIMD Optimization

- Vector operations parallelized using AVX2
- 4x single-precision or 2x double-precision simultaneous operations
- Critical for real-time 10 kHz simulation

### Parallel Execution

- Multiple scenarios simulated concurrently using Rayon
- Thread-safe metrics collection
- Mutex-protected file I/O

### Numerical Stability

- Avoid division by near-zero range (r → 0)
- Use quaternions for attitude if needed
- Double-precision floating point for position integration

### Real-world Considerations

- Sensor noise and lag
- Actuator saturation and rate limits
- Aerodynamic effects
- Time delays in control loop
- Target maneuver estimation errors

---

## References

- Zarchan, P. (2012). *Tactical and Strategic Missile Guidance*. AIAA.
- Yanushevsky, R. (2007). *Modern Missile Guidance*. CRC Press.

---

**Simulation Parameters:**

- Time step: 0.0001 s (10 kHz)
- Max simulation time: 30 s
- Hit threshold: 10 m
- Missile max acceleration: 1000-3000 m/s²
- Navigation constant: 3-7
- Random scenario generation: 500 cases
- Guidance laws tested: 6 (PPN, TPN, APN, PP, LP, DP)

