# Mastermind Helper

A command-line implementation of the classic **Mastermind** game, written in Rust. This version offers multiple modes of play, including algorithm-assisted analysis and simulations using **Knuth's optimal algorithm**.

## Description

In a standard game of Mastermind, the player has 6 attempts to guess a hidden combination.
The most common playstyle, while generally effective, can occasionally be suboptimal—sometimes leading to a final guess with only a 50/50 chance of success.

The main goal of this project is to help players practice and develop strategies by comparing their guesses with Knuth’s algorithm, which guarantees solving any combination in **5 moves or fewer**.
[Read about the algorithm here (PDF)](https://www.cs.uni.edu/~wallingf/teaching/cs3530/resources/knuth-mastermind.pdf)

## Features

1. **Normal Mode**  
   Play Mastermind by yourself. You have 6 attempts to guess a secret 4-symbol combination. Symbols are chosen from a fixed set (☻♣♠♥♦♪), and after each guess, you'll receive feedback on the number of:
   - Correct symbols in the correct position (•).
   - Correct symbols in the wrong position (○).

2. **Step-by-Step Analysis Mode**  
   Play as normal, but after each move you can review:
   - Whether your move was optimal according to Knuth’s algorithm.
   - All optimal guesses from the current state.
   - All optimal guesses from the current state similar to your guess.
   - Remaining possible combinations and their count.
   - Whether any of the calculated optimal moves could be the final combination.
    > **Explanation:** The algorithm filters optimal guesses to keep only those that might match the solution (only if there are any), increasing the chance of winning in fewer turns.  
    > This is highlighted because it reflects the natural human strategy of aiming for guesses that could directly be the final combination—maximizing correct symbols in their exact positions.  
    > However, the optimal algorithm sometimes makes seemingly counterintuitive moves that cannot be the final solution.  
    > These moves are mathematically chosen to most effectively reduce the pool of possible combinations, leading to a faster overall solution. In these cases the player must think outside the box to play optimally.

3. **Algorithm Simulation Mode**  
   Watch Knuth's algorithm optimally solve a game of Mastermind with a random combination. After the combination is shown on-screen, this simulation showcases the steps the algorithm uses to find it in 5 steps or fewer.

The game is currently supported only in the standard command-line interface. A plan for the future is to add a graphical user interface.
