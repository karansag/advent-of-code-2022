mod utils;

use std::str::FromStr;

enum CommandType {
    CD,
    LS,
}

struct ParsedCommand {
    cmd: CommandType,
    arg: String,
}

#[derive(Debug, Clone)]
struct File {
    size: i32,
    name: String,
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    // parent: Option<Box<Dir<'a>>>,
}

enum ParsedOutput {
    F(File),
    D(Dir),
}

enum ParsedBash {
    PC(ParsedCommand),
    PO(ParsedOutput),
}

fn parse_command(s: String) -> ParsedCommand {
    let splits: Vec<&str> = s.split_whitespace().collect();
    let command_type = match splits[1] {
        x if x == "cd" => CommandType::CD,
        x if x == "ls" => CommandType::LS,
        _ => panic!("Unsupported command"),
    };
    let arg = splits[2].to_string();
    ParsedCommand {
        cmd: command_type,
        arg: arg,
    }
}

fn parse_output(s: String) -> ParsedOutput {
    let s2 = s.clone();
    let splits: Vec<&str> = s2.split_whitespace().collect();
    if splits[0] == "dir" {
        let d = Dir {
            name: String::from(splits[1]),
        };
        return ParsedOutput::D(d);
    } else {
        let f = File {
            size: i32::from_str(&splits[0]).unwrap(),
            name: String::from(splits[1]),
        };
        return ParsedOutput::F(f);
    }
}

fn parse_line(s: String) -> ParsedBash {
    let s2 = s.clone();
    if s.starts_with('$') {
        let c = parse_command(s2);
        return ParsedBash::PC(c);
    } else {
        let o = parse_output(s2);
        return ParsedBash::PO(o);
    }
}

struct NodeFile<'a> {
    size: i32,
    name: &'a str,
}

struct Node<'a> {
    name: &'a str,
    files: Vec<NodeFile<'a>>,
    parent: Option<&'a mut Node<'a>>,
    children: Vec<&'a mut Node<'a>>,
}

fn process_bash<'a>(bash_lines: &'a Vec<ParsedBash>, base_node: &'a mut Node) -> &'a mut Node<'a> {
    let mut current_node = base_node;
    for line in bash_lines {
        match line {
            ParsedBash::PO(output) => {
                if let ParsedOutput::F(ref file) = output {
                    let f = current_node.files.iter().find(|n| n.name == file.name);
                    if let None = f {
                        let new_file = NodeFile {
                            size: file.size,
                            name: &file.name,
                        };
                        current_node.files.push(new_file);
                        let mut files = &current_node.files;
                        files.push(new_file);
                    }
                }
                if let ParsedOutput::D(ref dir) = output {
                    let d = current_node.children.iter().find(|n| n.name == dir.name);
                    if let None = d {
                        let new_node = Node {
                            name: &dir.name,
                            files: vec![],
                            parent: Some(&mut current_node),
                            children: vec![],
                        };
                        // current_node.files.push(new_file);
                        // let mut files = &current_node.files;
                        // files.push(new_file);
                    }
                }
            }
            ParsedBash::PC(command) => {
                match command.cmd {
                    CommandType::CD => {
                        if command.arg.starts_with('/') {
                            current_node = &mut base_node;
                        }
                        if command.arg == ".." {
                            current_node = current_node.parent.unwrap();
                        } else {
                            let children: Vec<&mut Node> = current_node.children;
                            // let ch_iter: Iterator<Item=&mut Node> = children.iter();
                            let x: Option<&mut Node> =
                                children.iter().find(|n| n.name == command.arg).map(|x| *x);
                            current_node = x.unwrap();
                        }
                    }
                    CommandType::LS => {}
                }
            }
        }
    }

    let mut ret_node = current_node;
    let parent_opt = current_node.parent;
    while let Some(parent) = parent_opt {
        ret_node = parent;
        parent_opt = ret_node.parent;
    }
    return ret_node;
}

// fn calculate_sizes(&Node base_node) {

// }

fn main() -> Result<(), std::io::Error> {
    let file_path = &"../inputs/day-7-input.txt";
    let contents = utils::read_file(file_path)?;
    let parsed_lines: Vec<ParsedBash> = contents
        .map(|l| l.unwrap())
        .map(|s: String| parse_line(s))
        .collect();

    let mut base_node = Node {
        name: &"/",
        files: vec![],
        parent: None,
        children: vec![],
    };
    let directory_tree = process_bash(&parsed_lines, &mut base_node);

    Ok(())
}
