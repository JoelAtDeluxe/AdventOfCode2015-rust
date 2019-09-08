# Modules

| Module | Replacements                                                                       |
| ------ | ---------------------------------------------------------------------------------- |
| Al     | ThF, ThRnFAr                                                                       |
| B      | BCa, TiB, TiRnFAr                                                                  |
| Ca     | CaCa, PB, PRnFAr, SiRnFYFAr, SiRnMgAr, SiTh                                        |
| F      | CaF, PMg, SiAl                                                                     |
| H      | CRnAlAr, CRnFYFYFAr, CRnFYMgAr, CRnMgYFAr, HCa, NRnFYFAr, NRnMgAr, NTh, OB, ORnFAr |
| Mg     | Mg, TiMg                                                                           |
| N      | CRnFAr, HSi                                                                        |
| O      | CRnFYFAr, CRnMgAr, HP, NRnFAr, OTi                                                 |
| P      | CaP, PTi, SiRnFAr                                                                  |
| Si     | CaSi                                                                               |
| Th     | ThCa                                                                               |
| Ti     | BP, TiTi                                                                           |
| e      | HF, NAl, OMg                                                                       |
| Rn     | --                                                                                 |
| Ar     | --                                                                                 |
| Y      | --                                                                                 |

each replacement takes one of the following forms:

1. M -> N.N (i.e. one element to two)
2. M -> N.Rn.N.Ar (i.e. one element to two, with encapsulation)
3. M -> N.Rn.N.Y.(N.Y.)N.Ar (i.e. Element, encapsulation with 1-2 elements)

If we asbstract this to consider total molecule length:
Rule 1 increases the length of the total molecule by 1
Rule 2 increases the length of the total molecule by 3
Rule 3 increases the length of the total molecule by 5-7
