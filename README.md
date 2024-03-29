# FM24 Stumbling Through Rust

## Rust notes
### Add a Crate from upstream
```
cargo add <crate_name>
```

### Create a library and add it to your project
Create the library,

```
cargo new --lib <library_name>
```

After that, in the main directory, add the library,

```
cargo add <library_name>
```
> Note: if there are dependencies for the lib, they will not be inherited and need to be specifically added via `cargo add <dep>` in the lib directory.

## FM Notes
### Attributes for players
+ Reg: Registration
+ Inf
+ Name
+ Age
+ Wage
+ Transfer Value
+ Nat
+ 2nd Nat
+ Position
+ Personality
+ Media Handling
+ Av Rat
+ Left Foot
+ Right Foot
+ Height
+ 1v1: One on Ones
+ Acc: Acceleration
+ Aer: Aerial
+ Agg: Aggression
+ Agi: Agility
+ Ant: Anticipation
+ Bal: Balance
+ Bra: Bravery
+ Cmd: Command of Area
+ Cnt: Concentration
+ Cmp: Composure
+ Cro: Crossing
+ Dec: Decisions
+ Det: Determination
+ Dri: Dribbling
+ Fin: Finishing
+ Fir: First Touch
+ Fla: Flair
+ Han: Handling 
+ Hea: Heading
+ Jum: Jumping
+ Kic: Kicking
+ Ldr: Leadership
+ Lon: Long Shots
+ Mar: Marking
+ OtB: Off the Ball
+ Pac: Pace
+ Pas: Passing
+ Pos: Positioning
+ Ref: Reflexes
+ Sta: Stamina
+ Str: Strength
+ Tck: Tackling
+ Tea: Teamwork
+ Tec: Technique
+ Thr: Long Throws
+ TRO: Tendency to Rush Out
+ Vis: Vision
+ Wor: Work Rate
+ UID
+ Cor
+ Club

### Key Attriibutes by Position
https://community.sigames.com/forums/topic/183726-key-attributes-for-players-in-certain-positions/

+ Goalkeepers (GK)
    + Handling, Positioning, Reflexes, One on Ones, Aerial Ability

+ Wing Defenders (DRL)
    + Tackling, Teamwork, Pace, Marking, Decisions

+ Central Defenders (DC)
    + Tackling, Jumping, Strength, Marking, Concentration

+ Central Defensive Midfielders (DMC)
    + Tackling, Passing, Bravery, Determination, Work Rate

+ Central Midfielders (MC)
    + Tackling, Passing, Teamwork, Decisions, Determination

+ Flank Midfielders/Wingers (ML/MR/AML/AMR)
    + Passing, Crossing, Dribbling, Pace, Teamwork

+ Central Attacking Midfielders (AMC)
    + Passing, Creativity, Flair, Technique, First Touch

+ Quick Forwards (FC)
    + Finishing, Composure, Pace, Acceleration, Dribbling

+ Towering Forwards (FC)
    + Finishing, Composure, Strength, Jumping, Anticipation

### Key Metrics per POsition
https://community.sigames.com/forums/topic/577736-key-metrics-kpi-for-each-position/

## Notes

+ Can I put the table processing in a lib?