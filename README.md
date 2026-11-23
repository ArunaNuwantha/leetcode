# LeetCode in Rust

A playground for writing your own LeetCode solutions in Rust, with quick table-driven tests.

## Workflow

1. On leetcode.com, set the language to **Rust** and copy the starter code (`impl Solution { ... }`).
2. Create the problem (it reads the starter code from your clipboard):
   ```sh
   ./lc new 1 Two Sum
   ```
   → `src/problems/p0001_two_sum.rs`, with the signature, a `todo!()` body and an empty test table.
3. Write your solution, add test cases (copy the examples from the problem page):
   ```rust
   cases! {
       Solution::two_sum;
       ex1: (vec![2, 7, 11, 15], 9) => vec![0, 1],
       ex2: (vec![3, 2, 4], 6)      => vec![1, 2],
   }
   ```
4. Run it:
   ```sh
   ./lc test        # tests the problem you edited most recently
   ./lc test 1      # or a specific one
   ./lc watch       # re-run on every save (cargo install cargo-watch)
   ```
5. Paste the `impl Solution { ... }` block back into LeetCode and submit.

## Commands

| Command | Does |
|---|---|
| `./lc new <id> <title...>` | Create a problem (`--no-paste` to ignore the clipboard) |
| `./lc test [id]` | Run its tests (default: last edited) |
| `./lc watch [id]` | Re-run tests on save |
| `./lc list` | List problems |
| `./lc rm <id>` | Delete a problem |

Tip: `alias lc=~/Documents/dev/rust/leetcode/lc` in `~/.zshrc` so you can run `lc` from anywhere.

## Test helpers (available in every problem file)

| Helper | Produces | Example |
|---|---|---|
| `list![1, 2, 3]` | `Option<Box<ListNode>>` | Linked-list inputs/outputs |
| `tree![1, null, 2, 3]` | `Option<Rc<RefCell<TreeNode>>>` | Paste LeetCode's tree array as-is |
| `s!("abc")` | `String` | |
| `vs!["a", "b"]` | `Vec<String>` | |
| `vvc![["1","0"],["0","1"]]` | `Vec<Vec<char>>` | Grid problems |
| `cases! { sorted Solution::f; ... }` | sorts both sides before comparing | "Return the answer in any order" |

`ListNode` and `TreeNode` match LeetCode's definitions exactly. `list_to_vec` and `tree_to_vec`
convert them back for debugging. For anything unusual you can write a plain `#[test] fn` in the
`tests` module. Use `println!`/`dbg!` freely, since tests run with `--nocapture`.

## Layout

```
src/util.rs          ListNode, TreeNode, helper macros
src/problems/        one file per problem (mod.rs updated automatically)
cli/                 the `lc` tool (a separate crate, so a broken solution never breaks the CLI)
```
