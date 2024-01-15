import archetypes from "../store/archetypeStore";

export function getDiva() {
    if (archetypes.diva) {
        return Promise.resolve(archetypes.diva);
    }
    
    // requestf from server
}