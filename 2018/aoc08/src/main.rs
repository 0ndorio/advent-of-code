use std::{
    error::Error,
    io::{self, Read},
    str::FromStr,
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let root: Node = input.parse()?;

    let sum = root.sum_meta_data();
    println!("Meta value sum: {}", sum);

    let sum = root.calc_value();
    println!("Value of the root node: {}", sum);

    Ok(())
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    children: Vec<Node>,
    meta_data: Vec<u32>,
}

impl Node {
    fn from_iterator(mut input: &mut dyn Iterator<Item = &str>) -> Result<Self> {
        let num_children: usize = input
            .next()
            .ok_or_else(|| "Couldn't parse number of children from header.")?
            .parse()?;

        let num_meta_data: usize = input
            .next()
            .ok_or_else(|| "Couldn't parse number of meta data entries from header.")?
            .parse()?;

        let mut children = vec![];
        for _ in 0..num_children {
            let child: Node = Node::from_iterator(&mut input)?;
            children.push(child);
        }

        let mut meta_data = vec![];
        for _ in 0..num_meta_data {
            let data: u32 = input
                .next()
                .ok_or_else(|| "Couldn't parse an expected meta value.")?
                .parse()?;

            meta_data.push(data);
        }

        Ok(Node {
            children,
            meta_data,
        })
    }

    fn sum_meta_data(&self) -> u32 {
        let mut value = 0u32;
        let mut outstanding_nodes = vec![self];

        while let Some(node) = outstanding_nodes.pop() {
            value += node.meta_data.iter().sum::<u32>();

            let mut child_references = node.children.iter().collect();
            outstanding_nodes.append(&mut child_references);
        }

        value
    }

    fn calc_value(&self) -> u32 {
        if self.children.is_empty() {
            return self.meta_data.iter().sum();
        }

        self.meta_data
            .iter()
            .filter(|&&value| value > 0)
            .map(|&value| {
                self.children
                    .get((value - 1) as usize)
                    .map_or(0, Node::calc_value)
            })
            .sum()
    }
}

impl FromStr for Node {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        Self::from_iterator(&mut input.split_whitespace())
    }
}
