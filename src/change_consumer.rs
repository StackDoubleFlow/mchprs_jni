use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use mchprs_core::world::World;

pub struct ChangeConsumer<'local> {
    pub env: JNIEnv<'local>,
    pub consumer: JObject<'local>,
}

impl<'local> World for ChangeConsumer<'local> {
    fn get_block_raw(&self, _pos: mchprs_blocks::BlockPos) -> u32 {
        unimplemented!()
    }

    fn set_block_raw(&mut self, pos: mchprs_blocks::BlockPos, block: u32) -> bool {
        self.env
            .call_method(
                &self.consumer,
                "onBlockChange",
                "(IIII)V",
                &[
                    JValue::Int(pos.x),
                    JValue::Int(pos.y),
                    JValue::Int(pos.z),
                    JValue::Int(block as i32),
                ],
            )
            .unwrap();
        true
    }

    fn delete_block_entity(&mut self, _pos: mchprs_blocks::BlockPos) {
        unimplemented!()
    }

    fn get_block_entity(
        &self,
        _pos: mchprs_blocks::BlockPos,
    ) -> Option<&mchprs_blocks::block_entities::BlockEntity> {
        unimplemented!()
    }

    fn set_block_entity(
        &mut self,
        _pos: mchprs_blocks::BlockPos,
        _block_entity: mchprs_blocks::block_entities::BlockEntity,
    ) {
        unimplemented!()
    }

    fn get_chunk(&self, _x: i32, _z: i32) -> Option<&mchprs_core::world::storage::Chunk> {
        unimplemented!()
    }

    fn get_chunk_mut(
        &mut self,
        _x: i32,
        _z: i32,
    ) -> Option<&mut mchprs_core::world::storage::Chunk> {
        unimplemented!()
    }

    fn schedule_tick(
        &mut self,
        _pos: mchprs_blocks::BlockPos,
        _delay: u32,
        _priority: mchprs_world::TickPriority,
    ) {
        unimplemented!()
    }

    fn pending_tick_at(&mut self, _pos: mchprs_blocks::BlockPos) -> bool {
        unimplemented!()
    }
}
