
# Organism definition

Levelized structure:
1. Every organism is made of a circumpherence
2. The circumpherence is made of section which corresponds to an organ
  - Circumpherence:
    - Section
3. An organ is made up of segments, which are just lines described of 2 points: start and end
  - A s 


- Circumpherence
  - Let N number of organs
  - Let S the space in degrees on the circumpherence
  - The sum of space of all organs S (each organ) must be 360 degrees  
  - There must be at least one of each organs: Mouth, Sensor, Flagellum, Armor

- Organ definition:
  - *START_SEGMENT* (start)
    - This point is on the circumpherence
  - segment1
    - start point is end point of start_segment
  - segment2
  - ...
  - segmentN
    - ends on start of end segment
  - *END_SEGMENT* (end)
    - its end point is on the cicumpherence
  - SEGMENTS RULES:
    - let i the i-nth segment
    - end point of i is also start point of i+1
    - exception made for end_segment which end on a point of circumpherence
    - each couple of segments has an internal angle A in between
      - we can define:
        - a fixed angle
        - a range of oscillation
        - a spring like behavior
    - Mass property
    - Thickness
    - Length
