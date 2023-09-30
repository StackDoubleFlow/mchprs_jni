mod change_consumer;

use jni::objects::{JClass, JIntArray, JObject, JObjectArray, ReleaseMode};
use jni::sys::{jboolean, jint, jobject};
use jni::JNIEnv;
use mchprs_blocks::block_entities::BlockEntity;
use mchprs_blocks::blocks::Block;
use mchprs_blocks::BlockPos;
use mchprs_core::redpiler::{Compiler, CompilerOptions};
use mchprs_core::world::storage::Chunk;
use mchprs_core::world::World;
use mchprs_world::{TickEntry, TickPriority};

use std::sync::Mutex;

use crate::change_consumer::ChangeConsumer;

#[derive(Debug)]
struct SmallWorld {
    x_dim: u32,
    y_dim: u32,
    z_dim: u32,
    states: Vec<u32>,
    to_be_ticked: Vec<TickEntry>,
}

impl SmallWorld {
    fn idx_for_pos(&self, pos: BlockPos) -> usize {
        (pos.y as usize * self.x_dim as usize * self.z_dim as usize)
            + (pos.z as usize * self.x_dim as usize)
            + pos.x as usize
    }
}

impl World for SmallWorld {
    fn get_block_raw(&self, pos: BlockPos) -> u32 {
        if pos.x >= 0
            && pos.x < self.x_dim as i32
            && pos.y >= 0
            && pos.y < self.y_dim as i32
            && pos.z >= 0
            && pos.z < self.z_dim as i32
        {
            let idx = self.idx_for_pos(pos);
            self.states[idx]
        } else {
            0
        }
    }

    fn set_block_raw(&mut self, pos: BlockPos, block: u32) -> bool {
        todo!()
    }

    fn delete_block_entity(&mut self, _pos: BlockPos) {
        unimplemented!()
    }

    fn get_block_entity(&self, pos: BlockPos) -> Option<&BlockEntity> {
        todo!()
    }

    fn set_block_entity(&mut self, _pos: BlockPos, _block_entity: BlockEntity) {
        unimplemented!()
    }

    fn get_chunk(&self, x: i32, z: i32) -> Option<&Chunk> {
        unimplemented!()
    }

    fn get_chunk_mut(&mut self, _x: i32, _z: i32) -> Option<&mut Chunk> {
        unimplemented!()
    }

    fn schedule_tick(&mut self, pos: BlockPos, delay: u32, priority: TickPriority) {
        self.to_be_ticked.push(TickEntry {
            pos,
            ticks_left: delay,
            tick_priority: priority,
        });
    }

    fn pending_tick_at(&mut self, _pos: BlockPos) -> bool {
        unimplemented!()
    }
}

struct GlobalState {
    world: Option<SmallWorld>,
    redpiler: Option<Compiler>,
}

static STATE: Mutex<GlobalState> = Mutex::new(GlobalState {
    world: None,
    redpiler: None,
});

#[no_mangle]
pub extern "system" fn Java_Redpiler_initializeWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    x_dim: jint,
    y_dim: jint,
    z_dim: jint,
    raw_states: JIntArray<'local>,
    tile_ticks: JObjectArray<'local>,
) {
    let mut to_be_ticked = Vec::new();
    for idx in 0..env.get_array_length(&tile_ticks).unwrap() {
        let item = env.get_object_array_element(&tile_ticks, idx).unwrap();
        let priority = env.get_field(&item, "priority", "I").unwrap().i().unwrap();
        let ticks_left = env
            .get_field(&item, "ticksRemaining", "I")
            .unwrap()
            .i()
            .unwrap();
        let x = env.get_field(&item, "xPos", "I").unwrap().i().unwrap();
        let y = env.get_field(&item, "yPos", "I").unwrap().i().unwrap();
        let z = env.get_field(&item, "zPos", "I").unwrap().i().unwrap();
        to_be_ticked.push(TickEntry {
            ticks_left: ticks_left as u32,
            tick_priority: match priority {
                0 => TickPriority::Normal,
                1 => TickPriority::High,
                2 => TickPriority::Higher,
                3 => TickPriority::Highest,
                _ => panic!("Invalid tick priority: {}", priority),
            },
            pos: BlockPos::new(x, y, z),
        });
    }
    // TODO: bounds check dims
    let mut buf = vec![0; (x_dim * y_dim * z_dim) as usize];
    env.get_int_array_region(raw_states, 0, buf.as_mut_slice())
        .unwrap();
    let world = SmallWorld {
        x_dim: x_dim as u32,
        y_dim: y_dim as u32,
        z_dim: z_dim as u32,
        states: buf.into_iter().map(|x| x as u32).collect(),
        to_be_ticked,
    };
    STATE.lock().unwrap().world = Some(world);
}

#[no_mangle]
pub extern "system" fn Java_Redpiler_compileWorld<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    optimize: jboolean,
    io_only: jboolean,
) {
    let options = CompilerOptions {
        optimize: optimize != 0,
        io_only: io_only != 0,
        export: false,
    };
    let lock = &mut *STATE.lock().unwrap();
    let world = lock.world.as_mut().expect("world not initialized");
    let redpiler = lock.redpiler.get_or_insert_with(|| Default::default());
    let ticks = std::mem::replace(&mut world.to_be_ticked, Vec::new());
    redpiler.compile(
        world,
        (
            BlockPos::new(0, 0, 0),
            BlockPos::new(
                world.x_dim as i32 - 1,
                world.y_dim as i32 - 1,
                world.z_dim as i32 - 1,
            ),
        ),
        options,
        ticks,
    );
}

#[no_mangle]
pub extern "system" fn Java_Redpiler_runTicks<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    amount: jint,
) {
    let mut lock = STATE.lock().unwrap();
    let redpiler = lock.redpiler.as_mut().expect("redpiler not initialized");

    for _ in 0..amount {
        redpiler.tick()
    }
}

#[no_mangle]
pub extern "system" fn Java_Redpiler_flush<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
    consumer: JObject<'local>,
) {
    let mut lock = STATE.lock().unwrap();
    let redpiler = lock.redpiler.as_mut().expect("redpiler not initialized");
    let mut consumer = ChangeConsumer { consumer, env };
    redpiler.flush(&mut consumer);
}

#[no_mangle]
pub extern "system" fn Java_Redpiler_reset<'local>(mut env: JNIEnv<'local>, class: JClass<'local>) {
    todo!();
}
