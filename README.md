## gt_rs
This tool essentially combines harpoon and cd. If you frequently are cd'ing the same directories, this is for you!

## Installation
1. Install `gt_rs`:
```
cargo install gt_rs
```
2. Add the following to your `.zshrc` / `bashrc` file:
```
function gt() {
  cd "$(gt_rs "$@")"
}
```

## Usage
- `gt`: opens path selector. Simply navigate to the path you want to go to, and press enter/space.
- `gt add <PATH>` adds a path to your list that opens with `gt`.

## Next features:
- Ability to delete entries
- Navigate by number
- Double press to confirm selection with number
- tests, lol
