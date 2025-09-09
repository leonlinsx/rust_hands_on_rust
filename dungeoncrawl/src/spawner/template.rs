use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::{fs::File, path::PathBuf};

// every type in struct needs to support Deserialize
#[derive(Deserialize, Clone, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

// top level collection representing file, vector of templates
#[derive(Deserialize, Clone, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

fn open_resource(rel: &str) -> std::io::Result<File> {
    // 1) Try current working dir (works if main already set CWD to the exe dir)
    if let Ok(f) = File::open(rel) {
        return Ok(f);
    }

    // 2) Try alongside the executable (works even if CWD wasn’t fixed)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join(rel);
            if let Ok(f) = File::open(&p) {
                return Ok(f);
            }
        }
    }

    // 3) Try project root (nice for `cargo test` / dev runs)
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    if let Ok(f) = File::open(&p) {
        return Ok(f);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Couldn't find resource: {}", rel),
    ))
}

impl Templates {
    pub fn load() -> Self {
        let file =
            open_resource("resources/template.ron").expect("Failed to open resources/template.ron");
        from_reader(file).expect("Failed to parse template.ron")
    }
    // pub fn load() -> Self { let file = File::open("resources/template.ron").expect("Failed to open file"); from_reader(file).expect("Failed to parse file") }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            // only consider entities that can appear on this level
            .filter(|e| e.levels.contains(&level))
            // add entities to available list according to frequency
            // e.g. frequency of 3 means 3 entries in list to increase chance of selection
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        // push spawn commands to command buffer, then flush at end to avoid borrow conflicts
        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            // shuffle available entities to ensure randomness
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, &mut commands);
            }
        });
        commands.flush(ecs);
    }

    fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer,
    ) {
        let entity = commands.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()), // if not cloned, rust will try to move out of template and fail to compile
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer {});
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }

        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(provides, n)| {
                // effects stored using list to allow multiple effects
                match provides.as_str() {
                    "Healing" => {
                        commands.add_component(entity, ProvidesHealing { amount: *n });
                    }
                    "MagicMap" => {
                        commands.add_component(entity, ProvidesDungeonMap {});
                    }
                    _ => {
                        println!("Unknown effect type: {}", provides);
                    }
                }
            });
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }
    }
}
