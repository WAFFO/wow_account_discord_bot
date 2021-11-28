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
