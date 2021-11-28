pub fn race_int_to_str(race_id: u8) -> &'static str {
  match race_id {
    1 => "Human",
    2 => "Orc",
    3 => "Dwarf",
    4 => "Night Elf",
    5 => "Undead",
    6 => "Tauren",
    7 => "Gnome",
    8 => "Troll",
    9 => "Goblin",
    10 => "Blood Elf",
    11 => "Draenei",
    _ => "bad id: {}",
  }
}

pub fn class_int_to_str(class_id: u8) -> &'static str {
  match class_id {
    1 => "Warrior",
    2 => "Paladin",
    3 => "Hunter",
    4 => "Rogue",
    5 => "Priest",
    6 => "Death Knight",
    7 => "Shaman",
    8 => "Mage",
    9 => "Warlock",
    10 => "Monk",
    11 => "Druid",
    12 => "Demon Hunter",
    _ => "bad id: {}",
  }
}
