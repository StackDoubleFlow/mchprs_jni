class Redpiler {
    private static native void initializeWorld(int xDim, int yDim, int zDim, int[] states, TickEntry[] tileTicks);
    private static native void compileWorld(boolean optimize, boolean io_only);
    private static native void runTicks(int amount);
    private static native void flush(BlockChangeConsumer comsumer);
    private static native TickEntry[] reset();

    private interface BlockChangeConsumer {
        void onBlockChange(int xPos, int yPos, int zPos, int newState);
    }

    private static class TickEntry {
        public int priority;
        public int ticksRemaining;
        public int xPos;
        public int yPos;
        public int zPos;

        public TickEntry(int priority, int ticksRemaining, int xPos, int yPos, int zPos) {
            this.priority = priority;
            this.ticksRemaining = ticksRemaining;
            this.xPos = xPos;
            this.yPos = yPos;
            this.zPos = zPos;
        }
    }

    static {
        System.loadLibrary("redpiler_jni");
    }

    public static void main(String[] args) {
        TickEntry[] ticks = new TickEntry[] {
            new TickEntry(0, 1, 1, 0, 1)
        };
        initializeWorld(2, 1, 2, new int[] {
            2114, 16014, // Wire, Target
            2114, 3960, // Wire, Wall Torch (South Lit)
        }, ticks);
        compileWorld(false, false);
        runTicks(2);
        flush(new BlockChangeConsumer() {
            @Override
            public void onBlockChange(int xPos, int yPos, int zPos, int newState) {
                System.out.println("Block at (" + xPos + "," + yPos + "," + zPos + ") changed to state " + newState);
            }
        });
    }
}
