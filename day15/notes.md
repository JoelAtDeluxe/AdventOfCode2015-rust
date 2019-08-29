# Cookie generation thoughts

## Part 1

### Condensed ingredients list

| Name      | capacity | durability | flavor | texture | calories |
| --------- | -------- | ---------- | ------ | ------- | -------- |
| Sugar     | 3        | 0          | 0      | -3      | 2        |
| Sprinkles | -3       | 3          | 0      | 0       | 9        |
| Candy     | -1       | 0          | 4      | 0       | 1        |
| Chocolate | 0        | 0          | -2     | 2       | 8        |

### Example combinations

* 100 Sugar = `300 * 0 * 0 * -300 = 0`
* 100 Sprinkles = `-300 * 300 * 0 * 0 = 0`
* 100 Candy = `-100 * 0 * 400 * 0 = 0`
* 100 Chocolate = `0 * 0 * -200 * 200 = 0`

* 1 of each:
  * Capacity:   ` 3 + -3 + -1 +  0 = -1`
  * Durability: ` 0 +  3 +  0 +  0 =  3`
  * Flavor:     ` 0 +  0 +  4 + -2 =  2`
  * Texture:    `-3 +  0 +  0 +  2 = -1`
  * Total (`0`) (net value: `3` when summed)

Thoughts:

* The best score is going to require a non-zero in every category
* ~~Negative scores can be okay, as long as there's an even number of negative values~~
  * Actually, a score for a property covers [0, +infinity), so negative numbers are bad

~~Maybe decent combination:~~

* ~~97 candy + 1 sugar + 1 sprinkles =>~~
  * ~~Capacity:   `(97 * -1) + (1 *  3) + (1 * -3) = - 97`~~
  * ~~Durability: `(97 *  0) + (1 *  0) + (1 *  3) =    3`~~
  * ~~Flavor:     `(97 *  4) + (1 *  0) + (1 *  0) =  388`~~
  * ~~Texture:    `(97 *  0) + (1 * -3) + (1 *  0) = -  3`~~
  * ~~Total Score: `-97 * 3 * -3 * 388 = 338724`~~

* Actually, for this set, there's a pattern. Every ingredient has:
  * 1 positive value
  * 1 negative value
  * 2 zero values
* Since there's only 1 positive value, and we need a positive value in all positions, the goal becomes to balance all values, and when balanced, choosing the most-valuable addition. 

## Best score for smaller number of ingredients

| #   | sugar | sprinkles | candy | choc | score |
| --- | ----- | --------- | ----- | ---- | ----- |
| 1   | 1     | 0         | 0     | 0    | 0     |
| 2   | 1     | 1         | 0     | 0    | 0     |
| 3   | 1     | 1         | 1     | 1    | 0     |