#![allow(non_snake_case,non_camel_case_types,dead_code)]

use std::collections::HashMap;

/*
    Fill in the boggle function below. Use as many helpers as you want.
    Test your code by running 'cargo test' from the tester_rs_simple directory.
    
    To demonstrate how the HashMap can be used to store word/coord associations,
    the function stub below contains two sample words added from the 2x2 board.
*/


struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    wordEnd: bool,
}

impl TrieNode 
{
    fn new() -> Self 
    {
        TrieNode 
        {
            children: HashMap::new(),
            wordEnd: false,
        }
    }
}

struct Trie 
{
    root: Box<TrieNode>,
}

impl Trie 
{
    fn new() -> Self 
    {
        Trie 
        {
            root: Box::new(TrieNode::new()),
        }
    }

    fn add(&mut self, word: &str) 
    {
        let mut node = &mut self.root;
        
        for char in word.chars() 
        {
            node = node.children.entry(char).or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.wordEnd = true;
    }
}

fn safe(x: i32, y: i32, board: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>) -> bool 
{
    
    let boardLength = board.len() as i32;
    let boardWidth = if boardLength > 0 {board[0].len() as i32} else {0};

    x >= 0 && x < boardLength &&
    y >= 0 && y < boardWidth &&
    !visited[x as usize][y as usize]
}

fn dfsSearch(trie: &TrieNode, board: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, word: &mut String, wordPath: &mut Vec<(u8, u8)>, found: &mut HashMap<String, Vec<(u8, u8)>>) 
{
    if !safe(x as i32, y as i32, board, visited) 
    {
        return;
    }

    let ch = board[x][y];
    if let Some(next_trie) = trie.children.get(&ch) 
    {
        visited[x][y] = true;
        word.push(ch);
        wordPath.push((x as u8, y as u8));

        if next_trie.wordEnd 
        {
            found.entry(word.clone()).or_insert_with(|| wordPath.clone());
        }

        let adjacentCellDirections = [(-1, -1), (-1, 0), 
                                      (-1, 1), (0, -1), 
                                      (0, 1), (1, -1), 
                                      (1, 0), (1, 1)];

        for (dx, dy) in adjacentCellDirections.iter() 
        {
            let newX = x as i32 + dx;
            let newY = y as i32 + dy;
            
            if newX >= 0 && newY >= 0 && newX < board.len() as i32 && newY < board[0].len() as i32 
            {
                dfsSearch(next_trie, board, newX as usize, newY as usize, visited, word, wordPath, found);
            }
        }

        visited[x][y] = false;
        word.pop();
        wordPath.pop();
    }
}

fn boggle(board: &[&str], words: &[String]) -> HashMap<String, Vec<(u8, u8)>> 
{
    let mut found = HashMap::new();
    let mut trie = Trie::new();
    
    for word in words 
    {
        trie.add(word);
    }

    let charBoard: Vec<Vec<char>> = board.iter().map(|&s| s.chars().collect()).collect();
    let mut visitedCells = vec![vec![false; charBoard[0].len()]; charBoard.len()];

    for x in 0..charBoard.len() 
    {
        for y in 0..charBoard[x].len() 
        {
            let mut word = String::new();
            let mut wordPath = Vec::new();
            dfsSearch(&trie.root, &charBoard, x, y, &mut visitedCells, &mut word, &mut wordPath, &mut found);
        }
    }

    found
}
    
#[cfg(test)]
#[path = "tests.rs"]
mod tests;