use crate::Rock;

use super::prelude::{Map, Pos};

struct AllPositions {
    pos_for_north_tilt: Vec<Pos>,
    pos_for_west_tilt: Vec<Pos>,
    pos_for_south_tilt: Vec<Pos>,
    pos_for_east_tilt: Vec<Pos>,
}

impl From<&Map> for AllPositions {
    fn from(map: &Map) -> Self {
        let mut all_pos = map.all_pos();
        all_pos.retain(|pos| map.get(pos) != Some(&Rock::SquareRock));
        let mut all_pos_horizontal = all_pos.clone();

        all_pos_horizontal.sort_by_key(|pos| pos.x * 1000 + pos.y);

        let mut pos = Self {
            pos_for_north_tilt: all_pos.clone(),
            pos_for_south_tilt: {
                all_pos.reverse();
                all_pos
            },
            pos_for_west_tilt: all_pos_horizontal.clone(),
            pos_for_east_tilt: {
                all_pos_horizontal.reverse();
                all_pos_horizontal
            },
        };

        // Remove first lines in each Vec because nothing can move further;
        pos.pos_for_north_tilt.retain(|pos| pos.y != 0);
        pos.pos_for_west_tilt.retain(|pos| pos.x != 0);
        pos.pos_for_south_tilt
            .retain(|pos| pos.y != map.height() - 1);
        pos.pos_for_east_tilt.retain(|pos| pos.x != map.width() - 1);

        pos
    }
}
