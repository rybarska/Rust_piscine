enum Command {
    Todo(String),   // Command: "TODO"
    Done(usize),    // Command: "DONE"
    Purge,          // Command: "PURGE"
    Quit,           // Command: "QUIT"
}

impl Command {
    fn prompt() -> Self
    {
        loop
        {
            println!("Enter command: ");
            let mut line = ftkit::read_line();
            let mut trimmed_line = line.trim();
            while trimmed_line.len() < 4
            {
                println!("Bro, enter command: ");
                line = ftkit::read_line();
                if line.is_empty()
                { return Command::Quit; }
                trimmed_line = line.trim();
            }
            println!("You say: {}", trimmed_line);
            let command_start = &trimmed_line[0..4];
            if trimmed_line == "QUIT"
                { return Command::Quit; }
            else if trimmed_line.len() > 4 && trimmed_line == "PURGE"
                { return Command::Purge; }
            else if command_start == "TODO"
            {
                if trimmed_line.len() < 5
                    { println!("Invalid input"); }
                else
                {
                    let gist = &trimmed_line[5..];
                    return Command::Todo(gist.to_string());
                }
            }
            else if command_start == "DONE"
            {
                if trimmed_line.len() < 5
                    { println!("Invalid input"); }
                else
                {
                    let index_string = &trimmed_line[5..].to_string();
                    let index = index_string.parse::<usize>();
                    match index
                    {
                        Ok(index) => return Command::Done(index),
                        Err(_index) =>
                        {
                            println!("Invalid index");
                        },
                    }
                }
            }
            else {
                println!("Invalid input");
            }
        }
    }
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self
    {
        Self {todos: Vec::new(), dones: Vec::new()}
    }

    fn display(&self)
    {
        for t in 0..self.todos.len()
        {
            println!("todos:");
            println!("  {:?}", self.todos[t]);
        }
        for d in 0..self.dones.len()
        {
            println!("dones:");
            println!("  {:?}", self.dones[d]);
        }
    }

    fn add(&mut self, todo: String)
    {
        self.todos.push(todo);
    }

    fn done(&mut self, index: usize)
    {
        if index < self.todos.len()
        {
            let task = self.todos.remove(index);
            self.dones.push(task);
        }
        else {
            println!("Invalid index");
        }
    }

    fn purge(&mut self)
    {
        if !self.dones.is_empty()
        {
            self.dones.clear();
        }
        else {
            println!("Nothing to purge");
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let mut _todo_len = 0;
    let mut _done_len = 0;
    loop {
        match Command::prompt()
        {
            Command::Todo(what) =>
            {
                _todo_len += 1;
                todo_list.add(what)
            },
            Command::Done(index) =>
            {
                if index < _todo_len && _todo_len > 0
                {
                    _todo_len -= 1;
                    _done_len += 1;
                    todo_list.done(index)
                }
                else {
                    println!("Invalid index");
                }
            },
            Command::Purge =>
            {
                if _done_len > 0
                {
                    _done_len = 0;
                    todo_list.purge()
                }
                else {
                    println!("Nothing to purge");
                }
            },
            Command::Quit =>
            {
                println!("Program is quitting...");
                break ;
            }
        }
    }
    todo_list.display();
}
