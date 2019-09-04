# Notes

I actually ended up solving this via excel (actually google sheets), via some experimentation and
trying to keep some of the more efficient spells always going.

**[Spreasheet to help in experimenting](https://docs.google.com/spreadsheets/d/13Hn5Susayuildxzi8dlBOkWF3KxC9ZtEZJH0KePs93Q/edit?usp=sharing)**

The rust code is geared towards this kind of experimentation (though, stopped at actually implementing stdin), but also only does part 1 -- I will go back and add in logic for part 2, and may attempt a general solution.

As an alternative, this can also be done without figuring out a specific skill order. The spells do 2 things, generally:

1. Reduces the number of turns the boss stays alive
2. increases the number of turns the player can live

Because of this, it's a small jump to figuring out that you need to keep poison up (how many times will this need to be cast?), in conjunction with how long you need to live (i.e. how often does shield need to be up), coupled with mana requirements (how many times does recharge need to be cast). And in general, trying to make the actions as compressed as possible (i.e. only let a particular status drop only when it's definitely no longer needed -- because you're about to win)

## Theory Crafting

* Do as much damage per turn as possible with mana-efficient skills
* don't die


## Skill Eval

| Skill        | Damage | Mana Cost | Mana per damage |
| ------------ | ------ | --------- | --------------- |
| Magic Missle | 4      | 53        | 13.25           |
| Drain        | 2      | 73        | 36.5            |
| Shield       | 0      | 113       | --              |
| Poison       | 18     | 173       | 9.6             |
| Recharge     | 0      | 229       | --              |

* Keep poison up, except"
  * you can kill with magic missles in 3 turns (12 life remaining)

## Enemy damage track

| turn | Damage dealth |
| ---- | ------------- |
| 1    | 9             |
| 2    | 18            |
| 3    | 27            |
| 4    | 36            |
| 5    | 45            |
| 6    | 54 (Dead)     |


### Possible strat

| Turn | Player Life | Casts      | Mana Available  | Boss Life | Player effects | Boss Effects | Mana Used |
| ---- | ----------- | ---------- | --------------- | --------- | -------------- | ------------ | --------- |
| 0    | 50          | --         | 500             | 58        | --             | --           |           |
| 1a   | 50          | Poison     | 500-173         | 58        | --             | Poisoned(6)  | 173       |
| 1b   | 50 - 9 = 41 | Smack      | 327             | 58-3=55   | --             | Poisoned(5)  |           |
| 2a   | 41          | Recharge   | 327-229         | 55-3=52   | Recharging(5)  | Poisoned(4)  | 402       |
| 2b   | 41 - 9 = 32 | Smack      | 98 + 101        | 52-3=49   | Recharging(4)  | Poisoned(3)  |           |
| 3a   | 32          | Shield     | 199 + 101 - 113 | 49-3=46   | Rec(3), sh(6)  | Poisoned(2)  | 514       |
| 3b   | 32 - 2 = 30 | Smack      | 188 + 101       | 46-3=43   | Rec(2), sh(5)  | Poisoned(1)  |           |
| 4a   | 30          | Poison     | 289 + 101 - 173 | 43-3=40   | Rec(1), sh(4)  | Poisoned(6)  | 687       |
| 4b   | 30 - 2 = 28 | Smack      | 217 + 101       | 40-3=37   | sh(3)          | Poisoned(5)  |           |
| 5a   | 28          | Recharge   | 318 - 229       | 37-3=34   | Rec(5), sh(2)  | Poisoned(4)  | 916       |
| 5b   | 28 - 2 = 26 | Smack      | 89 + 101        | 34-3=31   | Rec(4), sh(1)  | Poisoned(3)  |           |
| 6a   | 26          | shield     | 190 + 101 - 113 | 31-3=28   | Rec(3), sh(6)  | Poisoned(2)  | 1028      |
| 6b   | 26 - 2 = 24 | Smack      | 179 + 101       | 28-3=25   | Rec(2), sh(5)  | Poisoned(1)  |           |
| 7a   | 22          | Poison     | 280 + 101 - 173 | 25-3=22   | Rec(1), sh(4)  | Poisoned(6)  | 1201      |
| 7b   | 22 - 2 = 20 | Smack      | 208 + 101       | 22-3=19   | sh(3)          | Poisoned(5)  |           |
| 8a   | 20          | M. Missile | 309 - 53        | 19-3-4=12 | sh(2)          | Poisoned(4)  | 1254      |
| 8b   | 20 - 2 = 18 | Smack      | 256             | 12-3=9    | sh(1)          | Poisoned(3)  |           |
| 9a   | 18          | M. Missile | 256 - 53        | 9-3-4=2   |                | Poisoned(2)  | 1308      |
| 9b   | 18          | (dead)     | 203             | 2-3 = -1  |                | Poisoned(1)  |           |

mp used = 173 + 229 + 113 + 173 + 229 + 113 + 173 + 53 + 53 => 1309 (too high!)

# Alt

| Turn | Player Life | Casts      | Mana Available  | Boss Life | Player effects | Boss Effects | Mana Used |
| ---- | ----------- | ---------- | --------------- | --------- | -------------- | ------------ | --------- |
| 0    | 50          | --         | 500             | 58        | --             | --           |           |
| 1a   | 50          | Poison     | 500-173         | 58        | --             | Poisoned(6)  | 173       |
| 1b   | 50 - 9 = 41 | Smack      | 327             | 58-3=55   | --             | Poisoned(5)  |           |
| 2a   | 41          | Recharge   | 327-229         | 55-3=52   | Recharging(5)  | Poisoned(4)  | 402       |
| 2b   | 41 - 9 = 32 | Smack      | 98 + 101        | 52-3=49   | Recharging(4)  | Poisoned(3)  |           |
| 3a   | 32          | Shield     | 199 + 101 - 113 | 49-3=46   | Rec(3), sh(6)  | Poisoned(2)  | 514       |
| 3b   | 32 - 2 = 30 | Smack      | 188 + 101       | 46-3=43   | Rec(2), sh(5)  | Poisoned(1)  |           |
| 4a   | 30          | Poison     | 289 + 101 - 173 | 43-3=40   | Rec(1), sh(4)  | Poisoned(6)  | 687       |
| 4b   | 30 - 2 = 28 | Smack      | 217 + 101       | 40-3=37   | sh(3)          | Poisoned(5)  |           |
| 5a   | 28          | Recharge   | 318 - 229       | 37-3=34   | Rec(5), sh(2)  | Poisoned(4)  | 916       |
| 5b   | 28 - 2 = 26 | Smack      | 89 + 101        | 34-3=31   | Rec(4), sh(1)  | Poisoned(3)  |           |
| 6a   | 26          | shield     | 190 + 101 - 113 | 31-3=28   | Rec(3), sh(6)  | Poisoned(2)  | 1028      |
| 6b   | 26 - 2 = 24 | Smack      | 179 + 101       | 28-3=25   | Rec(2), sh(5)  | Poisoned(1)  |           |
| 7a   | 22          | M. Missile | 280 + 101 - 53  | 25-3-4=18 | Rec(1), sh(4)  | --           | 1092      |
| 7b   | 22 - 2 = 20 | Smack      | 328 + 101       | 18        | sh(3)          |              |           |
| 8a   | 20          | M. Missile | 429 - 53        | 18-4=14   | sh(1)          |              | 1145      |
| 8b   | 20 - 2 = 18 | Smack      | 376             | 14        |                |              |           |
| 9a   | 18          | M. Missile | 376 - 53        | 14-4=10   |                |              | 1198      |
| 9b   | 18 - 9 = 9  | Smack      | ???             | 10        |                |              |       |
(dies, eventually)