####Bubble stats

| Col | Type  | Description                                   |
|-----|-------|-----------------------------------------------|
| 1   | int   | Bubble id                                     |
| 2   | int   | Core number                                   |
| 3   | int   | Number of sub bubbles                         |
| 4   | int   | Minimal length of the bubble                  |
| 5   | int   | Maximum length of the bubble                  |
| 6   | float | Mean length of the bubble                     |
| 7   | int   | Number of traversals                          |
| 8   | int   | Number of intervals                           |
| 9   | int   | Parent names (bubble id) (separated by comma) |
| 10  | int   | Anchor 1                                      |
| 11  | int   | Anchor 2                                      |
| 12  | float | Ratio Min/Max                                 |
| 13  | Bool  | Small                                         |
| 14  | int   | Type                                          | 


### Type in bubble stats

| Number | Description                |
|--------|----------------------------|
| 0      | SNP                        |
| 1      | Indel (small)              |
| 2      | MNP                        |
| 3      | Indel (big)                |
| 4      | Different size (Ratio<0.9) |
| 5      | Same size (Ratio>0.9)      |

### Bed output
| Col | Type   | Description                         |
|-----|--------|-------------------------------------|
| 1   | String | Genome name                         |
| 2   | int    | Start position                      |
| 3   | int    | End position                        |
| 4   | int    | Bubble id                           |
| 5   | int    | Bubble core level                   |
| 6   | int    | Bubble category                     |
| 7   | bool   | Maximum size of the bubble (<50 bp) |
| 8   | int    | Maximum size of the bubble          |
| 9   | int    | Minimum size of the bubble          |