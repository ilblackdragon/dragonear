use near_sdk::{env, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Element {
    Physical,
    Fire,
    Water,
    Air,
    Earth,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Ratio {
    num: u32,
    denom: u32,
}

impl Ratio {
    pub fn new(num: u32, denom: u32) -> Self {
        Self { num, denom }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum SkillKind {
    Attack,
    Buff,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Skill {
    pub element: Element,
    pub kind: SkillKind,
    pub powerup_ratio: Ratio,
    pub cooldown: u32,
}

fn random_number() -> u8 {
    env::random_seed()[0]
}

fn random_powerup() -> Ratio {
    match random_number() {
        0..=99 => Ratio::new(1, 1),
        100..=174 => Ratio::new(5, 4),
        175..=224 => Ratio::new(3, 2),
        225..=250 => Ratio::new(2, 1),
        _ => Ratio::new(5, 1),
    }
}

fn random_cooldown() -> u32 {
    match random_number() {
        0..=99 => 3,
        100..=200 => 2,
        _ => 1,
    }
}

impl Element {
    pub fn random() -> Self {
        Element::Physical
    }
}

impl Skill {
    pub fn physical() -> Self {
        Self {
            element: Element::Physical,
            kind: SkillKind::Attack,
            powerup_ratio: Ratio::new(1, 1),
            cooldown: 0,
        }
    }

    pub fn random() -> Self {
        Self {
            element: Element::random(),
            kind: if random_number() % 2 == 0 { SkillKind::Attack } else { SkillKind::Buff },
            powerup_ratio: random_powerup(),
            cooldown: random_cooldown(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Constitution {
    pub max_hp: u32,
    pub attack: [u32; 5],
    pub defense: [u32; 5],
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Dragon {
    pub owner_id: AccountId,
    pub generation: u8,
    pub level: u8,
    pub exp: u32,
    pub element: Element,
    pub skills: Vec<Skill>,
    pub constitution: Constitution,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VDragon {
    V1(Dragon)
}

impl From<VDragon> for Dragon {
    fn from(c: VDragon) -> Self {
        match c {
            VDragon::V1(c) => c
        }
    }
}

impl Dragon {
    /// Create a random dragon of 0th generation.
    pub fn random(account_id: &AccountId) -> Self {
        Self {
            owner_id: account_id.clone(),
            generation: 0,
            level: 0,
            exp: 0,
            element: Element::random(),
            skills: vec![Skill::physical()],
            constitution: Constitution {
                max_hp: (random_number() % 10) as u32 + 10,
                attack: [1, 1, 1, 1, 1],
                defense: [1, 1, 1, 1, 1],
            }
        }
    }

    /// Mate two dragons and create a new dragon of next generation and of the given properties.
    // pub fn mate(dragon_a: &Dragon, dragon_b: &Dragon) -> Self {
    //     Self::random(dragon_a)
    // }

    pub fn uplevel(&mut self) {
        assert!(self.exp >= (self.level as u32) * 1000, "NOT_ENOUGH_EXP");
        self.exp -= (self.level as u32) * 1000;
        self.level += 1;
        self.skills.push(Skill::random());
    }

    pub fn apply_skill(&self, constitution: &mut Constitution, other_constitution: &mut Constitution, skill_id: u8) {
        let skill = &self.skills[skill_id as usize];
        let element_id = skill.element.clone() as usize;
        match skill.kind {
            SkillKind::Attack => {
                let mut attack = constitution.attack[element_id] * skill.powerup_ratio.num / skill.powerup_ratio.denom;
                let defense = other_constitution.defense[element_id];
                attack -= if attack < defense { 1 } else { attack - defense };
                other_constitution.max_hp -= std::cmp::min(attack, other_constitution.max_hp);
            },
            SkillKind::Buff => {
                constitution.defense[element_id] = constitution.defense[element_id] * skill.powerup_ratio.num / skill.powerup_ratio.denom;
            }
        }
    }
}