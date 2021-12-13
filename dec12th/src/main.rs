use indextree::{Arena, NodeEdge, NodeId};

const START: &str = "start";
const END: &str = "end";
 
fn main() {
    let mut links = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    links.sort();
    let links = links
        .iter()
        .map(|x| {
            let process = |s: String| {
                if s.chars().next().unwrap().is_ascii_uppercase() {
                    (s, true) // Big cave: Reentrant
                } else {
                    (s, false) // Small cave
                }
            };
            let mut iter = x.split("-");
            (
                process(iter.next().unwrap().to_string()),
                process(iter.next().unwrap().to_string()),
            )
        })
        .collect::<Vec<((String, bool), (String, bool))>>();

    let mut path = Arena::new();
    let root = path.new_node(START);
    println!("1: {}", fill(&mut path, (root, START), &links, false));
    //printtree(&path, root);

    let mut path = Arena::new();
    let root = path.new_node(START);
    println!("2: {}", fill(&mut path, (root, START), &links, true));
    //printtree(&path, root);
}

fn fill<'a>(
    path: &mut Arena<&'a str>,
    handle: (NodeId, &str),
    links: &'a Vec<((String, bool), (String, bool))>,
    usetwice: bool,
) -> u32 {
    fn isparent<'a>(path: &mut Arena<&'a str>, item: &'a str, handle: NodeId) -> bool {
        let node = path.get(handle).unwrap();
        if node.get() == &item {
            return true;
        } else {
            if let Some(x) = node.parent() {
                return isparent(path, item, x);
            } else {
                return false;
            }
        }
    }

    fn istwiceused(path: &mut Arena<&str>, handle: NodeId) -> bool {
        let mut items = Vec::new();
        let mut handle = handle;
        loop {
            let node = path.get(handle).unwrap();
            items.push(node.get());
            if let Some(x) = node.parent() {
                handle = x;
            } else {
                break;
            }
        }
        if items.len() > 1 {
            items.sort();
            let repeated = items
                .windows(2)
                .filter(|x| x[0] == x[1])
                .map(|x| *x[0])
                .collect::<Vec<&str>>();
            for item in repeated {
                if item.chars().next().unwrap().is_ascii_lowercase() {
                    return true;
                }
            }
        }
        false
    }

    let mut completedpaths = 0;

    let dests: Vec<&(String, bool)> = links
        .iter()
        .filter(|l| l.0 .0 == handle.1 || l.1 .0 == handle.1)
        .map(|l| if l.0 .0 == handle.1 { &l.1 } else { &l.0 })
        .collect();

    for dest in dests {
        if dest.1 == true
            || (dest.1 == false && !isparent(path, &dest.0, handle.0))
            || (usetwice
                &&dest.1 == false
                && &dest.0 != START
                && isparent(path, &dest.0, handle.0)
                && !istwiceused(path, handle.0))
        {
            let child_nodeid = path.new_node(&dest.0);
            handle.0.append(child_nodeid, path);
            if &dest.0 != END {
                completedpaths += fill(path, (child_nodeid, &dest.0), links, usetwice);
            } else {
                completedpaths += 1;
            }
        }
    }
    completedpaths
}

#[allow(dead_code)]
fn printtree(
    path: &Arena<&str>,
    root: NodeId) {
        let mut depth = 0;
        for item in root.traverse(&path) {
            match item {
                NodeEdge::Start(nodeid) => {
                    let node = path.get(nodeid).unwrap().get();
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("{:?}", node);
                    depth += 1;
                }
                NodeEdge::End(_) => {
                    depth -= 1;
                }
            }
        }

}