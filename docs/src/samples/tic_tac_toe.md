# Tic-Tac-Toe

```canary
use Player.*
use Cell.*
use Outcome.*

enum Player {
    | X
    | O

    fn symbol() -> String := match self {
        X => "X"
        O => "O"
    }

    fn next() -> Player := match self {
        X => O
        O => X
    }
}

enum Cell {
    | Empty
    | Taken(Player)
}

enum Outcome {
    | Playing
    | Won(Player)
    | Draw
}

class Board {
    static const WIN_LINES: [[Int]] := [
        [0,1,2],[3,4,5],[6,7,8],
        [0,3,6],[1,4,7],[2,5,8],
        [0,4,8],[2,4,6],
    ]

    mut cells: [Cell] := [
        Empty, Empty, Empty,
        Empty, Empty, Empty,
        Empty, Empty, Empty,
    ]

    fn place(row: Int, col: Int, player: Player) -> Bool {
        idx := row * 3 + col
        if cells[idx] != Empty { return false }
        cells[idx] = Taken(player)
        true
    }

    fn winsFor(player: Player) -> Bool {
        p := Taken(player)
        for line in WIN_LINES {
            if cells[line[0]] == p & cells[line[1]] == p & cells[line[2]] == p {
                return true
            }
        }
        false
    }

    fn full() -> Bool {
        for c in cells {
            if c == Empty { return false }
        }
        true
    }

    fn outcome() -> Outcome {
        if winsFor(X) { return Won(X) }
        if winsFor(O) { return Won(O) }
        if full() { return Draw }
        Playing
    }

    fn display() {
        for r in 0..3 {
            for c in 0..3 {
                sym := match cells[r * 3 + c] {
                    Empty => "."
                    Taken(p) => p.symbol()
                }
                print(sym)
                if c < 2 { print("|") }
            }
            println()
            if r < 2 { println("-+-+-") }
        }
    }
}

fn askMove(player: Player) -> (Int, Int) {
    loop {
        print("Player " + player.symbol() + " - row and col (0-2): ")
        parts := readLine().split(" ")
        if parts.len() != 2 { continue }
        row := parts[0].parse::[Int]() catch { continue }
        col := parts[1].parse::[Int]() catch { continue }
        if row < 0 | row > 2 | col < 0 | col > 2 { continue }
        return (row, col)
    }
}

fn main() {
    board := Board()
    mut current := X

    loop {
        board.display()
        (row, col) := askMove(current)

        if !board.place(row, col, current) {
            println("Cell taken - try again.")
            continue
        }

        match board.outcome() {
            Playing => {}
            Won(p) => {
                board.display()
                println("Player " + p.symbol() + " wins!")
                return
            }
            Draw => {
                board.display()
                println("It's a draw!")
                return
            }
        }

        current = current.next()
    }
}
```
