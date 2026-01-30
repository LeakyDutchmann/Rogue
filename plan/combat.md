# Introduction to my ***A.H.D*** `Combat system`  

## Goal

In the game i want to have abilities:  
- Attack enemies,
- Mine tiles,

To implement these, i am thinking about making  
unified system which will handle all related actions.

So the idea is to split all combat into five steps:  
1. Attack initialization
2. Attack progression && Attack animation
2. Hit detection
3. Damage calculation
4. Damage execution

## Implementation

**Attack initialization** is going to be system which  
reacts on mouse clicks and starts attack animation.  
For enemys we'll have different system to handle initialization.  
Attack animation is going to be a component:

```Rust
#[derive(Component)]
struct AttackAnimation {
    progress: f32,      
    duration: f32,      
    max_angle: f32,
    impact_triggered: bool,
    target: Option<Vec2>,
    item: Option<Entity>,
}
```
So when clicked we add `AttackAnimation`  to `Entity` and we set  
duration and max_angle depended on the attack_speed of   
item-entity. Also we store click position in `target` field  
to find the tile we clicked if it was tile. I am using `Option`  
to handle case when haven't clicked yet.  
Also we set `item` field to store the `Entity` player  
was holding in hands when clicked. 

**Attack progression** is going to update  
`progress` field based on `duration` and `max_angle`
when `progress` >= 0.5  
it sends `HitMessage` and set's impact_triggered to true. 
other fuction called `attack_animation` is reading  
`progress` field and rotates the entity. When progress  
== 1 it removes `AttackAnimation` from entity.


**Hit detection** reads the `HitMessage` message  
to see who hit and what it hit. It decides did it hit  
tile or enemy or nothing. For each case it calculates  
damage except the last one and sends message to the last system.  
To calculate damage i think to use other function  
called `damage_calculation`. Doing that will save  
a lot of space in `hit_detection` fn.

**Damage execution** is going to read messages from   
previous system and apply calculated damage to entity  
which was hit.

### Item 

As i said above - i'll have `Item` component.  
And also `CombatStats`,`WeaponStats` and `ToolStats`.  
`Item` stores `image`, `name`. Also `Usable` component.  
`CombatStats` stores:
- `attack_speed`
- `swing_angle` 
- `durability`
- `radius`

`WeaponStats` stores:
- `enemy_damage`


`ToolStats` stores:
- `structure_damage`

So if some entity hasn't `CombatStates` component  
We don't initialize attack and save our CPU from exploding.


## Potential issues and benefits

### benefits

This aproach gives us a lot of flexibility cause all systems  
are separated and don't depend on each other so it's going  
to be really easy to upgrade it. Also i just like to separate  
everything. In programming it looks good.

### issues

It might have problem with optimization when a lot of  
entities are trying to hit something, but i think,  
chunk-based rendering will solve that.










