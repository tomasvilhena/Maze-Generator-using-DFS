use rand::{Rng, seq::SliceRandom};
use colored::Colorize;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::{stdout, Write};

const EMPTY_NODE: usize = 0;
const WALL_NODE: usize = 1;
const OPEN_WALL_NODE: usize = 2;
const STARTING_NODE: usize = 3;
const ENDING_NODE: usize = 4;
const ALREADY_VISITED: usize = 5;
const PLAYER_CHARACTER_INDICATOR: usize = 9;

#[derive(Debug)]
enum MoveResult 
{
  Invalid,
  Move,
  Finished,
  Kill,
  Restart,
}

fn initial_maze_grid_maker(width: usize, height: usize) -> Vec<Vec<usize>>
{
  let width: usize = width + width + 1;
  let height: usize = height + height + 1;

  let mut maze: Vec<Vec<usize>> = vec![vec![0; width]; height];

  for y in 0..height
  {
    for x in 0..width
    {
      if x == EMPTY_NODE
      || x % 2 == 0
      || y == EMPTY_NODE
      || y % 2 == 0
      {
        maze[y][x] = WALL_NODE;
      }
    }
  }

  maze
}

fn generate_starting_and_ending_node_position_and_place_player(maze: Vec<Vec<usize>>, original_height: usize) -> Vec<Vec<usize>>
{
  let mut maze = maze;

  let random_starting_node_number = rand::thread_rng().gen_range(1..=(original_height));
  let mut random_ending_node_number = rand::thread_rng().gen_range(1..=(original_height));

  while random_ending_node_number == random_starting_node_number
  {
    random_ending_node_number = rand::thread_rng().gen_range(1..=(original_height));
  }

  let mut count_last_row = 1;
  let mut count_first_row = 1;

  let height: usize = maze.len();
  let width: usize = maze[0].len();

  for y in 0..height
  {
    for x in 0..width
    {

      if x == 0 && (y != 0 && y != maze.len())
      {
        if y % 2 != 0
        {
          if maze[y][x] == WALL_NODE
          {
            if count_first_row == random_starting_node_number
            {
              maze[y][x] = STARTING_NODE;
              maze[y][x + 1] = PLAYER_CHARACTER_INDICATOR;
            }
          }

          count_first_row += 1;
        }
      } else if x == maze[0].len() - 1  && (y != 0 && y != maze.len())
      {
        if y % 2 != 0
        {
          if count_last_row == random_ending_node_number
          {
            maze[y][x] = ENDING_NODE;
          }

          count_last_row += 1;
        }
      }
    }
  }

  maze
}

fn random_cell_picker(maze: &Vec<Vec<usize>>, original_width: usize) -> (usize, usize)
{
  let random_node_number = rand::thread_rng().gen_range(1..=(original_width*original_width));
  let mut count = 1;

  let height: usize = maze.len();
  let width: usize = maze[0].len();

  for y in 0..height
  {
    for x in 0..width
    {
      if maze[y][x] == EMPTY_NODE
      {
        if count == random_node_number
        {
          return (y, x);
        }

        count += 1
      }
    }
  }

  (0, 0)
}

fn print_formatted_maze(maze: &Vec<Vec<usize>>)
{
  let height: usize = maze.len();
  let width: usize = maze[0].len();

  let mut line = false;

  for y in 0..height
  {
    for x in 0..width
    {
      if maze[y][x] == EMPTY_NODE || maze[y][x] == ALREADY_VISITED
      {
        print!("   ");
      } else if y == 0
      || y % 2 == 0
      {
        if line
        {
          if maze[y][x] == OPEN_WALL_NODE
          {
            print!("   ");
          } else
          {
            print!("---");
          }
        } else
        {
          print!("+");
        }

        if x != width - 1
        {
          line = !line;
        }
      } else if maze[y][x] == OPEN_WALL_NODE
      {
         print!(" ");
      } else if maze[y][x] == STARTING_NODE
      {
        print!("{}", "█".green());
      } else if maze[y][x] == ENDING_NODE
      {
        print!("{}", "█".red());
      } else if maze[y][x] == PLAYER_CHARACTER_INDICATOR
      {
        print!("{}", "███".bright_blue());
      } else if maze[y][x] == WALL_NODE
      {
         print!("|");
      }
    }

    print!("\r\n");
  }
}

fn generate_maze_using_recursive_backtracking(x: usize, y: usize, maze: &mut Vec<Vec<usize>>)
{
  if maze[y][x] != STARTING_NODE && maze[y][x] != ENDING_NODE
  {
    maze[y][x] = ALREADY_VISITED;
  }

  let mut neighbors = vec![
      (x as i32, y as i32 - 2),
      (x as i32, y as i32 + 2),
      (x as i32 - 2, y as i32),
      (x as i32 + 2, y as i32),
  ];

  neighbors.shuffle(&mut rand::thread_rng());


  for (neighbor_x, neighbor_y) in neighbors
  {
    if neighbor_x < 0 || neighbor_y < 0
    {
      continue;
    }

    let (neighbor_x, neighbor_y) =  (neighbor_x as usize, neighbor_y as usize);

    if neighbor_x >= maze[0].len() || neighbor_y >= maze.len()
    {
      continue;
    }

    if maze[neighbor_y][neighbor_x] != EMPTY_NODE
    {
      continue;
    }

    let wall_x = (x + neighbor_x) / 2;
    let wall_y = (y + neighbor_y) / 2;

    maze[wall_y][wall_x] = OPEN_WALL_NODE;

    generate_maze_using_recursive_backtracking(neighbor_x, neighbor_y, maze);
  }
}

fn clean_up_maze_after_algorith(maze: Vec<Vec<usize>>) -> Vec<Vec<usize>>
{
  let mut maze = maze;
  let height: usize = maze.len();
  let width: usize = maze[0].len();

  for y in  0..height
  {
    for x in 0..width
    {
      if maze[y][x] == ALREADY_VISITED
      {
        maze[y][x] = EMPTY_NODE;
      }
    }
  }

  maze
}


fn make_player_move_based_on_direction(direction_pressed_by_player: KeyCode, mut maze: Vec<Vec<usize>>) -> (Vec<Vec<usize>>, MoveResult)
{
  let height = maze.len();
  let width = maze[0].len();

  for y in 0..height 
  {
    for x in 0..width 
    {
      if maze[y][x] == PLAYER_CHARACTER_INDICATOR 
      {
        match direction_pressed_by_player 
        {
          KeyCode::Up | KeyCode::Char('w') => 
          {
            if (y as i32) - 2 > 0 && maze[y - 2][x] == EMPTY_NODE && maze[y - 1][x] != WALL_NODE
            {
              maze[y][x] = EMPTY_NODE;
              maze[y - 2][x] = PLAYER_CHARACTER_INDICATOR;
              return (maze, MoveResult::Move);
            }
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Down | KeyCode::Char('s') => 
          {
            if y + 2 <= height - 1 && maze[y + 2][x] == EMPTY_NODE &&  maze[y + 1][x] != WALL_NODE 
            {
              maze[y][x] = EMPTY_NODE;
              maze[y + 2][x] = PLAYER_CHARACTER_INDICATOR;
              return (maze, MoveResult::Move);
            }
            
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Left | KeyCode::Char('a') => 
          {
            if x >= 2 && maze[y][x - 2] == EMPTY_NODE && maze[y][x - 1] != WALL_NODE 
            {
              maze[y][x] = EMPTY_NODE;
              maze[y][x - 2] = PLAYER_CHARACTER_INDICATOR;
              return (maze, MoveResult::Move);
            }
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Right | KeyCode::Char('d') => 
          {
            if x + 2 < width 
            {
              if maze[y][x + 2] == EMPTY_NODE && maze[y][x + 1] != WALL_NODE 
              {
                maze[y][x] = EMPTY_NODE;
                maze[y][x + 2] = PLAYER_CHARACTER_INDICATOR;
                return (maze, MoveResult::Move);
              } else if maze[y][x + 2] == ENDING_NODE 
              {
                return (maze, MoveResult::Finished);
              }
            } else if maze[y][x + 1] == ENDING_NODE 
            {
               return (maze, MoveResult::Finished);
            }
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Char('r') => 
          {
            maze[y][x] = EMPTY_NODE;

            for y in 0..height 
            {
              for x in 0..width 
              {
                if maze[y][x] == STARTING_NODE 
                {
                  maze[y][x + 1] = PLAYER_CHARACTER_INDICATOR
                }
              }
            }

            return (maze, MoveResult::Restart);
          }

          KeyCode::Esc =>
          {
             return (maze, MoveResult::Kill);
          },

          _ => return (maze, MoveResult::Invalid),
        }
      }
    }
  }

  (maze, MoveResult::Invalid)
}

fn main()
{
  print!("\x1B[2J\x1B[H");
  enable_raw_mode().unwrap();
  let original_width: usize = 10;
  let original_height: usize = 10;
  let mut maze: Vec<Vec<usize>> =  initial_maze_grid_maker(original_width, original_height);


  let (x_coordinate_where_to_start_generation, y_coordinate_where_to_start_generation) =  random_cell_picker(&maze, original_width);
  let height = maze.len();
  let width = maze[0].len();


  generate_maze_using_recursive_backtracking(x_coordinate_where_to_start_generation, y_coordinate_where_to_start_generation, &mut maze);
  maze = clean_up_maze_after_algorith(maze);
  maze = generate_starting_and_ending_node_position_and_place_player(maze, original_width);

  let mut movement_result_as_string = String::new();

  print!("\x1B[2J\x1B[H");
  print_formatted_maze(&maze);
  stdout().flush().unwrap();

  'game_loop: loop
  { 
    if let Event::Key(event) = read().unwrap() 
    {
      let (new_maze, result) =
        make_player_move_based_on_direction(event.code, maze);
      
      maze = new_maze;
      
      match result 
      {
        MoveResult::Move => {}
        MoveResult::Invalid => {},
        MoveResult::Finished => 
        {
          execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap(); 
          print!("{}", "✩░▒▓▆▅▃▂▁𝐲𝐨𝐮 𝐰𝐢𝐧▁▂▃▅▆▓▒░✩".yellow());
          return;
        },
        MoveResult::Restart => {},
        MoveResult::Kill => break,
      }
    }

    execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();    
    print_formatted_maze(&maze);
    stdout().flush().unwrap();
  }


  execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();
  print_formatted_maze(&maze);

  
}
