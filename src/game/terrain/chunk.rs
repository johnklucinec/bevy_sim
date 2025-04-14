//Spawning in chunks of terrain to create the enviornment
// around the roads

fn spawn_chunk(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk_coord: IVec2,
    road: &RoadSpline,
    perlin: &Perlin,
    set: &TerrainSettings,
){
    let world = chunk_coord.as_vec2() * set.chunk_size as f32;
    let step = set.chunk_size as f32 / set.verts_per_side as f32;

    let mut verts = Vec::<[f32; 3]>::with_capacity(((set.verts_per_side+1).pow(2)) as usize);
    for z in 0..=set.verts_per_side {
        for x in 0..=set.verts_per_side {
            let px = world.x + x as f32 * step;
            let pz = world.y + z as f32 * step;
            let mut h = perlin.get([px as f64 * set.freq, pz as f64 * set.freq]) as f32 * set.amp;

            // keep the road strip flat
            if road.distance_to(Vec3::new(px, 0.0, pz)) < set.road_clearance {
                h = road.height;          
            }
            verts.push([px, h, pz]);
        }
    }
    
}