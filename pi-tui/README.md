# pi-tui — Colorful Leibniz π Digit Circle

Terminal UI that runs the classic Leibniz series for π and draws the stream of
**last digits** (of the 10-decimal approximation) as text arranged on a
**circular path** in the terminal.

## Same math as the original snippet

```rust
sum += sign / n as f64;
let pi_approx = 4.0 * sum;
let last_digit = format!("{:.10}", pi_approx).chars().last().unwrap();
```

## Features

- Digits placed on a perfect circle (polar → terminal cells)
- Continuous rotation of the ring
- Rainbow colour cycle + gold highlight on the newest digit
- Live π approximation and term counter in the centre
- Adjustable speed

## Controls

| Key        | Action              |
|------------|---------------------|
| `Space`    | Pause / Resume      |
| `r`        | Reset series        |
| `s`        | Toggle spin         |
| `+` / `-`  | Speed up / down     |
| `q` / `Esc`| Quit                |

## Build & run

```bash
cd pi-tui
cargo run --release
# or
./pi-tui   # pre-built binary (Linux x86_64)
```

Requires a terminal that supports 24-bit colour and at least ~80×24 cells
for a nice circle.
