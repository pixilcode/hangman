# Hangman Challenge
A recreation of the game 'Hangman' in Rust.

**Time spent:** `4:00:00`

(This time started when I read the problem and ended with the commit
[Final product](https://github.com/pixilcode/hangman/commit/43f1922dd56610ffe175ca2e00655fa67b6cbd1c))

Though the code itself was left unchanged after the final commit,
other files were added to prepare for the submission, including:
  * [`assets/notes.jpg](assets/notes.jpg)`
  * [`README.md`](README.md)
  * [Linux and Windows binaries](https://github.com/pixilcode/php-course/releases/tag/submission)

In the case that the above links don't work, the binaries can be
found in the `submission` release.

## The `-f` option (possible failure)
In accordance with requirement 6, the program continues to ask the
user for guesses until the user has correctly guessed the word.
This is the default behavior.

However, in order to more closely simulate a real game of hangman,
a `-f` option was added. The `-f` option makes it so that a user
only has six incorrect guesses before losing the game.

This option was added in anticipation of the implementation of
requirement 10, in which the gallows, as well as the man being
hanged, is drawn. This requirement conflicts with requirement 6
because requirement 6 specifies that the game continues until
the player succeeds.

Requirement 10, on the other hand, implies
that there is a failure, as there is not an infinite number of
body parts that can be drawn. The original game also defines
failure as the moment when all the body parts are drawn. Thus,
the `-f` option allows the player to choose whether they want
to play according to Requirement 6 or Requirement 10.

The choice of six incorrect guesses is justified by the number
of body parts drawn, as shown in the following diagram:

```
   O        1
  /|\     3 2 4
  / \     5   6
```

In future versions, the `-f` option will also print out an
image representing the man in the gallows.

## Future Work
- [ ] Create function for getting user input
- [ ] Create interface for standardizing output
- [ ] Add visual dividers to make output easier to read
      *or* clear the screen at the beginning of the game loop
- [ ] For the `-f` option, print out the gallows so that the
      can get a visual of their progression.