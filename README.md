# Tunul - Minecraft server

One of the github projects that will never be finished. Most likely.
The off chance you would want to help, submit a private Issue with your email

##Goals/Features

#####  (These have to be designed from start, so useless to put in bug tracker) 

---------
##### Written in Rust

##### Won't be Backwards compatible - wont Use NDT format

##### Multithreaded by design

##### Lazy - will call threads instead of looping based on needs. 
  *Only constant active process would be Keep-alive ping, and time based things e.g. hunger,growth. All others will be  
  triggered by player movement,client action, or proximity
 
##### Better map generation and saves
 
  * Ore viens will be have a radom but equivlent chance of being generated as compared to vanilla(by default),
  but instead being generated when the chunck is generated, they will be generated when a player is within a few blocks of
  them.  This allows for more effiecient packet transfer, less data to store, and will provide costless xray 
  * By default minecraft is generated, saved as literal block values in a 1/16 of a chuck increments with meta-data, then
  compressed. When read, it is uncompress block by block, and reads block by block to packet.
  * Tunnel will "Graph" the maps when generating and saving i.e. Everything y < 64 is stone; 64< dirt < 76 < air
  Structures such as mountins will be graphed with a mathematic function. Ideally Tunnel will graph straigt into
  a compressed packet with no conversion to drasrically increase load performance . Small Abnormalies will be
  saved in traditional way and eddited afterword.
  * Non cave/mine Ore viens will be have the same generation in a practical sense to vanilla(by default), but 
  rather than being generated when the chunck is generated, veins will have a chance to spawn when a player nears
  a viens requirements,   This is calculated for a block when a a block within X meters is mined by a player
  * Terrain generation will be configurable, E.g. extreme, or mild
  * Since "abnormalities" are independent of chunks, non reasource intensive things such as plant growth will still occur
  when no chunk are'nt loaded In a sepperate thread
  * Anti X-ray will be built in, but as a side effect of lazy loading, and  computed ahead of time to INCREASE performance.  When not under load, TUNUL will calculate what is viewable from where. If Object A is not viewable from area B, and a player is in area B, Object A will not be edited into to the graphed map before being sent to the player, increasing performance under load. When the player is within a buffer of a few seconds of being able to see object A, the map will update
