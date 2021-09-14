# Dragonear

> dra·go·ne·ar. intransitive. America impersonar to pass oneself off as, impersonate. manipular to manipulate.

This the game of Dragon controllers who want to summon the unlimited power of the magical creatures and win in the epic battles.

## Logic

`Character` - the character user is playing with.
`Dragon` - NFTs that users can own. 
  - One can own unlimited number of dragons but only can play one a day.
  - Dragons have preferences: fire, water, earth and air. Depending on the preference they get different powers as they evolve.
  - Generation defines how many slots they get for advanced
  - To mate 2 dragons, you must be in the same "cluster", agree to mate on both sides and one of you will randomly get the offspring. Mating allowed only after dragon receives lvl 10.
  - To receive xp, you need to battle other dragons. You commit to battle, get matched. You see opponent and submit action sequence hash. They submit their sequence. After that both of you reveal and battle unfolds. Whoever wins, receives xp from the battle. 
  - Dragon needs to rest after a battle depending how much hp it lost.
  - As xp reaches next level, you can train your dragon and pick one of the skills.

`Skill` - there are set of skills that you can leverage in the battle. Each skill has preference to the power. Skill either defensive or attack. Skill has recharge time (number of steps between using them).

Skills going to be hard to balance initially, so DAO controlling the game can vote to add and disable skills. If skill is already used by some dragons, it going get disabled in some period of time - user can choose same level skill instead during this period of time.

`Cluster` - location where you are on the map. Each cluster defines preference to one of 4 powers. Cluster has upper lvl limit for dragons. If you dragon overgrew it's level, you need to leave the cluster. Each cluster maintains top + there is a global top players by wins/lvls.

- `create_character()` // anyone can call, creates the character in the game.
- `create_dragon(account_id) -> u64` // owner method, allows to create new dragon for given account. Returns `token_id` of Dragon.
- `select_dragon(dragon_id: u64)` // select given dragon as main one. you can only do it once a day.
- `move_cluster(cluster_id: u64)` // fly character from one cluster to another one. different clusters have different properties and will in result different dragons will have behave differently. Plus clusters have limitations for levels to reduce PvP with overpowered. There are feww
- `start_battle()`
- `commit_battle(hash)`
- `reveal_battle(action_sequence)`
- `evolve(skill_ids)`
- `add_skill()`
- `disable_skill()`
- `replace_skill(skill_id, new_skill_id)` -- replaces existing disabled skill with new skill
