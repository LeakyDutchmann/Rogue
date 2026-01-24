# Introduction to my combat system
 
## Idea

In a game i want to have two systems `combat` and  
`destruction` to be connected to each other.  
I want to have something i could implement as  
unified system that could decide what are you destroying.

## Implementation 

Everything starts with a hit. In code i call it  
`KickAnimation` and it's a struct whihc can be used as   
`Component` to the entity. 
```Rust
#[derive(Component)]
struct KickAnimation {
    progress: f32,      // 0..1
    duration: f32,      // seconds
    max_angle: f32,     // radians
    active: bool,
    impact_triggered: bool,
    target: Option<Vec2>,
    item: Option<Entity>,
}
```

Those field are used to track stages of animation.  
When some `Entity` wants to hit something, we modify the  
fields of `KickAnimation` component. And based on it  
decide what will happen later.

The kick is independent of what you actually want to do.  
It checks what you can do.

So, we need to separate everything into four systems:  

1. `Start kick system`
2. `Animate kick system`
3. `Detect impact system`
4. `Apply damage system`

The `Start kick system` is responsible for initializing   
the kick animation, and setting the value of `duration`,   
`max_angle`, `item` fields based on the item we use.   
Also, it sets the `active` field to `true` to start the animation.


The `Animate kick system` is responsible for updating the  
progress of the animation and the angle of `Entity` with  
the `KickAnimation` component. It also checks if the kick  
has reached its maximum angle and sends a `ImpactTrigger`   
message.


The `Detect impact system` is responsible for reading the  
`ImpactTrigger` message and deside:   

      - Did we hit something?
            - Was that tile or enemy?
      - How much damage should we do? 

And just then `Apply damage system` is going to apply the   
damage it's told from `Detect impact system`.


## Conclusion

In my opinion this system is good because it separates the  
resoponsibilities and is really flexible.   
Also i think it's very unified, because i only need  
to make different system for starting `KickAnimation`.

