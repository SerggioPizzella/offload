import sys
import numpy as np

def solve(board):
    # Find the next empty cell
    for i in range(9):
        for j in range(9):
            if board[i][j] == 0:
                # Try all possible values for the empty cell
                for val in range(1, 10):
                    if is_valid(board, i, j, val):
                        # If the value is valid, assign it to the cell and recursively try to solve the rest of the board
                        board[i][j] = val
                        if solve(board):
                            return True
                # If no value works, backtrack
                board[i][j] = 0
                return False
    # If the board is full, it's solved
    return True

def is_valid(board, row, col, val):
    # Check the row
    if val in board[row]:
        return False
    # Check the column
    if val in [board[i][col] for i in range(9)]:
        return False
    # Check the 3x3 grid
    start_row = row // 3 * 3
    start_col = col // 3 * 3
    for i in range(3):
        for j in range(3):
            if board[start_row + i][start_col + j] == val:
                return False
    # The value is valid
    return True

def is_valid2(board, row, col, val):
    # Check the row, skipping the cell that is being checked
    for i in range(9):
        if i != col and board[row][i] == val:
            return False

    # Check the column, skipping the cell that is being checked
    for i in range(9):
        if i != row and board[i][col] == val:
            return False

    # Check the 3x3 grid, skipping the cell that is being checked
    start_row = row // 3 * 3
    start_col = col // 3 * 3
    for i in range(3):
        for j in range(3):
            if (start_row + i != row or start_col + j != col) and board[start_row + i][start_col + j] == val:
                return False

    # The value is valid
    return True

def is_board_valid(board):
    for i in range(9):
        for j in range(9):
            if board[i][j] != 0 and not is_valid2(board, i, j, board[i][j]):
                return False
    return True

def main():
    # Example usage
    board = [[0, 0, 0, 0, 0, 9, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 5, 0, 9],
            [0, 0, 0, 0, 0, 0, 2, 6, 3],
            [7, 0, 0, 4, 1, 0, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 0, 2],
            [6, 5, 1, 0, 0, 0, 0, 7, 0],
            [0, 4, 0, 2, 0, 5, 0, 0, 0],
            [5, 0, 0, 7, 3, 0, 0, 0, 8],
            [0, 0, 0, 8, 0, 0, 9, 0, 1]]

    # Modify the board with the values supplied through the command line
    for i in range(9):
        for j in range(9):
            if board[i][j] == 0:
                try:
                    board[i][j] = int(sys.argv[1])
                    sys.argv.pop(1)
                except:
                    continue
                
    # Check if the board is valid before trying to solve it
    if not is_board_valid(board):
        print("The board is invalid.")
        return
        
    if solve(board):
        print(np.array(board))
    else:
        print("No solution found")

if __name__ == '__main__':
  main()