const MAX_RESOLUTION: usize = 11;

fn main() {
    let mut current_graph = [[0.0; MAX_RESOLUTION]; MAX_RESOLUTION];

    let mut current_general_number = 0;
    for row in current_graph.iter_mut() {
        let current_team_two_win_chance =
            current_general_number as f32 / (MAX_RESOLUTION - 1) as f32;
        let mut current_number = current_general_number;
        for item in row.iter_mut() {
            let current_team_one_win_chance =
                (current_number - current_general_number) as f32 / (MAX_RESOLUTION - 1) as f32;
            *item = (current_team_two_win_chance * current_team_one_win_chance)
                + ((1.0 - current_team_one_win_chance) * (1.0 - current_team_two_win_chance));
            current_number += 1;
        }
        current_general_number += 1;
    }

    for row in current_graph.iter().rev() {
        for item in row.iter() {
            print!("{:.2}\t", item);
        }
        println!()
    }

    let mut minimal_value = current_graph.first().unwrap().first().unwrap();

    for row in current_graph.iter() {
        for item in row.iter() {
            if item <= minimal_value {
                minimal_value = item;
            }
        }
    }

    println!("Valor mínimo de taxa de empate: {}", minimal_value);

    println!("Tabela dos valores do empate por vitoria\n");

    let mut current_general_number = 0;
    for row in current_graph.iter_mut() {
        let current_team_two_win_chance =
            current_general_number as f32 / (MAX_RESOLUTION - 1) as f32;
        let mut current_number = current_general_number;
        for item in row.iter_mut() {
            let current_team_one_win_chance =
                (current_number - current_general_number) as f32 / (MAX_RESOLUTION - 1) as f32;
            *item = current_team_two_win_chance * current_team_one_win_chance;
            current_number += 1;
        }
        current_general_number += 1;
    }

    for row in current_graph.iter().rev() {
        for item in row.iter() {
            print!("{:.2}\t", item);
        }
        println!()
    }
}
