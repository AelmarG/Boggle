defmodule Boggle do

  @moduledoc """
  Add your boggle function below. You may add additional helper functions if you desire. 
  Test your code by running 'mix test' from the tester_ex_simple directory.
  """
  
  def boggle(board, words) do
      #create the trie with all the words provided
      trie = populateTrie(words)

      #populate wordsInBoard list by calling the searchBoard function
      wordsInBoard = for y <- 0..tuple_size(board)-1, reduce: [] do
          wordsInBoard -> [wordsInBoard, for x <- 0..tuple_size(board)-1, reduce: [] do
              wordsInBoard -> [wordsInBoard, searchBoard(board, trie, [{y, x}])]
          end]
      end

      #take the wordsInBoard list and turn it into a map
      for {word, path} <- List.flatten(wordsInBoard), reduce: %{} do
          wordsInBoard -> Map.put(wordsInBoard, word, path)
      end
  end

  #does DFS through board to find all existing words within it
  def searchBoard(board, trie, [{x, y} | restOfPath] = path) when 0 <= x and x < tuple_size(board) and 0 <= y and y < tuple_size(board) do
    if {x, y} not in restOfPath do
      char = elem(elem(board, x), y)
      
      #checks what the current value of char is
      case trie[char] do
        nil -> []
        subTrie -> 
          wordsInBoard = if subTrie[:word], do: [{subTrie[:word], Enum.reverse(path)}], else: []
          adjacentCells = [
            {1, 0},{-1, 0},
            {0, 1},{0, -1},
            {-1, -1},{-1, 1},
            {1, -1},{1, 1}]
          Enum.reduce(adjacentCells, wordsInBoard, fn {dy, dx}, acc -> acc ++ searchBoard(board, subTrie, [{x + dx, y + dy} | path]) end)
      end
    else
      []
    end
  end
  def searchBoard(_, _, _), do: []
 
  def populateTrie(words) do
   populateTrieHelp(words, %{})
  end
  
  defp populateTrieHelp([], trie) do trie end
  defp populateTrieHelp([word | rest], trie) do
      populateTrieHelp(rest, addWordToTrie(
          String.codepoints(word),
          trie,
          word
      ))
  end

  defp addWordToTrie([], trie, word) do
      Map.put(trie, :word, word)
  end
  defp addWordToTrie([char | rest], trie, word) do
      subTrie = if Map.has_key?(trie, char) do
          Map.get(trie, char)
      else
          %{}
      end
      Map.put(trie, char, addWordToTrie(rest, subTrie, word))
  end

end
