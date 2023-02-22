# Guessing Game
Introduction
This program is a guessing game. The user is prompted to choose a difficulty level (easy, normal, or hard) and then asked to guess a secret number. The game will keep track of the user's name, guesses, and score.

## Difficulty Levels
The user is prompted to choose the difficulty level.

Easy: The secret number is between 1 and 75.
Normal: The secret number is between 1 and 150.
Hard: The secret number is between 1 and 1000.
If the user enters an invalid input, the game will default to the normal difficulty (1-150).

## Gameplay
The user is prompted to enter their name, or type q to quit.

The user is then prompted to enter their guess. If they enter quit, the game will end and the user will not receive a score.

If the user's input is valid, the game will check to see if the guess is too small, too big, or correct. If the guess is too small or too big, the user will lose 5 points. If the guess is correct, the user will receive a score based on the number of guesses taken.

## Scoreboard
The game will also keep track of the user's name, games played, guesses, and score. At the end of the game, the scoreboard will be displayed with each user's score.
