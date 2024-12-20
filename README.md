# wordle-replacer
Replace your Wordle characters
## Usage
1. Place your Wordle result in `input.txt`. One is already provided as an example.

2. Edit `replace.json` as follows:
- `"correct"` for 🟩 
> Correct letter, correct position

- `"in_word"` for 🟨
> Letter is in the word, but not this position

- `"wrong"` for ⬛ 
> This letter is not in the word at all

3. Run the program with `cargo run`. The terminal will output the revised Wordle results.