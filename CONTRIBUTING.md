Please follow these guidlines when contributing

1. Braces begin at the end of a line,even with functions and implements

fn foo() -> bar {  //brace starts here
// not here

2. Indention (tabs) should be 8 spaces

fn foo() -> bar {
        if x == bar {

3. No Syntax Voodoo. Code should be correct, then readable, then small.
   If it seems "hacky", dont do it.

4. Dont ignore errors

5. unless its a temp variable (for swapping two others, etc.) and its purpose is clear
 be explicit

6.Document WHAT your code does.  i.e. explain what the function does, its parameters,
and what it returns, even if it seems obvious.  Someone who has never played minecraft
should be able to figure out what almost everything does

/*Spawns a mob,
 * coord is where on the map the mob will spawn
 * Mob ID identifies which mob,
 * if it has a special type, you can set it with subMobID e.g. default white sheep is 0, black is 1 
 */
fn spawnMob(coord: point,mobID: i8,subMobID: i8,items: &[i8])-> {  


