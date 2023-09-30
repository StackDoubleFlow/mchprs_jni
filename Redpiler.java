public class Redpiler {
    private long stateValueHandle = 0;

    private native void init();
    public native void initializeWorld(int xDim, int yDim, int zDim, int[] states, TickEntry[] tileTicks);
    public native void compileWorld(boolean optimize, boolean io_only);
    public native void runTicks(int amount);
    public native void flush(BlockChangeConsumer comsumer);
    public native TickEntry[] reset();

    public interface BlockChangeConsumer {
        void onBlockChange(int xPos, int yPos, int zPos, int newState);
    }

    public static class TickEntry {
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

    public Redpiler() {
        init();
    }
}
