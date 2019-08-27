use std::collections::HashSet;
use std::iter::FromIterator;

use std::cmp::{Ord, Ordering};

fn main() {
    let path = "input.txt";
    let input =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to open: {}", path));

    // test input
    //     let input = String::from(
    // r#"London to Dublin = 464
    // London to Belfast = 518
    // Dublin to Belfast = 141"#,
    //     );

    let mut edges = parse_input(input);
    add_reverse_routes(&mut edges);
    let cities: HashSet<String> =
        HashSet::from_iter(edges.iter().map(|edge| edge.city1.clone()).into_iter());

    let mut trees: Vec<RouteTreeNode> = cities.iter().map(|c| RouteTreeNode::root(c)).collect();

    //build trees
    for mut root in trees.iter_mut() {
        let city_name = root.city_name.clone();
        build_tree(&mut root, vec![city_name], &edges);
    }

    let route_iter = trees.iter().map(|t| t.list_full_routes(0)).flatten();
    let route_iter_copy = route_iter.clone();
    let min_route = route_iter_copy.min();
    let max_route = route_iter.max();
    println!("shortest Route: {}", min_route.unwrap().path);
    println!("longest Route: {}", max_route.unwrap().path);
}

#[derive(Clone, Debug)]
struct CityEdge {
    city1: String,
    city2: String,
    weight: i32,
}

impl CityEdge {
    pub fn new(city1: &str, city2: &str, weight: i32) -> CityEdge {
        CityEdge {
            city1: String::from(city1),
            city2: String::from(city2),
            weight,
        }
    }

    pub fn reverse_route(c: &CityEdge) -> CityEdge {
        CityEdge {
            city1: c.city2.clone(),
            city2: c.city1.clone(),
            weight: c.weight,
        }
    }
}

#[derive(Debug)]
struct RouteTreeNode {
    city_name: String,
    travel_weight: i32,
    kids: Vec<RouteTreeNode>,
}

#[derive(Debug, Eq, Clone)]
struct Route {
    path: String,
    cost: i32,
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl RouteTreeNode {
    pub fn root(city_name: &str) -> RouteTreeNode {
        RouteTreeNode {
            city_name: String::from(city_name),
            travel_weight: 0,
            kids: Vec::new(),
        }
    }
    pub fn new(city_name: &str, weight: i32) -> RouteTreeNode {
        RouteTreeNode {
            city_name: String::from(city_name),
            travel_weight: weight,
            kids: Vec::new(),
        }
    }

    pub fn list_full_routes(&self, prev_route_sum: i32) -> Vec<Route> {
        let mut all_routes = Vec::new();
        let mut this_route = format!(
            "{} {}",
            &self.city_name,
            if self.kids.is_empty() { "= " } else { "-> " }
        );

        let route_sum = prev_route_sum + self.travel_weight;
        if self.kids.len() == 0 {
            this_route.push_str(&format!("{}", route_sum));
            // all_routes.push(this_route);
            all_routes.push(Route {
                path: this_route,
                cost: route_sum,
            })
        } else {
            for subroute in &self.kids {
                let described_routes: Vec<Route> = subroute
                    .list_full_routes(route_sum)
                    .iter()
                    .map(|route| Route {
                        path: format!("{}{}", this_route, route.path),
                        cost: route.cost,
                    })
                    .collect();
                for desc_route in described_routes {
                    all_routes.push(desc_route);
                }
            }
        }

        all_routes
    }
}

fn add_reverse_routes(given_routes: &mut Vec<CityEdge>) {
    let mut reversed_routes: Vec<CityEdge> = given_routes
        .iter()
        .map(|c| CityEdge::reverse_route(c))
        .collect();

    given_routes.append(&mut reversed_routes);
}

fn build_tree(root: &mut RouteTreeNode, exclude: Vec<String>, edge_list: &Vec<CityEdge>) {
    let root_city = &(root.city_name);
    //get a list of cities adjacencies
    let adjacent: Vec<&CityEdge> = edge_list
        .iter()
        .filter(|city_edge| city_edge.city1 == *root_city) // limit to cities originating from our last node
        .filter(|city_edge| !exclude.contains(&city_edge.city2)) // make sure we haven't been there before
        .collect();

    for city_edge in adjacent {
        let mut subroot = RouteTreeNode::new(&city_edge.city2, city_edge.weight);
        let mut exclude_copy = exclude.clone();
        exclude_copy.push(city_edge.city2.clone());

        build_tree(&mut subroot, exclude_copy, edge_list);
        root.kids.push(subroot);
    }
}

fn parse_input(file_contents: String) -> Vec<CityEdge> {
    let mut rtn = Vec::new();

    for line in file_contents.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts.as_slice() {
            [city1, "to", city2, "=", distance] => rtn.push(CityEdge::new(
                city1,
                city2,
                distance.parse::<i32>().unwrap(),
            )),
            _ => panic!("This doesn't look right..."),
        }
    }

    rtn
}
