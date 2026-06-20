# Tic-Tac-Toe

```canary
enum Player { X, O }

enum Cell {
    Empty,
    Taken(Player),
}

enum Outcome {
    Playing,
    Won(Player),
    Draw,
}

class Board {
    mut cells: [Cell] := [
        Cell.Empty, Cell.Empty, Cell.Empty,
        Cell.Empty, Cell.Empty, Cell.Empty,
        Cell.Empty, Cell.Empty, Cell.Empty,
    ]

    fn place(row: Int, col: Int, player: Player) -> Bool {
        idx := row * 3 + col
        if cells[idx] != Cell.Empty { return false }
        cells[idx] = Cell.Taken(player)
        true
    }

    fn winsFor(player: Player) -> Bool {
        p := Cell.Taken(player)
        for line in [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]] {
            if cells[line[0]] == p & cells[line[1]] == p & cells[line[2]] == p {
                return true
            }
        }
        false
    }

    fn full() -> Bool {
        for c in cells {
            if c == Cell.Empty { return false }
        }
        true
    }

    fn outcome() -> Outcome {
        if winsFor(Player.X) { return Outcome.Won(Player.X) }
        if winsFor(Player.O) { return Outcome.Won(Player.O) }
        if full()            { return Outcome.Draw }
        Outcome.Playing
    }

    fn display() {
        for r in 0..3 {
            for c in 0..3 {
                sym := match cells[r * 3 + c] {
                    Cell.Empty    => "."
                    Cell.Taken(p) => playerSymbol(p)
                }
                print(sym)
                if c < 2 { print("|") }
            }
            println("")
            if r < 2 { println("-+-+-") }
        }
    }
}

fn playerSymbol(p: Player) -> String := match p {
    Player.X => "X"
    Player.O => "O"
}

fn nextPlayer(p: Player) -> Player := match p {
    Player.X => Player.O
    Player.O => Player.X
}

fn askMove(player: Player) -> [Int] {
    loop {
        print("Player " + playerSymbol(player) + " — row and col (0–2): ")
        line := readLine()
        parts := line.split(" ")
        if parts.len() != 2 { continue }
        row := parts[0].parse::[Int]() catch { continue }
        col := parts[1].parse::[Int]() catch { continue }
        if row < 0 | row > 2 | col < 0 | col > 2 { continue }
        return [row, col]
    }
}

fn main() {
    board := Board()
    mut current := Player.X

    loop {
        board.display()
        move := askMove(current)

        if !board.place(move[0], move[1], current) {
            println("Cell taken — try again.")
            continue
        }

        match board.outcome() {
            Outcome.Playing    => {}
            Outcome.Won(p)     => {
                board.display()
                println("Player " + playerSymbol(p) + " wins!")
                return
            }
            Outcome.Draw       => {
                board.display()
                println("It's a draw!")
                return
            }
        }

        current = nextPlayer(current)
    }
}
```
