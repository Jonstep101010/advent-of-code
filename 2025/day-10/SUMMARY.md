# Part 1 Algorithm Summary

## button_combination_patterns

### Purpose
Precomputes all possible button press combinations and their resulting indicator patterns to enable O(1) lookup of minimal presses needed to reach any target pattern.

### Algorithm
**Brute-force enumeration with memoization:**
1. Iterate through all 2^n button subsets using bitmask representation (0 to 2^n - 1)
2. For each subset, compute the XOR effect on indicators:
   - Filter button indices where the corresponding mask bit is set
   - Fold over selected buttons, XOR-ing their coefficient vectors into an accumulator
3. Record the minimal press count (number of set bits in mask) for each unique effect pattern

### Key Implementation Details
- **Bitmask iteration**: Each mask represents a unique subset of buttons
  - Bit i set → button i is pressed
  - `count_ones()` gives the total number of presses
- **XOR fold**: Starting from all-false indicators, toggle each indicator affected by pressed buttons
  - `acc.iter_mut().zip(&coeffs[idx])` pairs accumulator with coefficient vector
  - `*e ^= bit` toggles indicator e if button affects it
- **Minimum tracking**: `and_modify().or_insert()` ensures we keep the fewest presses per pattern
  - Multiple button combinations may yield the same effect; we only care about the cheapest

### Complexity
- Time: O(2^n × m × k) where n=buttons, m=indicators, k=avg buttons per effect
- Space: O(2^m) in worst case (all indicator patterns reachable)

### Returns
`HashMap<Vec<bool>, usize>` mapping each reachable indicator pattern to its minimal press count.

---

## validate_button_sequences

### Purpose
Solves the indicator puzzle for a single machine by finding the minimum number of button presses needed to reach the goal indicator pattern.

### Algorithm
**Two-phase approach:**
1. **Transform**: Convert button sequences to a binary coefficient matrix
   - Each button becomes a row of booleans: `true` if button affects indicator i, `false` otherwise
   - Result: `coeffs[button_idx][indicator_idx]` = button affects indicator
2. **Lookup**: Query precomputed patterns for the goal state
   - Calls `button_combination_patterns()` to enumerate all reachable patterns
   - Direct hashmap lookup of `goal_indicator_seq`
   - Returns minimal press count or `usize::MAX` if unreachable

### Key Implementation Details
- **Coefficient matrix construction**:
  ```rust
  (0..num_indicators).map(|i| button.contains(&(i as u8)))
  ```
  - Iterates each indicator position
  - Checks if button's list of affected indicators includes position i
  - Produces a boolean vector representing button's effect profile

- **Lookup with fallback**:
  ```rust
  patterns.get(&machine.goal_indicator_seq)
      .copied()
      .unwrap_or(usize::MAX)
  ```
  - Retrieves minimal press count for goal pattern
  - `usize::MAX` indicates unreachable (should never happen with valid input)

### Complexity
- Time: Dominated by `button_combination_patterns` - O(2^n × m)
- Space: O(n × m) for coefficient matrix + O(2^m) for patterns

### Returns
`i32` - Minimal number of button presses to achieve goal state, or `i32::MAX` if impossible.

---

## Overall Strategy

**Problem**: Given a set of buttons that toggle specific indicators, find minimum presses to reach a target on/off pattern.

**Solution approach**: 
- Model as XOR operations over GF(2) - each button press toggles its affected indicators
- Precompute all 2^n combinations since pressing the same button twice cancels out
- Use hashmap for O(1) lookup after O(2^n) preprocessing
- Optimal for small n (≤20 buttons), becomes infeasible for larger inputs

**Why this works**:
- Button presses are commutative and idempotent mod 2 (order doesn't matter, A+A=0)
- This reduces infinite search space to finite 2^n combinations
- Minimum press count = minimum cardinality subset achieving target XOR pattern
