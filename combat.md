# Introduction to my `Combat system`  

## Goal

In the game i want to have abilities:  
- Attack enemies,
- Mine tiles,

To implement these, i am thinking about making  
unified system which will handle all related actions.

So the idea is to split all combat into five steps:  
1. Kick initialization
2. Kick animation
2. Hit detection
3. Damage calculation
4. Damage execution

## Implementation

**Kick initialization** is going to be system which  
reacts on mouse clicks and starts kick animation.  
For enemys we'll have different system to handle initialization.  
Kick animation is going to be a component:

```Rust
#[derive(Component)]
struct KickAnimation {
    progress: f32,      
    duration: f32,      
    max_angle: f32,     
    active: bool,
    impact_triggered: bool,
    target: Option<Vec2>,
    item: Option<Entity>,
}
```
So when clicked we set `active` field to true and we set  
duration and max_angle depended on the attack_speed of   
item-entity. Also we store click position in `target` field  
to find the tile we clicked if it was tile. I am using `Option`  
to handle case when haven't clicked yet.  
Also we set `item` field to store the `Entity` player  
was holding in hands when clicked. 

**Kick animation** is going to rotate the entity  
with `KickAnimation` component with `active = true` field,  
depended on `max_angle` and `duration`. Each turn it   
changes the `progress` field and when it hits 0,5 -  
sends `the message` to hit detection system.  
Also it sets the `impact_triggered` field to  
avoid multiple impacts at once, because it sends  
only if it's false.

**Hit detection** reads the `ImpactTrigger` message  
to see who hit and what it hit. It decides did it hit  
tile or enemy or nothing. For each case it calculates  
damage except the last one and sends message to the last system.  
To calculate damage i think to use other function calles  
to add some flexibility to the code. Because if i place  
it in the `hit detection` function - it's going to be really  
messy there.

**Damage execution** is going to read messages from   
previous system and apply calculated damage to entity  
which was hit.

### Item 

As i said above - i'll have `item` component.  
I think, it will be really nice to store all the  
characteristics of the item in there.  

Characteristics i might have:

- `attack_speed`
- `swing_angle` - to calculate `max_angle`.
- `enemy_damage`
- `structure_damage`
- `durability`
- `usable`

Also in component we'll store image, name and all we'll  
need to indentify the item.


## Potential issues and benefits

### benefits

This aproach gives us a lot of flexibility cause all systems  
are separated and don't depend on each other so it's going  
to be really easy to upgrade it. Also i just like to separate  
everything. In programming it looks good.

### issues

Can't see for now.









