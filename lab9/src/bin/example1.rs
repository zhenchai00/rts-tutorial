use crate::Team::ManCity;
enum Team {
    Arsenal,
    ManUtd,
    Chelsea,
    ManCity,
}

fn value_of_team(team: Team) -> String {
    match team {
        Team::Arsenal => String::from("rubbish"),
        Team::ManUtd => String::from("rubbish rubbish"),
        Team::Chelsea => String::from("extremely rubbish, i fell sick just think about them"),
        Team::ManCity => String::from("greatest team ever"),
    }
}

fn main() {
    println!("{}", value_of_team(ManCity));
}
