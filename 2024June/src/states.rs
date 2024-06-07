use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub name: String,
    pub population: u32,
}

lazy_static::lazy_static! {
    pub static ref STATES: Vec<State> = vec![
        State {
            name: "ALABAMA".to_string(),
            population: 5024279
        },
        State {
            name: "ALASKA".to_string(),
            population: 733391
        },
        State {
            name: "ARIZONA".to_string(),
            population: 7151502
        },
        State {
            name: "ARKANSAS".to_string(),
            population: 3011524
        },
        State {
            name: "CALIFORNIA".to_string(),
            population: 39538223
        },
        State {
            name: "COLORADO".to_string(),
            population: 5773714
        },
        State {
            name: "CONNECTICUT".to_string(),
            population: 3605944
        },
        State {
            name: "DELAWARE".to_string(),
            population: 989948
        },
        State {
            name: "FLORIDA".to_string(),
            population: 21538187
        },
        State {
            name: "GEORGIA".to_string(),
            population: 10711908
        },
        State {
            name: "HAWAII".to_string(),
            population: 1455271
        },
        State {
            name: "IDAHO".to_string(),
            population: 1839106
        },
        State {
            name: "ILLINOIS".to_string(),
            population: 12812508
        },
        State {
            name: "INDIANA".to_string(),
            population: 6785528
        },
        State {
            name: "IOWA".to_string(),
            population: 3190369
        },
        State {
            name: "KANSAS".to_string(),
            population: 2937880
        },
        State {
            name: "KENTUCKY".to_string(),
            population: 4505836
        },
        State {
            name: "LOUISIANA".to_string(),
            population: 4657757
        },
        State {
            name: "MAINE".to_string(),
            population: 1362359
        },
        State {
            name: "MARYLAND".to_string(),
            population: 6177224
        },
        State {
            name: "MASSACHUSETTS".to_string(),
            population: 7029917
        },
        State {
            name: "MICHIGAN".to_string(),
            population: 10077331
        },
        State {
            name: "MINNESOTA".to_string(),
            population: 5706494
        },
        State {
            name: "MISSISSIPPI".to_string(),
            population: 2961279
        },
        State {
            name: "MISSOURI".to_string(),
            population: 6154913
        },
        State {
            name: "MONTANA".to_string(),
            population: 1084225
        },
        State {
            name: "NEBRASKA".to_string(),
            population: 1961504
        },
        State {
            name: "NEVADA".to_string(),
            population: 3104614
        },
        State {
            name: "NEWHAMPSHIRE".to_string(),
            population: 1377529
        },
        State {
            name: "NEWJERSEY".to_string(),
            population: 9288994
        },
        State {
            name: "NEWMEXICO".to_string(),
            population: 2117522
        },
        State {
            name: "NEWYORK".to_string(),
            population: 20201249
        },
        State {
            name: "NORTHCAROLINA".to_string(),
            population: 10439388
        },
        State {
            name: "NORTHDAKOTA".to_string(),
            population: 779094
        },
        State {
            name: "OHIO".to_string(),
            population: 11799448
        },
        State {
            name: "OKLAHOMA".to_string(),
            population: 3959353
        },
        State {
            name: "OREGON".to_string(),
            population: 4237256
        },
        State {
            name: "PENNSYLVANIA".to_string(),
            population: 13002700
        },
        State {
            name: "RHODEISLAND".to_string(),
            population: 1097379
        },
        State {
            name: "SOUTHCAROLINA".to_string(),
            population: 5118425
        },
        State {
            name: "SOUTHDAKOTA".to_string(),
            population: 886667
        },
        State {
            name: "TENNESSEE".to_string(),
            population: 6910840
        },
        State {
            name: "TEXAS".to_string(),
            population: 29145505
        },
        State {
            name: "UTAH".to_string(),
            population: 3271616
        },
        State {
            name: "VERMONT".to_string(),
            population: 643077
        },
        State {
            name: "VIRGINIA".to_string(),
            population: 8631393
        },
        State {
            name: "WASHINGTON".to_string(),
            population: 7705281
        },
        State {
            name: "WESTVIRGINIA".to_string(),
            population: 1793716
        },
        State {
            name: "WISCONSIN".to_string(),
            population: 5893718
        },
        State {
            name: "WYOMING".to_string(),
            population: 576851
        }
    ];
}
