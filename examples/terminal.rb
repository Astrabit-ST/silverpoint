# Copyright (c) 2023 Lily Lyons
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

require "silverpoint"

def input(prompt)
    print prompt

    gets.chomp
end

def get_cpu_move(board, best)
    move, count, _ = if best
        board.best_next_move(4)
    else
        board.worst_next_move(4)
    end

    print "CPU evaluated #{count} moves before choosing to "

    if move.piece?
        (from, to) = move.piece_positions
        
        piece, takes = board.piece(from), board.piece(to)
        if piece && takes
            puts "take #{takes.name}(#{to}) with #{piece.name}(#{from})"
        elsif piece && !takes
            puts "move #{piece.name}(#{from}) to #{to}"
        else
            puts "move #{from} to #{to}"
        end
    elsif move.kingsidecastle?
       puts "castle kingside"
    elsif move.queensidecastle?
        puts "castle queenside"
    elsif move.resign?
        puts "resign" 
    end

    move
end

board = Silverpoint::Board::new
puts board
history = []

loop do
    s = input(">>> ")

    m = if s.empty? 
        puts "Waiting for CPU to choose best move..."
        get_cpu_move(board, true)
    elsif s == "worst"
        puts "Waiting for CPU to choose worst move..."
        get_cpu_move(board, false)
    elsif s == "rate"
        next
    elsif s == "pass"
        board = board.change_turn()
        next
    elsif s ==  "history"
        for i in 0..history.length() - 1 
            if i < history.length() - 1
                puts "#{history[i]} #{history[i + 1]}"
            else
                puts "#{history[i]}"
            end
        end
        next
    else
        begin
            Silverpoint::Move::parse(s)
        rescue Exception => e
            puts e
            next
        end
    end

    result = board.play_move(m)
    if result.continuing?
        board = result.next_board
        puts board
        history << m
    elsif result.victory?
        puts board
        puts "#{result.winning_color} wins."
        break
    elsif result.illegal_move?
        puts "#{result.illegal_move} is an illegal move"
    elsif result.stalemate?
        puts "Drawn game."
        break
    end
end

history.each do |history|
    puts history
end