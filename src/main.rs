use rand::{Rng, seq::SliceRandom};
use colored::Colorize;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
enum UserOptions 
{
  Play,
  Solve,
  Quit
}

#[derive(PartialEq, Clone, Copy)]
enum NodeState 
{
  Empty,
  StartPoint,
  EndPoint,
  Visited,
}

#[derive(PartialEq, Clone, Copy)]
enum WallState 
{
  Closed,
  Open,
  Passed,
}

#[derive(PartialEq, Clone, Copy)]
enum Character 
{
  Player,
  Computer,
}

#[derive(PartialEq, Clone, Copy)]
enum Cell 
{
  Wall(WallState),
  Node(NodeState),
  Character(Character)
}

#[derive(Debug)]
enum MoveResult 
{
  Invalid,
  Move,
  Finished,
  Kill,
  Restart,
}

fn clean_up_maze_after_algorith(maze: Vec<Vec<Cell>>) -> Vec<Vec<Cell>>
{
  let mut maze = maze;
  let height: usize = maze.len();
  let width: usize = maze[0].len();

  for y in  0..height
  {
    for x in 0..width
    {
      if maze[y][x] == Cell::Node(NodeState::Visited)
      {
        maze[y][x] = Cell::Node(NodeState::Empty);
      }
    }
  }

  maze
}

fn initial_maze_grid_maker(width: usize, height: usize) -> Vec<Vec<Cell>>
{
  let width: usize = width + width + 1;
  let height: usize = height + height + 1;

  let mut maze: Vec<Vec<Cell>> = vec![vec![Cell::Node(NodeState::Empty); width]; height];
  
  for y in 0..height
  {
    for x in 0..width
    {
      if x == 0
      || x % 2 == 0
      || y == 0
      || y % 2 == 0
      {
        maze[y][x] = Cell::Wall(WallState::Closed);
      }
    }
  }

  maze
}

fn set_up_maze_for_solve(maze: Vec<Vec<Cell>>, original_height: usize, character: Character) -> Vec<Vec<Cell>>
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
          if maze[y][x] == Cell::Wall(WallState::Closed)
          {
            if count_first_row == random_starting_node_number
            {
              maze[y][x] = Cell::Node(NodeState::StartPoint);

              if character == Character::Player
              {  
                maze[y][x + 1] = Cell::Character(Character::Player);
              } else if character == Character::Computer
              {
                maze[y][x + 1] = Cell::Character(Character::Computer);
              }
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
            maze[y][x] = Cell::Node(NodeState::EndPoint);
          }

          count_last_row += 1;
        }
      }
    }
  }

  maze
}

fn random_cell_picker(maze: &Vec<Vec<Cell>>, original_width: usize) -> (usize, usize)
{
  let random_node_number = rand::thread_rng().gen_range(1..=(original_width*original_width));
  let mut count = 1;

  let height: usize = maze.len();
  let width: usize = maze[0].len();

  for y in 0..height
  {
    for x in 0..width
    {
      if maze[y][x] == Cell::Node(NodeState::Empty)
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

fn animate_path_movement(duration: Duration, maze: &Vec<Vec<Cell>>) 
{
  print!("\x1B[2J\x1B[H");
  print_maze(maze);
  stdout().flush().unwrap();
  thread::sleep(duration);
}

fn print_maze(maze: &Vec<Vec<Cell>>)
{
  let height: usize = maze.len();
  let width: usize = maze[0].len();

  let mut line = false;

  for y in 0..height
  {
    for x in 0..width
    {
      if maze[y][x] == Cell::Node(NodeState::Empty)
      {
        print!("   ");
      } else if maze[y][x] == Cell::Node(NodeState::Visited)
      {
         print!("{}", "▇▇▇".bright_yellow());
      } else if y == 0 || y % 2 == 0
      {
        if line
        {
          if maze[y][x] == Cell::Wall(WallState::Open)
          {
            print!("   ");
          } else if maze[y][x] == Cell::Wall(WallState::Passed) 
          {
            print!("{}", "▇▇▇".bright_yellow());
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
      } else if maze[y][x] == Cell::Wall(WallState::Open)
      {
        print!(" ");
      } else if maze[y][x] == Cell::Node(NodeState::StartPoint)
      {
        print!("{}", "█".green());
      } else if maze[y][x] == Cell::Node(NodeState::EndPoint)
      {
        print!("{}", "█".red());
      } else if maze[y][x] == Cell::Character(Character::Player) ||  maze[y][x] == Cell::Character(Character::Computer)
      {
        print!("{}", "███".bright_yellow());
      } else if maze[y][x] == Cell::Wall(WallState::Passed)
      {
        print!("{}", "█".bright_yellow());
      } else if maze[y][x] == Cell::Wall(WallState::Closed)
      {
        print!("|");
      }
    }

    print!("\r\n");
  }
}

fn maze_generator(x: usize, y: usize, maze: &mut Vec<Vec<Cell>>)
{
  if maze[y][x] != Cell::Node(NodeState::StartPoint) && maze[y][x] != Cell::Node(NodeState::EndPoint)
  {
    maze[y][x] = Cell::Node(NodeState::Visited);
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

    if maze[neighbor_y][neighbor_x] != Cell::Node(NodeState::Empty)
    {
      continue;
    }

    let wall_x = (x + neighbor_x) / 2;
    let wall_y = (y + neighbor_y) / 2;

    maze[wall_y][wall_x] = Cell::Wall(WallState::Open);

    maze_generator(neighbor_x, neighbor_y, maze);
  }
}


fn solve_maze(x: usize, y: usize, maze: &mut Vec<Vec<Cell>>) -> bool
{
  if maze[y][x + 1] == Cell::Node(NodeState::EndPoint)
  {
    return true;
  }
  
  let neighbors = calculate_heuristic_score_for_position(x, y, maze);

  for (neighbor_x, neighbor_y) in neighbors 
  {
    let middle_x = (x + neighbor_x) / 2;
    let middle_y = (y + neighbor_y) / 2;

    let old_current = maze[y][x];
    let old_middle = maze[middle_y][middle_x];
    let old_next = maze[neighbor_y][neighbor_x];

    maze[y][x] = Cell::Node(NodeState::Visited);
    animate_path_movement(Duration::from_millis(12), &maze);
    
    maze[middle_y][middle_x] = Cell::Wall(WallState::Passed);
    animate_path_movement(Duration::from_millis(12), &maze);
    
    maze[neighbor_y][neighbor_x] = Cell::Character(Character::Computer);
    animate_path_movement(Duration::from_millis(25), &maze);
    
    if solve_maze(neighbor_x, neighbor_y, maze) 
    {
      return true;
    }

    maze[y][x] = old_current;
    animate_path_movement(Duration::from_millis(12), &maze);
    
    maze[middle_y][middle_x] = old_middle;
    animate_path_movement(Duration::from_millis(12), &maze);
    
    maze[neighbor_y][neighbor_x] = old_next;
    animate_path_movement(Duration::from_millis(12), &maze);
  }
  false
}

fn move_player(direction: KeyCode, mut maze: Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, MoveResult)
{
  let height = maze.len();
  let width = maze[0].len();

  for y in 0..height 
  {
    for x in 0..width 
    {
      if maze[y][x] == Cell::Character(Character::Player)
      {
        match direction
        {
          KeyCode::Up | KeyCode::Char('w') => 
          {
            if (y as i32) - 2 > 0 && maze[y - 2][x] == Cell::Node(NodeState::Empty) && maze[y - 1][x] != Cell::Wall(WallState::Closed)
            {
              maze[y][x] = Cell::Node(NodeState::Empty);
              maze[y - 2][x] = Cell::Character(Character::Player);
              return (maze, MoveResult::Move);
            }
            
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Down | KeyCode::Char('s') => 
          {
            if y + 2 <= height - 1 && maze[y + 2][x] == Cell::Node(NodeState::Empty) &&  maze[y + 1][x] != Cell::Wall(WallState::Closed) 
            {
              maze[y][x] = Cell::Node(NodeState::Empty);
              maze[y + 2][x] = Cell::Character(Character::Player);
              return (maze, MoveResult::Move);
            }
            
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Left | KeyCode::Char('a') => 
          {
            if x >= 2 && maze[y][x - 2] == Cell::Node(NodeState::Empty) && maze[y][x - 1] != Cell::Wall(WallState::Closed) 
            {
              maze[y][x] = Cell::Node(NodeState::Empty);
              maze[y][x - 2] = Cell::Character(Character::Player);
              return (maze, MoveResult::Move);
            }
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Right | KeyCode::Char('d') => 
          {
            if x + 2 < width 
            {
              if maze[y][x + 2] == Cell::Node(NodeState::Empty) && maze[y][x + 1] != Cell::Wall(WallState::Closed)
              {
                maze[y][x] = Cell::Node(NodeState::Empty);
                maze[y][x + 2] = Cell::Character(Character::Player);
                return (maze, MoveResult::Move);
              }
            } else if maze[y][x + 1] == Cell::Node(NodeState::EndPoint)
            {
              return (maze, MoveResult::Finished);
            }
            return (maze, MoveResult::Invalid);
          }

          KeyCode::Char('r') => 
          {
            maze[y][x] = Cell::Node(NodeState::Empty);

            for y in 0..height 
            {
              for x in 0..width 
              {
                if maze[y][x] == Cell::Node(NodeState::StartPoint)
                {
                  maze[y][x + 1] = Cell::Character(Character::Player);
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

fn show_menu(selected_option: UserOptions, maze: &Vec<Vec<Cell>>) 
{
  execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();    
  print_maze(&maze);
  stdout().flush().unwrap();
  
  println!("+-------------------------------+\r\n");
  println!("+  Welcome to the Maze Program  +\r\n");
  println!("+-------------------------------+\r\n");

  let options = 
  [
    (UserOptions::Solve, "1 - Solve the Maze"),
    (UserOptions::Play, "2 - Play the Maze"),
    (UserOptions::Quit, "3 - Quit Application")
  ];

  for &option in options.iter()
  {
    if option.0 == selected_option
    {
      print!("-> {}\r\n", option.1.green());
      continue;
    }

    print!("{}\r\n", option.1);
  }
} 

fn calculate_heuristic_score_for_position(x: usize, y: usize, maze: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> 
{
  let height = maze.len();
  let width = maze[0].len();

  let mut ending_position_x = 0;
  let mut ending_position_y = 0;
  
  for row in 0..height 
  {
    for col in 0..width 
    {
      if maze[row][col] == Cell::Node(NodeState::EndPoint) 
      {
        ending_position_x = col;
        ending_position_y = row;
      }
    }
  };

  let neighbors = vec![
    (x as i32, y as i32 - 2),
    (x as i32, y as i32 + 2),
    (x as i32 - 2, y as i32),
    (x as i32 + 2, y as i32),
  ];

  let mut neighbors_with_heuristic: Vec<(usize, usize, i32)> = Vec::new();

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

    if maze[neighbor_y][neighbor_x] != Cell::Node(NodeState::Empty)
    {
      continue;
    }

    let middle_x = (x + neighbor_x) / 2;
    let middle_y = (y + neighbor_y) / 2;

    if maze[middle_y][middle_x] == Cell::Wall(WallState::Closed) 
    {
      continue;
    }

    let amount = amount_of_walls_next_to_any_given_cell(neighbor_x, neighbor_y, maze);

    let heuristic = (ending_position_x as i32 - neighbor_x as i32).abs() + (ending_position_y as i32 - neighbor_y as i32).abs();

    if amount >= 3 
    {
      continue
    }
    
    neighbors_with_heuristic.push((neighbor_x, neighbor_y, heuristic));
  }

  neighbors_with_heuristic.sort_by(|a, b| 
  {
    a.2.cmp(&b.2)
  });

  let neighbors: Vec<(usize, usize)> = neighbors_with_heuristic.into_iter()
    .map(|tuple| (tuple.0, tuple.1)).collect();

  neighbors
  
} 

fn amount_of_walls_next_to_any_given_cell(x: usize, y: usize, maze: &Vec<Vec<Cell>>) -> usize 
{

  let mut count = 0;
  
  if maze[y][x + 1] == Cell::Wall(WallState::Closed)
  {
    count += 1;
  }

  if maze[y][x - 1] == Cell::Wall(WallState::Closed)
  {
    count += 1;
  }

  if maze[y + 1][x] == Cell::Wall(WallState::Closed)
  {
    count += 1;
  }

  if maze[y - 1][x] == Cell::Wall(WallState::Closed)
  {
    count += 1;
  }
  
  count
}

fn main()
{
  enable_raw_mode().unwrap();
  let original_width: usize = 20;
  let original_height: usize = 20;
  let mut maze: Vec<Vec<Cell>> = initial_maze_grid_maker(original_width, original_height);


  let (x_coordinate_where_to_start_generation, y_coordinate_where_to_start_generation) = random_cell_picker(&maze, original_width);
  let height = maze.len();
  let width = maze[0].len();

  maze_generator(x_coordinate_where_to_start_generation, y_coordinate_where_to_start_generation, &mut maze);
  maze = clean_up_maze_after_algorith(maze);

  let mut selected_option = UserOptions::Play;
  show_menu(selected_option, &maze);

  loop 
  {
    if let Event::Key(event) = read().unwrap() 
    {
      match event.code 
      {
        KeyCode::Esc => 
        {
          std::process::exit(0);
        },
  
        KeyCode::Up => 
        {
          match selected_option 
          {
            UserOptions::Solve => selected_option = UserOptions::Quit,
            UserOptions::Play => selected_option = UserOptions::Solve,
            UserOptions::Quit => selected_option = UserOptions::Play,
          }
          
          show_menu(selected_option, &maze);
        },
  
        KeyCode::Down => 
        {
          match selected_option 
          {
            UserOptions::Solve => selected_option = UserOptions::Play,
            UserOptions::Play => selected_option = UserOptions::Quit,
            UserOptions::Quit => selected_option = UserOptions::Solve,
          }
          
          show_menu(selected_option, &maze);
        },
  
        KeyCode::Enter => 
        {
          break;
        },
  
        _ => {},
      }
    }
  }

  match selected_option
  {
    UserOptions::Solve => 
    {
      maze = set_up_maze_for_solve(maze, original_width, Character::Computer);
      
      print!("\x1B[2J\x1B[H");
      print_maze(&maze);
      stdout().flush().unwrap();

      let (mut starting_x, mut starting_y) = (0, 0);
      
      for y in 0..height
      {
        for x in 0..width 
        {
          if maze[y][x] == Cell::Character(Character::Computer)
          {
            starting_x = x;
            starting_y = y;
          }
        }
      }
      
      solve_maze(starting_x, starting_y, &mut maze);
      print!("\x1B[2J\x1B[H");
      print_maze(&maze);
      stdout().flush().unwrap();
      return;
    },

    UserOptions::Play => 
    {
      maze = set_up_maze_for_solve(maze, original_width, Character::Player);
      
      print!("\x1B[2J\x1B[H");
      print_maze(&maze);
      stdout().flush().unwrap();
      
      'game_loop: loop
      { 
        if let Event::Key(event) = read().unwrap() 
        {
          let (new_maze, result) = move_player(event.code, maze);
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
            MoveResult::Kill => break 'game_loop,
          }
        }
        
        execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();    
        print_maze(&maze);
        stdout().flush().unwrap();
      }
      
      
      execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();
      print_maze(&maze);
      return;
    },

    UserOptions::Quit => 
    {
      std::process::exit(0);
    },
  }  
}
