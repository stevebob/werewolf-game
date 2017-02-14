use ecs::*;
use game::*;
use game::data::*;
use game::terrain::util;
use coord::Coord;

const START_COORD: Coord = Coord { x: 18, y: 14 };

pub fn demo_a<S: TurnScheduleQueue>(ids: &EntityIdReserver,
                                  rng: &GameRng,
                                  schedule: &mut S,
                                  g: &mut EcsAction) -> TerrainMetadata {

    let level_switch = LevelSwitch {
        terrain_type: TerrainType::DemoB,
    };
    let (width, height) = util::terrain_from_strings(&level_str(), Some(level_switch), ids, schedule, g);

    util::generate_clouds(width, height, ids, rng, schedule, g);

    TerrainMetadata {
        width: width,
        height: height,
        start_coord: START_COORD,
    }
}

fn level_str() -> Vec<&'static str> {
    vec!["&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",
         "&,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,&",
         "&,,############################,,,,,,&",
         "&,,#.........#................#,,&,,,&",
         "&,,#.........#................#,,,&,,&",
         "&,,#..........................#,,&,,,&",
         "&&,#.........#................#,,,,,,&",
         "&,&#.........##########+#######,,,,,,&",
         "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
         "&&,#.........#,t,,,,,,,&,,,,,,,&,&,&,&",
         "&,,#.........#,,,,t&,,,,,,,,&,,,,,,,,&",
         "&,,#.........+,,,,,,&,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
         "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
         "&,&#.........#,,,,,,,=,&,,,,,,,,,,,,,&",
         "&,,###########,t,,,,,&,,,,,,,&,&,,,,,&",
         "&,,&,,,,,,,,,,,,,,,,,&,,,,&,,,,,,,,,,&",
         "&,&,,,,,,,,,,,,&,,,,,,,,,,,,,,,,,,,,,&",
         "&,,,&,,,,,,,,,,,,,,,,&,,,,,#########,&",
         "&,&,,,&,,,,,&,,&,,,,&,,,,,,#.......#,&",
         "&,,,,,&,,,,,,,,,&,,,,&,,,,,#.......#,&",
         "&,,,,,,,,,&,,,,,,,,,,,,,&,,........#,&",
         "&,&,&,,,,&&,,,&,&,,,,,,,&,,#.......#,&",
         "&,,,,,,,,,,,,,,,,,,,&,,,,,,#.......#,&",
         "&,,,&,,,,,,,&,,,,,,,,,,,,,,#########,&",
         "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",]
}