use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::{anyhow, Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{eof, opt, recognize},
    error::{convert_error, VerboseError},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult, Parser,
};

type NomResult<T, U> = IResult<T, U, VerboseError<T>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/07.txt").context("Error reading input file")?;

    // get a map of "outer bag -> inner bags"
    let outer_to_inner = input
        .lines()
        .map(|line| nom_parse(bag_line, line))
        .collect::<Result<HashMap<_, _>, _>>()?;

    // invert map to get map of "inner bag -> outer bags"
    let inner_to_outer = outer_to_inner
        .iter()
        .flat_map(|(outer, inner)| inner.iter().map(|bag| (bag, outer)).collect::<Vec<_>>())
        .fold(HashMap::new(), |mut map, (&inner, &outer)| {
            map.entry(inner).or_insert_with(Vec::new).push(outer);
            map
        });

    let containing_gold = get_containing_bags(&inner_to_outer, "shiny gold", HashSet::new());
    println!("Part 1: {}", containing_gold.len());

    Ok(())
}

fn get_containing_bags<'a>(
    all_bags: &'a HashMap<&str, Vec<&str>>,
    bag: &'a str,
    mut candidates: HashSet<&'a str>,
) -> HashSet<&'a str> {
    if let Some(outer_bags) = all_bags.get(bag) {
        for outer_bag in outer_bags {
            candidates.insert(outer_bag);
            candidates = get_containing_bags(all_bags, outer_bag, candidates);
        }
    }
    candidates
}

fn nom_parse<I, O, P>(parser: P, input: I) -> Result<O>
where
    I: Copy + nom::InputLength + std::ops::Deref<Target = str>,
    P: Parser<I, O, VerboseError<I>>,
{
    match Finish::finish(terminated(parser, eof)(input)) {
        Ok((_, result)) => Ok(result),
        Err(err) => Err(anyhow!("{}", convert_error(input, err))),
    }
}

fn bag_line(input: &str) -> NomResult<&str, (&str, Vec<&str>)> {
    terminated(
        separated_pair(bag, tag(" contain "), contained_bags),
        char('.'),
    )(input)
}

fn contained_bags(input: &str) -> NomResult<&str, Vec<&str>> {
    match tag::<_, _, VerboseError<_>>("no other bags")(input) {
        Ok((next, _)) => Ok((next, Vec::new())),
        Err(_) => separated_list1(tag(", "), bag_with_count)(input),
    }
}

fn bag_with_count(input: &str) -> NomResult<&str, &str> {
    separated_pair(digit1, char(' '), bag)(input).map(|(next, (_count, colour))| (next, colour))
}

fn bag(input: &str) -> NomResult<&str, &str> {
    let (input, colour) = recognize(separated_pair(alpha1, char(' '), alpha1))(input)?;
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = opt(char('s'))(input)?;
    Ok((input, colour))
}
