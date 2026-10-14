## Power reserve schedule

The `power-reserve-schedule` change remains under review. Regression tests cover its boundary
behavior. Before integration, compare its policy with the current development
branch and resolve the documented differences deliberately.

## Planning horizon

A coast plan accepts horizons from one second through eighteen hours (64,800 s).
Longer windows require a fresh reserve estimate before discretionary draw is
authorized. The charge fraction and reserve floor must describe the same pack.
